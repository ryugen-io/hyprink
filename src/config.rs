use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

// === Config Sections ===

#[derive(Debug, Deserialize, Serialize)]
pub struct ThemeSection {
    pub name: String,
    pub active_icons: String,
    pub colors: HashMap<String, String>,
    pub fonts: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IconsSection {
    pub nerdfont: HashMap<String, String>,
    pub ascii: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutSection {
    pub tag: TagConfig,
    pub labels: HashMap<String, String>,
    pub structure: StructureConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagConfig {
    pub prefix: String,
    pub suffix: String,
    pub transform: String,
    pub min_width: usize,
    pub alignment: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StructureConfig {
    pub terminal: String,
    pub file: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub base_dir: String,
    pub path_structure: String,
    pub filename_structure: String,
    pub timestamp_format: String,
    pub write_by_default: bool,
    #[serde(default = "default_app_name")]
    pub app_name: String,
    #[serde(default)]
    pub retention: RetentionConfig,
}

fn default_app_name() -> String {
    "hyprink".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RetentionConfig {
    #[serde(default = "default_max_age_days")]
    pub max_age_days: u32,
    #[serde(default)]
    pub max_total_size: Option<String>,
    #[serde(default = "default_compress_after_days")]
    pub compress_after_days: Option<u32>,
}

impl Default for RetentionConfig {
    fn default() -> Self {
        Self {
            max_age_days: default_max_age_days(),
            max_total_size: None,
            compress_after_days: default_compress_after_days(),
        }
    }
}

fn default_max_age_days() -> u32 {
    30
}

fn default_compress_after_days() -> Option<u32> {
    Some(7)
}

// === Main Config ===

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: ThemeSection,
    pub icons: IconsSection,
    pub layout: LayoutSection,
}

// === Errors ===

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not determine config directory")]
    ConfigDirNotFound,
    #[error("Config file not found: {0}")]
    ConfigFileNotFound(PathBuf),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
}

// === Paths ===

pub fn config_path() -> PathBuf {
    // Respect XDG_CONFIG_HOME, else fall back to ~/.config/hypr/
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs_next::home_dir()
                .map(|h| h.join(".config"))
                .unwrap_or_else(|| PathBuf::from("/etc"))
        })
        .join("hypr/hyprink.conf")
}

pub fn cache_dir() -> PathBuf {
    std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs_next::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp")))
        .join("hyprink")
}

pub fn data_dir() -> PathBuf {
    std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs_next::data_dir().unwrap_or_else(|| PathBuf::from("/tmp")))
        .join("hyprink")
}

pub fn cache_file() -> PathBuf {
    cache_dir().join("config.bin")
}

/// Expand ~ to home directory in paths
pub fn expand_path(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs_next::home_dir()
    {
        return home.join(rest);
    }
    PathBuf::from(path)
}

// === Config Implementation ===

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let conf_path = config_path();
        let bin_path = cache_file();

        Self::load_with_cache(&conf_path, &bin_path, false)
    }

    pub fn load_no_cache() -> Result<Self, ConfigError> {
        let conf_path = config_path();
        let bin_path = cache_file();

        Self::load_with_cache(&conf_path, &bin_path, true)
    }

    pub fn load_from_path(conf_path: &Path) -> Result<Self, ConfigError> {
        let bin_path = cache_file();
        Self::load_with_cache(conf_path, &bin_path, true)
    }

    pub fn load_with_cache(
        conf_path: &Path,
        bin_path: &Path,
        force: bool,
    ) -> Result<Self, ConfigError> {
        // Try binary cache first
        if !force
            && bin_path.exists()
            && Self::is_cache_fresh(bin_path, conf_path)?
            && let Ok(file) = fs::File::open(bin_path)
        {
            let mut reader = std::io::BufReader::new(file);
            match bincode::serde::decode_from_std_read::<Config, _, _>(
                &mut reader,
                bincode::config::standard(),
            ) {
                Ok(cfg) => {
                    debug!("Loaded config from cache: {:?}", bin_path);
                    return Ok(cfg);
                }
                Err(e) => {
                    debug!("Cache decode failed (loading from conf): {}", e);
                }
            }
        } else {
            debug!("Cache miss or stale, loading from conf");
        }

        // Load from hyprink.conf
        if !conf_path.exists() {
            return Err(ConfigError::ConfigFileNotFound(conf_path.to_path_buf()));
        }

        let content = fs::read_to_string(conf_path)?;
        let config: Config = toml::from_str(&content)?;

        debug!("Loaded config from: {:?}", conf_path);

        Ok(config)
    }

    pub fn save_cache(&self, path: &Path) -> Result<(), ConfigError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = fs::File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);

        bincode::serde::encode_into_std_write(self, &mut writer, bincode::config::standard())
            .map_err(|e| ConfigError::Io(std::io::Error::other(e)))?;

        debug!("Saved config cache to: {:?}", path);
        Ok(())
    }

    fn is_cache_fresh(bin_path: &Path, conf_path: &Path) -> Result<bool, ConfigError> {
        let bin_meta = fs::metadata(bin_path)?;
        let bin_mtime = bin_meta.modified()?;

        // Check if executable is newer (embedded defaults changed)
        if let Ok(exe_path) = std::env::current_exe()
            && let Ok(exe_meta) = fs::metadata(&exe_path)
            && let Ok(exe_mtime) = exe_meta.modified()
            && exe_mtime > bin_mtime
        {
            return Ok(false);
        }

        // Check if conf is newer
        if conf_path.exists() {
            let conf_meta = fs::metadata(conf_path)?;
            let conf_mtime = conf_meta.modified()?;
            if conf_mtime > bin_mtime {
                return Ok(false);
            }
        }

        // Check if any file in hyprink.d is newer
        let hyprink_d = conf_path.parent().map(|p| p.join("hyprink.d"));
        if let Some(d) = hyprink_d
            && d.exists()
            && let Ok(entries) = fs::read_dir(&d)
        {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata()
                    && let Ok(mtime) = meta.modified()
                    && mtime > bin_mtime
                {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}
