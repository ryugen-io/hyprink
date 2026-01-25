use anyhow::{Context, Result, anyhow};
use clap::{CommandFactory, Parser};
use hyprink::cli::args::Cli;
use hyprink::cli::commands;
use hyprink::cli::logging::{init_logging, spawn_debug_viewer};
use std::env;
use std::fs;
use std::os::unix::io::AsRawFd;
use tracing::{debug, warn};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If --debug, spawn hyprdt viewer if needed
    if cli.debug {
        spawn_debug_viewer()?;
    }

    // Init Logging
    let logging_enabled = init_logging(cli.debug)?;

    // Acquire global lock (clients only)
    let _lock_file = match acquire_lock() {
        Ok(f) => Some(f),
        Err(e) => {
            warn!("Failed to acquire global lock: {}", e);
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };

    // Handle Commands
    match cli.command {
        None => {
            if !cli.debug {
                Cli::command().print_help()?;
            }
            return Ok(());
        }
        Some(cmd) => {
            if logging_enabled {
                debug!("Executing command: {:?}", cmd);
            }
            commands::process_command(cmd)?;
        }
    }

    Ok(())
}

fn acquire_lock() -> Result<fs::File> {
    let runtime_dir = directories::BaseDirs::new()
        .and_then(|d| d.runtime_dir().map(|p| p.to_path_buf()))
        .unwrap_or_else(env::temp_dir);

    debug!("Using runtime directory for lock: {:?}", runtime_dir);

    if !runtime_dir.exists() {
        let _ = fs::create_dir_all(&runtime_dir);
    }

    let lock_path = runtime_dir.join("hyprink.lock");
    let file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&lock_path)
        .with_context(|| format!("Failed to open lock file at {:?}", lock_path))?;

    let fd = file.as_raw_fd();
    let ret = unsafe { libc::flock(fd, libc::LOCK_EX | libc::LOCK_NB) };

    if ret != 0 {
        let err = std::io::Error::last_os_error();
        return Err(anyhow!(
            "Could not acquire lock for {:?} (another instance running?). OS Error: {} (code: {:?})",
            lock_path,
            err,
            err.raw_os_error()
        ));
    }

    Ok(file)
}
