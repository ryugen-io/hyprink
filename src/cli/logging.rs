//! CLI logging integration with hyprdt

use crate::logger;
use anyhow::{Context, Result};
use hyprdt::HyprdtLayer;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use tracing_subscriber::{EnvFilter, prelude::*};

/// Log an info message
pub fn info(scope: &str, msg: &str) {
    logger::info(scope, msg);
    tracing::info!(scope = scope, message = msg);
}

/// Log a debug message
pub fn debug(scope: &str, msg: &str) {
    logger::debug(scope, msg);
    tracing::debug!(scope = scope, message = msg);
}

/// Log a warning message
pub fn warn(scope: &str, msg: &str) {
    logger::warn(scope, msg);
    tracing::warn!(scope = scope, message = msg);
}

/// Log an error message
pub fn error(scope: &str, msg: &str) {
    logger::error(scope, msg);
    tracing::error!(scope = scope, message = msg);
}

fn get_socket_path() -> PathBuf {
    hyprdt::default_socket_path()
}

pub fn init_logging(force_debug: bool) -> Result<bool> {
    let _ = tracing_log::LogTracer::init();

    let socket_path = get_socket_path();
    let watcher_active = socket_path.exists();

    let enable_debug = force_debug || watcher_active;

    let env_filter = if enable_debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    let registry = tracing_subscriber::registry().with(env_filter);

    // Use hyprdt's layer instead of custom implementation
    let layer = HyprdtLayer::new("hyprink");
    let _ = tracing::subscriber::set_global_default(registry.with(layer));

    Ok(true)
}

pub fn spawn_debug_viewer() -> Result<()> {
    let socket_path = get_socket_path();

    // Check if hyprdt is already running
    if socket_path.exists() {
        return Ok(());
    }

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
        tracing::debug!("Spawning hyprdt with: {}", term);

        // Try to find hyprdt binary
        let hyprdt_path = which::which("hyprdt").unwrap_or_else(|_| PathBuf::from("hyprdt"));

        let _ = Command::new(&term)
            .arg("-e")
            .arg(&hyprdt_path)
            .arg("--app")
            .arg("hyprink")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn hyprdt terminal")?;

        println!("Debug Mode Started (hyprdt).");
        println!("Socket: {:?}", socket_path);

        // Wait for socket to be ready
        let start = std::time::Instant::now();
        while !socket_path.exists() && start.elapsed() < Duration::from_secs(2) {
            std::thread::sleep(Duration::from_millis(50));
        }
        std::thread::sleep(Duration::from_millis(100));
    } else {
        tracing::warn!("No supported terminal emulator found.");
        println!("Cannot spawn debug terminal.");
    }

    Ok(())
}
