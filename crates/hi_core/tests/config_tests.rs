use hi_core::config::{
    Config, ConfigError, RetentionConfig, cache_dir, cache_file, config_path, data_dir,
};
use std::path::{Path, PathBuf};
use tempfile::tempdir;

mod common;

#[test]
fn test_cache_roundtrip() {
    let dir = tempdir().unwrap();
    let cache_path = dir.path().join("config.bin");
    let config = common::create_test_config();
    config.save_cache(&cache_path).unwrap();
    assert!(cache_path.exists());
}

#[test]
fn test_retention_config_defaults() {
    let retention = RetentionConfig::default();
    assert_eq!(retention.max_age_days, 30);
    assert!(retention.max_total_size.is_none());
    assert_eq!(retention.compress_after_days, Some(7));
}

#[test]
fn test_config_path_returns_path() {
    let path = config_path();
    assert!(path.to_string_lossy().contains("hyprink.conf"));
}

#[test]
fn test_cache_dir_returns_path() {
    let dir = cache_dir();
    assert!(dir.to_string_lossy().contains("hyprink"));
}

#[test]
fn test_data_dir_returns_path() {
    let dir = data_dir();
    assert!(dir.to_string_lossy().contains("hyprink"));
}

#[test]
fn test_cache_file_returns_path() {
    let file = cache_file();
    assert!(file.to_string_lossy().contains("config.bin"));
}

#[test]
fn test_load_from_path_not_found() {
    let result = Config::load_from_path(Path::new("/nonexistent/path/config.toml"));
    assert!(result.is_err());
}

#[test]
fn test_load_from_valid_toml() {
    let dir = tempdir().unwrap();
    let conf_path = dir.path().join("test.conf");

    let toml_content = r##"
[theme]
name = "TestTheme"
active_icons = "ascii"

[theme.colors]
bg = "#000000"

[theme.fonts]
mono = "Consolas"

[icons.nerdfont]
success = ""

[icons.ascii]
success = "+"

[layout.tag]
prefix = "["
suffix = "]"
transform = "lowercase"
min_width = 3
alignment = "left"

[layout.labels]
info = "INF"

[layout.structure]
terminal = "{msg}"
file = "{msg}"

[layout.logging]
base_dir = "/tmp"
path_structure = "{app}"
filename_structure = "{level}.log"
timestamp_format = "%H:%M"
write_by_default = true
"##;
    std::fs::write(&conf_path, toml_content).unwrap();

    let config = Config::load_from_path(&conf_path).unwrap();
    assert_eq!(config.theme.name, "TestTheme");
    assert_eq!(config.theme.active_icons, "ascii");
    assert_eq!(config.layout.tag.transform, "lowercase");
    assert!(config.layout.logging.write_by_default);
}

#[test]
fn test_config_error_display() {
    let err = ConfigError::ConfigDirNotFound;
    assert!(err.to_string().contains("config directory"));

    let err = ConfigError::ConfigFileNotFound(PathBuf::from("/test/path"));
    assert!(err.to_string().contains("/test/path"));
}
