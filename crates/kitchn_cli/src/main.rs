use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use k_lib::config::Cookbook;
use k_lib::db::Pantry;
use k_lib::logger;
use k_lib::{ingredient::Ingredient, packager, processor};
use tracing::{debug, warn};
use tracing_subscriber::{EnvFilter, Layer, prelude::*};

use colored::Colorize;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;

// mod watcher; // Removed as we use sender-side coloring

/// Helper to log using k_lib::logger directly
fn log(config: &Cookbook, preset_key: &str) {
    if let Some(preset) = config.dictionary.presets.get(preset_key)
        && let Some(scope) = &preset.scope
    {
        // 1. User Facing Log
        logger::log_to_terminal(config, &preset.level, scope, &preset.msg);

        // 2. Debug Watcher is now handled via tracing-log capturing the logger::log_to_terminal internal log calls if they emit events,
        // OR capturing log::* calls from k_lib.

        // RE-ENABLED MANUAL TRACE to ensure high level semantic logs are visible
        tracing::info!(scope = scope, message = &preset.msg);

        if config.layout.logging.write_by_default {
            let _ = logger::log_to_file(config, &preset.level, scope, &preset.msg, None);
        }
    }
}

/// Helper to log with custom msg
fn log_msg(config: &Cookbook, preset_key: &str, msg_override: &str) {
    if let Some(preset) = config.dictionary.presets.get(preset_key)
        && let Some(scope) = &preset.scope
    {
        // 1. User Facing Log
        logger::log_to_terminal(config, &preset.level, scope, msg_override);

        // 2. Debug Watcher Log (Broadcast)
        // RE-ENABLED MANUAL TRACE to ensure high level semantic logs are visible
        tracing::info!(scope = scope, message = msg_override);

        if config.layout.logging.write_by_default {
            let _ = logger::log_to_file(config, &preset.level, scope, msg_override, None);
        }
    }
}

#[derive(Parser)]
#[command(name = "kitchn", version, about = "Kitchn Ingredient Manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable debug mode with verbose logging in a separate terminal
    #[arg(long, global = true)]
    debug: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Stock .ing ingredients or .bag packages into the pantry
    Stock { path: PathBuf },
    /// Wrap .ing ingredients from a directory into a .bag package
    Wrap {
        /// Directory containing .ing files
        input: PathBuf,
        /// Output .bag file (optional, defaults to <dirname>.bag)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Cook all ingredients from pantry into the system
    Cook,
    /// List stocked ingredients
    Pantry {
        #[command(subcommand)]
        command: Option<PantryCommands>,
    },
    /// Bake cookbook into binary pastry for faster startup
    Bake,
    /// Internal command to watch logs via socket (Hidden)
    #[command(hide = true)]
    InternalWatch { socket_path: PathBuf },
}

#[derive(Subcommand, Debug)]
enum PantryCommands {
    /// Remove all ingredients from the pantry
    Clean,
}

// --- Socket Logging Implementation ---

fn get_socket_path() -> PathBuf {
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::temp_dir());
    runtime_dir.join("kitchn-debug.sock")
}

struct SocketSubscriberLayer {
    socket: Mutex<Option<UnixStream>>,
}

impl SocketSubscriberLayer {
    fn new(socket_path: &Path) -> Self {
        // Try to connect, if fail, we just won't log to socket
        let socket = UnixStream::connect(socket_path).ok();
        Self {
            socket: Mutex::new(socket),
        }
    }
}

