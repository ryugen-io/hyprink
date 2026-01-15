use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn setup_config(root: &Path) -> std::path::PathBuf {
    let config_home = root.join("config");
    let hypr_config = config_home.join("hypr");
    fs::create_dir_all(&hypr_config).unwrap();

    // Create minimal hyprink.conf
    fs::write(
        hypr_config.join("hyprink.conf"),
        r#"
[theme]
name = "test"
active_icons = "none"
[theme.colors]
[theme.fonts]

[icons]
[icons.nerdfont]
[icons.ascii]

[layout]
[layout.tag]
prefix = ""
suffix = ""
transform = "none"
min_width = 0
alignment = "left"
[layout.labels]
[layout.structure]
terminal = "{msg}"
file = "{msg}"
[layout.logging]
base_dir = "logs"
path_structure = "sys.log"
filename_structure = "log"
timestamp_format = ""
write_by_default = false
"#,
    )
    .unwrap();

    config_home
}

#[test]
fn test_cli_pack_and_install() {
    let dir = tempdir().unwrap();
    let config_home = setup_config(dir.path());
    let source_dir = dir.path().join("source");
    let output_file = dir.path().join("test.pkg");

    fs::create_dir_all(&source_dir).unwrap();
    fs::write(
        source_dir.join("test.tpl"),
        r#"[manifest]
name = "test"
version = "0.1.0"
authors = ["Test"]
description = "Test template"
"#,
    )
    .unwrap();

    cargo_bin_cmd!("hyprink")
        .env("XDG_CONFIG_HOME", &config_home)
        .env("XDG_CACHE_HOME", dir.path().join("cache"))
        .env("XDG_DATA_HOME", dir.path().join("data"))
        .arg("pack")
        .arg(&source_dir)
        .arg("-o")
        .arg(&output_file)
        .assert()
        .success();

    assert!(output_file.exists());
}

#[test]
fn test_cli_help() {
    let _dir = tempdir().unwrap();
    cargo_bin_cmd!("hyprink")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}
