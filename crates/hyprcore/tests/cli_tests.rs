use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_cli_pack_and_install() {
    let dir = tempdir().unwrap();
    let source_dir = dir.path().join("source");
    let output_file = dir.path().join("test.fpkg");

    fs::create_dir_all(&source_dir).unwrap();
    fs::write(source_dir.join("test.frag"), "[meta]\nid = \"test\"\n").unwrap();

    cargo_bin_cmd!("hyprcore")
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
    cargo_bin_cmd!("hyprcore")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}