impl<S> Layer<S> for SocketSubscriberLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Only lock if we have a socket.
        // But we need to lock to check if we have one.
        if let Ok(mut guard) = self.socket.lock()
            && let Some(stream) = guard.as_mut()
        {
            // 1. Get Metadata (Level)
            let metadata = event.metadata();
            let level_color = match *metadata.level() {
                tracing::Level::ERROR => "ERROR".red(),
                tracing::Level::WARN => "WARN".yellow(),
                tracing::Level::INFO => "INFO".green(),
                tracing::Level::DEBUG => "DEBUG".blue(),
                tracing::Level::TRACE => "TRACE".magenta(),
            };

            // 2. Timestamp
            let timestamp = chrono::Local::now().format("%H:%M:%S").to_string().dimmed();

            // 3. Visitor with Scope support
            struct MessageVisitor {
                message: String,
                scope: Option<String>,
            }

            impl tracing::field::Visit for MessageVisitor {
                fn record_debug(
                    &mut self,
                    field: &tracing::field::Field,
                    value: &dyn std::fmt::Debug,
                ) {
                    if field.name() == "message" {
                        self.message.push_str(&format!("{:?}", value));
                    }
                }
                fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                    if field.name() == "message" {
                        self.message.push_str(value);
                    } else if field.name() == "scope" {
                        self.scope = Some(value.to_string());
                    }
                }
            }

            let mut visitor = MessageVisitor {
                message: String::new(),
                scope: None,
            };
            event.record(&mut visitor);

            if visitor.message.is_empty() {
                return;
            }

            // 4. Format: TIME [LEVEL] [SCOPE] Message
            let scope_part = if let Some(s) = visitor.scope {
                format!("[{}] ", s)
            } else {
                String::new()
            };

            // Strip tags for clean debug output
            let clean_message = strip_tags(&visitor.message);

            let final_msg = format!(
                "{} [{}] {}{}\n",
                timestamp, level_color, scope_part, clean_message
            );

            if stream.write_all(final_msg.as_bytes()).is_err() {
                *guard = None;
            }
        }
    }
}

// Helper to strip XML-like tags from log messages
fn strip_tags(msg: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < msg.len() {
        if let Some(start) = msg[i..].find('<') {
            result.push_str(&msg[i..i + start]);
            i += start;
            if let Some(end) = msg[i..].find('>') {
                i += end + 1;
            } else {
                result.push('<');
                i += 1;
            }
        } else {
            result.push_str(&msg[i..]);
            break;
        }
    }
    result
}

fn init_logging(force_debug: bool) -> Result<bool> {
    // Basic EnvFilter
    // Init LogTracer to bridge log crate events to tracing
    // Ignore error if already initialized (for tests/multiple calls safety)
    let _ = tracing_log::LogTracer::init();

    let socket_path = get_socket_path();
    let watcher_active = socket_path.exists();

    // Enable debug if flag is passed OR if the debug watcher is active/socket exists
    let enable_debug = force_debug || watcher_active;

    let env_filter = if enable_debug {
        // Force debug level, ignoring environment variables
        EnvFilter::new("debug")
    } else {
        // Use environment variable RUST_LOG, defaulting to info
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    let registry = tracing_subscriber::registry().with(env_filter);

    // Always attempt to connect to socket if it exists/is valid.
    let layer = SocketSubscriberLayer::new(&socket_path);
    // We use set_global_default instead of init() to avoid panics if called multiple times,
    // and because we manually initialized LogTracer above.
    // init() would try to init LogTracer again if the feature was enabled, causing a panic/error.
    let _ = tracing::subscriber::set_global_default(registry.with(layer));

    Ok(true)
}

fn spawn_debug_viewer() -> Result<()> {
    let socket_path = get_socket_path();

    // Check if socket connectable
    if UnixStream::connect(&socket_path).is_ok() {
        return Ok(());
    }

    // Remove stale socket file
    if socket_path.exists() {
        let _ = fs::remove_file(&socket_path);
    }

    // Detect terminal
    let terminal = env::var("TERMINAL").ok().or_else(|| {
        let terminals = ["rio", "alacritty", "kitty", "gnome-terminal", "xterm"];
        for term in terminals {
            if which::which(term).is_ok() {
                return Some(term.to_string());
            }
        }
        None
    });

    if let Some(term) = terminal {
        debug!("Spawning debug viewer with: {}", term);
        let self_exe = env::current_exe().context("Failed to get current executable path")?;

        // Spawn terminal running internal-watch
        let _ = Command::new(&term)
            .arg("-e")
            .arg(&self_exe)
            .arg("internal-watch")
            .arg(&socket_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn debug terminal")?;

        println!("Debug Mode Started.");
        println!("Socket: {:?}", socket_path);

        // Wait for socket to appear (max 2s)
        let start = std::time::Instant::now();
        while !socket_path.exists() && start.elapsed() < Duration::from_secs(2) {
            std::thread::sleep(Duration::from_millis(50));
        }
        std::thread::sleep(Duration::from_millis(100)); // Grace period for bind
    } else {
        warn!("No supported terminal emulator found.");
        println!("Cannot spawn debug terminal.");
    }

    Ok(())
}

fn run_socket_watcher(socket_path: &Path) -> Result<()> {
    // Note: colors are provided by the sender now (like kitchnsink)
    // receiver is dumb and just prints.

    if socket_path.exists() {
        let _ = fs::remove_file(socket_path);
    }

    let listener = UnixListener::bind(socket_path).context("Failed to bind debug socket")?;

    println!(
        "{}",
        "Kitchn Debug Watcher (Socket Mode)".bold().underline()
    );
    println!("Listening on: {:?}\n", socket_path);

    // Accept connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle client in thread to allow concurrent clients
                std::thread::spawn(move || {
                    let reader = BufReader::new(stream);
                    for line in reader.lines() {
                        if let Ok(l) = line {
                            // Dumb print: just output what we get
                            // kitchnsink doesn't use println! because msg includes newline?
                            // sender sends with \n.
                            println!("{}", l);
                        } else {
                            break;
                        }
                    }
                });
            }
            Err(e) => eprintln!("Accept error: {}", e),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 1. Handle Internal Watcher (Server Mode)
    if let Some(Commands::InternalWatch { socket_path }) = &cli.command {
        return run_socket_watcher(socket_path);
    }

    // 2. If --debug, spawn viewer (Server) if needed
    if cli.debug {
        spawn_debug_viewer()?;
    }

    // 3. Init Logging (Client Mode)
    // If --debug was set, we hopefully spawned the viewer and socket is ready.
    // init_logging will try to connect.
    let logging_enabled = init_logging(cli.debug)?;

    // Acquire global lock (clients only)
    let _lock_file = match acquire_lock() {
        Ok(f) => Some(f),
        Err(e) => {
            warn!("Failed to acquire global lock: {}", e);
            eprintln!("Error: Another instance of kitchn is already running.");
            return Ok(());
        }
    };

    // Handle Commands
    match cli.command {
        None => {
            if !cli.debug {
                use clap::CommandFactory;
                Cli::command().print_help()?;
            }
            return Ok(());
        }
        Some(cmd) => {
            if logging_enabled {
                debug!("Executing command: {:?}", cmd);
            }
            process_command(cmd)?;
        }
    }

    Ok(())
}

fn acquire_lock() -> Result<File> {
    use std::os::unix::io::AsRawFd;

    let runtime_dir = directories::BaseDirs::new()
        .and_then(|d| d.runtime_dir().map(|p| p.to_path_buf()))
        .unwrap_or_else(env::temp_dir);

    if !runtime_dir.exists() {
        let _ = fs::create_dir_all(&runtime_dir);
    }

    let lock_path = runtime_dir.join("kitchn.lock");
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&lock_path)
        .context("Failed to open lock file")?;

    let fd = file.as_raw_fd();
    let ret = unsafe { libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) };

    if ret != 0 {
        let err = std::io::Error::last_os_error();
        return Err(anyhow::anyhow!(
            "Could not acquire lock (another instance running?): {}",
            err
        ));
    }

    Ok(file)
}

