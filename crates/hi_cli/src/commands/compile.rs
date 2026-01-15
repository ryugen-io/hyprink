use crate::logging::{log, log_msg};
use anyhow::{Result, anyhow};
use hi_core::config::{self, Config};
use std::fs;

pub fn execute(config: &Config) -> Result<()> {
    log(config, "compile_start");

    let conf_path = config::config_path();
    let cache_path = config::cache_file();

    log_msg(config, "compile_scan", &conf_path.to_string_lossy());

    if conf_path.exists() {
        log_msg(config, "compile_file", "hyprink.conf");
    }

    if cache_path.exists() {
        let _ = fs::remove_file(&cache_path);
    }

    match Config::load_from_path(&conf_path) {
        Ok(new_config) => {
            log_msg(config, "compile_save", &cache_path.to_string_lossy());
            if let Err(e) = new_config.save_cache(&cache_path) {
                log(config, "compile_fail");
                return Err(anyhow!("Failed to save config cache: {}", e));
            }
            log_msg(
                config,
                "compile_ok",
                &format!("compiled config to {}", cache_path.display()),
            );
        }
        Err(e) => {
            log(config, "compile_fail");
            return Err(anyhow!("Failed to load configuration: {}", e));
        }
    }

    Ok(())
}
