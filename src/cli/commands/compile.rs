use super::super::logging::{debug, error, info};
use crate::config::{self, Config};
use anyhow::{Result, anyhow};
use std::fs;

pub fn execute(_config: &Config) -> Result<()> {
    info("COMPILE", "Starting config compilation...");

    let conf_path = config::config_path();
    let cache_path = config::cache_file();

    debug("COMPILE", &format!("Scanning {}", conf_path.display()));

    if conf_path.exists() {
        debug("COMPILE", "Found hyprink.conf");
    }

    if cache_path.exists() {
        let _ = fs::remove_file(&cache_path);
    }

    match Config::load_from_path(&conf_path) {
        Ok(new_config) => {
            debug("COMPILE", &format!("Saving to {}", cache_path.display()));
            if let Err(e) = new_config.save_cache(&cache_path) {
                error("COMPILE", "Failed to compile config");
                return Err(anyhow!("Failed to save config cache: {}", e));
            }
            info(
                "COMPILE",
                &format!("compiled config to {}", cache_path.display()),
            );
        }
        Err(e) => {
            error("COMPILE", "Failed to compile config");
            return Err(anyhow!("Failed to load configuration: {}", e));
        }
    }

    Ok(())
}