fn process_command(cmd: Commands) -> Result<()> {
    let dirs = ProjectDirs::from("", "", "kitchn").context("Could not determine project dirs")?;
    let data_dir = dirs.data_dir();
    let db_path = data_dir.join("pantry.db");
    let mut db = Pantry::load(&db_path)?;
    let config = Cookbook::load().context("Failed to load Kitchn cookbook")?;

    match cmd {
        Commands::Stock { path } => {
            let installed = stock_pantry(&path, &mut db, &config)?;
            db.save()?;

            for pkg in installed {
                log_msg(
                    &config,
                    "cook_start",
                    &format!("simmering {}", pkg.meta.name),
                );
                let _ = processor::apply(&pkg, &config)?;
            }
        }
        Commands::Wrap { input, output } => {
            let out = output.unwrap_or_else(|| {
                let name = input.file_name().unwrap_or_default().to_string_lossy();
                PathBuf::from(format!("{}.bag", name))
            });
            packager::pack(&input, &out)?;
            log_msg(
                &config,
                "wrap_ok",
                &format!("wrapped package to {}", out.display()),
            );
        }
        Commands::Cook => {
            cook_db(&db, &config)?;
        }
        Commands::Pantry { command } => match command {
            Some(PantryCommands::Clean) => {
                let count = db.list().len();
                if count == 0 {
                    log_msg(&config, "pantry_empty", "pantry is already empty");
                } else {
                    db.clean();
                    db.save()?;
                    log_msg(
                        &config,
                        "pantry_clean_ok",
                        &format!("removed {} ingredients", count),
                    );
                }
            }
            None => {
                list_pantry(&db, &config);
            }
        },
        Commands::Bake => {
            bake_config(&dirs, &config)?;
        }
        Commands::InternalWatch { .. } => {}
    }
    Ok(())
}

