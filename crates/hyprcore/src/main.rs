use hyprcore::fragment::Fragment;
use hyprcore::packager;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use colored::Colorize;
use core_lib::config::HyprConfig;
use directories::ProjectDirs;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tera::{Context as TeraContext, Tera};

#[derive(Parser)]
#[command(name = "hyprcore", version, about = "Hyprcore Fragment Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a .frag fragment or .fpkg package
    Install { path: PathBuf },
    /// Pack .frag files from a directory into a .fpkg
    Pack {
        /// Directory containing .frag files
        input: PathBuf,
        /// Output .fpkg file (optional, defaults to <dirname>.fpkg)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Sync all fragments (render templates)
    Sync,
    /// List installed fragments
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dirs = ProjectDirs::from("", "", "hyprcore").context("Could not determine project dirs")?;
    let data_dir = dirs.data_dir(); // ~/.local/share/hyprcore
    let fragments_dir = data_dir.join("fragments");

    match cli.command {
        Commands::Install { path } => {
            install_fragment_or_package(&path, &fragments_dir)?;
            println!("{}", "Installed successfully.".green());
            sync_fragments(&fragments_dir)?;
        }
        Commands::Pack { input, output } => {
            let out = output.unwrap_or_else(|| {
                let name = input.file_name().unwrap_or_default().to_string_lossy();
                PathBuf::from(format!("{}.fpkg", name))
            });
            packager::pack(&input, &out)?;
            println!("{} {}", "Created package:".green(), out.display());
        }
        Commands::Sync => {
            sync_fragments(&fragments_dir)?;
        }
        Commands::List => {
            list_fragments(&fragments_dir)?;
        }
    }

    Ok(())
}

fn install_fragment_or_package(path: &Path, fragments_dir: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("File not found: {:?}", path));
    }

    fs::create_dir_all(fragments_dir)?;

    // Check if it's a .fpkg package
    if path.extension().is_some_and(|ext| ext == "fpkg") {
        packager::unpack(path, fragments_dir)?;
    } else {
        // Single .frag file
        let filename = path.file_name().context("Invalid filename")?;
        let target = fragments_dir.join(filename);
        fs::copy(path, target)?;
    }
    Ok(())
}

fn list_fragments(fragments_dir: &Path) -> Result<()> {
    if !fragments_dir.exists() {
        println!("No fragments installed.");
        return Ok(());
    }

    for entry in fs::read_dir(fragments_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "frag") {
            // Try to parse to get ID
            let content = fs::read_to_string(&path)?;
            if let Ok(pkg) = toml::from_str::<Fragment>(&content) {
                println!(
                    "{} ({})",
                    pkg.meta.id.cyan(),
                    path.file_name().unwrap().to_string_lossy()
                );
            } else {
                println!(
                    "{} (Invalid)",
                    path.file_name().unwrap().to_string_lossy().red()
                );
            }
        }
    }
    Ok(())
}

fn sync_fragments(fragments_dir: &Path) -> Result<()> {
    println!("Syncing fragments...");

    // 1. Load Config
    let config = HyprConfig::load().context("Failed to load Hyprcore config")?;

    // 2. Prepare Tera Context
    let mut ctx = TeraContext::new();

    // Colors
    ctx.insert("colors", &config.theme.colors);

    // Fonts
    ctx.insert("fonts", &config.theme.fonts);

    // Icons (Active Set)
    let active_icons = if config.theme.settings.active_icons == "nerdfont" {
        &config.icons.nerdfont
    } else {
        &config.icons.ascii
    };
    ctx.insert("icons", active_icons);

    // 3. Process Fragments
    if !fragments_dir.exists() {
        println!("No fragments to sync.");
        return Ok(());
    }

    for entry in fs::read_dir(fragments_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "frag") {
            process_fragment(&path, &ctx)?;
        }
    }

    println!("{}", "Sync complete.".green());
    Ok(())
}

fn process_fragment(path: &Path, ctx: &TeraContext) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let pkg: Fragment = toml::from_str(&content).context(format!("Failed to parse {:?}", path))?;

    println!("Processing {}...", pkg.meta.id);

    // Render Templates
    for tpl in &pkg.templates {
        render_and_write(&tpl.target, &tpl.content, ctx)?;
    }

    // Render Files (Same logic, maybe different semantic meaning but same operation)
    for file in &pkg.files {
        render_and_write(&file.target, &file.content, ctx)?;
    }

    // Run Hooks
    if let Some(cmd) = &pkg.hooks.reload {
        println!("  Running hook: {}", cmd);
        println!("  Running hook: {}", cmd);
        let _ = Command::new("sh").arg("-c").arg(cmd).spawn();
    }

    Ok(())
}

fn render_and_write(
    target_template: &str,
    content_template: &str,
    ctx: &TeraContext,
) -> Result<()> {
    // Expand ~ in target path
    let target_path_str = if target_template.starts_with("~") {
        let home = directories::UserDirs::new().context("Could not find home dir")?;
        target_template.replace("~", home.home_dir().to_str().unwrap())
    } else {
        target_template.to_string()
    };

    let target_path = PathBuf::from(&target_path_str);

    // Render Content
    let rendered =
        Tera::one_off(content_template, ctx, false).context("Failed to render template")?;

    // Write
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&target_path, rendered)?;
    println!("  -> Wrote {}", target_path.display());

    Ok(())
}
