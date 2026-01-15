use crate::config::Config;
use anyhow::Result;
use hl_core::Level;
use std::str::FromStr;

// Re-export types from hl_core
pub use hl_core::{CleanupOptions, CleanupResult, LogStats};

pub fn log_to_terminal(config: &Config, level: &str, scope: &str, msg: &str) {
    let hl_level = Level::from_str(level).unwrap_or(Level::Info);

    let template = &config.layout.structure.terminal;

    let logger = hl_core::Logger::builder()
        .terminal()
        .colors(true)
        .structure(template)
        .done()
        .build();

    logger.log(hl_level, scope, msg);
}

pub fn log_to_file(
    config: &Config,
    level: &str,
    scope: &str,
    msg: &str,
    app_override: Option<&str>,
) -> Result<()> {
    let hl_level = Level::from_str(level).unwrap_or(Level::Info);
    let app_name = app_override.unwrap_or(&config.layout.logging.app_name);

    let logger = hl_core::Logger::builder()
        .file()
        .base_dir(&config.layout.logging.base_dir)
        .app_name(app_name)
        .path_structure(&config.layout.logging.path_structure)
        .filename_structure(&config.layout.logging.filename_structure)
        .content_structure(&config.layout.structure.file)
        .timestamp_format(&config.layout.logging.timestamp_format)
        .done()
        .build();

    logger.log(hl_level, scope, msg);
    Ok(())
}

pub fn cleanup(config: &Config, options: CleanupOptions) -> Result<CleanupResult> {
    let base_dir = resolve_base_dir(&config.layout.logging.base_dir)?;
    helpers::cleanup(&base_dir, &options).map_err(|e| anyhow::anyhow!("Cleanup error: {}", e))
}

pub fn stats(config: &Config, app: Option<&str>) -> Result<LogStats> {
    let base_dir = resolve_base_dir(&config.layout.logging.base_dir)?;
    helpers::stats(&base_dir, app).map_err(|e| anyhow::anyhow!("Stats error: {}", e))
}

fn resolve_base_dir(base_dir_str: &str) -> Result<std::path::PathBuf> {
    if base_dir_str.starts_with('~') {
        let home = directories::UserDirs::new()
            .ok_or_else(|| anyhow::anyhow!("Could not find home dir"))?;
        Ok(std::path::PathBuf::from(
            base_dir_str.replace('~', home.home_dir().to_str().unwrap()),
        ))
    } else {
        Ok(std::path::PathBuf::from(base_dir_str))
    }
}

mod helpers {
    pub use hl_core::{cleanup, stats};
}