fn stock_pantry(path: &Path, db: &mut Pantry, config: &Cookbook) -> Result<Vec<Ingredient>> {
    let mut installed_list = Vec::new();

    if !path.exists() {
        return Err(anyhow!("File not found: {:?}", path));
    }

    if path.extension().is_some_and(|ext| ext == "bag") {
        let file = fs::File::open(path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name().ends_with(".ing") {
                let mut content = String::new();
                std::io::Read::read_to_string(&mut file, &mut content)?;
                let pkg: Ingredient = toml::from_str(&content).with_context(|| {
                    format!("Failed to parse ingredient inside zip: {}", file.name())
                })?;

                log_msg(
                    config,
                    "stock_ok",
                    &format!("stocked {} v{}", pkg.meta.name, pkg.meta.version),
                );

                let pkg_clone = pkg.clone();
                db.store(pkg)?;
                installed_list.push(pkg_clone);
            }
        }
    } else {
        let content = fs::read_to_string(path)?;
        let pkg: Ingredient = toml::from_str(&content)
            .with_context(|| format!("Failed to parse ingredient: {:?}", path))?;

        log_msg(
            config,
            "stock_ok",
            &format!("stocked {} v{}", pkg.meta.name, pkg.meta.version),
        );
        let pkg_clone = pkg.clone();
        db.store(pkg)?;
        installed_list.push(pkg_clone);
    }
    Ok(installed_list)
}

fn list_pantry(db: &Pantry, config: &Cookbook) {
    use colored::*;
    println!("{}", "\nStocked Ingredients (Pantry):\n".bold().underline());

    let fragments = db.list();
    if fragments.is_empty() {
        log(config, "pantry_empty");
        return;
    }

    for pkg in fragments {
        println!(
            "  {} {}\n    {}\n    {}",
            pkg.meta.name.blue().bold(),
            format!("v{}", pkg.meta.version).green(),
            pkg.meta.description.italic(),
            format!("by {}", pkg.meta.authors.join(", ")).dimmed()
        );
        println!();
    }
}

fn cook_db(db: &Pantry, config: &Cookbook) -> Result<()> {
    let ingredients = db.list();
    if ingredients.is_empty() {
        log(config, "cook_empty");
        return Ok(());
    }

    let count = ingredients.len();
    let mut hook_failures = 0;

    for pkg in ingredients {
        log_msg(
            config,
            "cook_start",
            &format!("simmering <primary>{}</primary>", pkg.meta.name),
        );
        if !processor::apply(pkg, config)? {
            hook_failures += 1;
        }
    }

    if hook_failures > 0 {
        log_msg(
            config,
            "cook_ok",
            &format!(
                "cooked {} ingredients successfully but {} hooks failed",
                count, hook_failures
            ),
        );
    } else {
        log_msg(
            config,
            "cook_ok",
            &format!("cooked {} ingredients successfully", count),
        );
    }

    Ok(())
}

fn bake_config(dirs: &ProjectDirs, config: &Cookbook) -> Result<()> {
    log(config, "bake_start");
    let config_dir = dirs.config_dir();
    let cache_dir = dirs.cache_dir();

    log_msg(config, "bake_scan", &config_dir.to_string_lossy());

    let files = ["theme.toml", "icons.toml", "layout.toml", "cookbook.toml"];
    for f in files {
        let p = config_dir.join(f);
        if p.exists() {
            log_msg(config, "bake_file", f);
        }
    }

    let bin_path = cache_dir.join("pastry.bin");
    if bin_path.exists() {
        let _ = fs::remove_file(&bin_path);
    }

    match Cookbook::load_from_dir(config_dir) {
        Ok(new_config) => {
            log_msg(config, "bake_save", &bin_path.to_string_lossy());
            if let Err(e) = new_config.save_binary(&bin_path) {
                log(config, "bake_fail");
                return Err(anyhow!("Failed to save binary config: {}", e));
            }
            log_msg(
                config,
                "bake_ok",
                &format!("baked configuration to {}", bin_path.display()),
            );
        }
        Err(e) => {
            log(config, "bake_fail");
            return Err(anyhow!("Failed to load configuration: {}", e));
        }
    }

    Ok(())
}
