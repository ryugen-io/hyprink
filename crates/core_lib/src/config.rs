use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    pub meta: ThemeMeta,
    pub settings: ThemeSettings,
    pub colors: HashMap<String, String>,
    pub fonts: HashMap<String, String>, // Using HashMap to allow dynamic font keys like "mono", "ui", "size_mono"
}

#[derive(Debug, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ThemeSettings {
    pub active_icons: String,
}

#[derive(Debug, Deserialize)]
pub struct IconsConfig {
    pub nerdfont: HashMap<String, String>,
    pub ascii: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub tag: TagConfig,
    pub labels: HashMap<String, String>,
    pub structure: StructureConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct TagConfig {
    pub prefix: String,
    pub suffix: String,
    pub transform: String,
    pub min_width: usize,
    pub alignment: String,
}

#[derive(Debug, Deserialize)]
pub struct StructureConfig {
    pub terminal: String,
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub base_dir: String,
    pub path_structure: String,
    pub filename_structure: String,
    pub timestamp_format: String,
    pub write_by_default: bool,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryConfig {
    pub presets: HashMap<String, Preset>,
}

#[derive(Debug, Deserialize)]
pub struct Preset {
    pub level: String,
    pub scope: Option<String>,
    pub msg: String,
}

#[derive(Debug)]
pub struct HyprConfig {
    pub theme: ThemeConfig,
    pub icons: IconsConfig,
    pub layout: LayoutConfig,
    pub dictionary: DictionaryConfig,
}

use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not determine config directory")]
    ConfigDirNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
}

impl HyprConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_dir = Self::get_config_dir()?;

        let theme: ThemeConfig = Self::load_toml(&config_dir.join("theme.toml"))?;
        let icons: IconsConfig = Self::load_toml(&config_dir.join("icons.toml"))?;
        let layout: LayoutConfig = Self::load_toml(&config_dir.join("layout.toml"))?;
        let dictionary: DictionaryConfig = Self::load_toml(&config_dir.join("dictionary.toml"))?;

        Ok(HyprConfig {
            theme,
            icons,
            layout,
            dictionary,
        })
    }

    fn get_config_dir() -> Result<PathBuf, ConfigError> {
        // Use XDG_CONFIG_HOME/hyprcore or ~/.config/hyprcore
        if let Some(proj_dirs) = ProjectDirs::from("", "", "hyprcore") {
            return Ok(proj_dirs.config_dir().to_path_buf());
        }
        Err(ConfigError::ConfigDirNotFound)
    }

    fn load_toml<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: T = toml::from_str(&content)?;
        Ok(config)
    }
}
