use anyhow::{Context, Result};
use chrono::Local;
use clap::Parser;
use colored::Colorize;
use core_lib::config::HyprConfig;
use core_lib::factory::{ColorResolver, TagFactory};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "corelog", version, about = "Hyprcore Logging Tool")]
struct Cli {
    /// Preset key from dictionary.toml
    preset: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = HyprConfig::load().context("Failed to load Hyprcore config")?;

    let preset = config.dictionary.presets.get(&cli.preset).context(format!(
        "Preset '{}' not found in dictionary.toml",
        cli.preset
    ))?;

    let level = &preset.level;
    let scope = preset
        .scope
        .as_ref()
        .context("Preset missing 'scope' field")?;
    let msg = &preset.msg;

    log_to_terminal(&config, level, scope, msg);

    if config.layout.logging.write_by_default {
        log_to_file(&config, level, scope, msg)?;
    }

    Ok(())
}

fn log_to_terminal(config: &HyprConfig, level: &str, scope: &str, msg: &str) {
    let icon_set_key = &config.theme.settings.active_icons;
    let icon = if icon_set_key == "nerdfont" {
        config
            .icons
            .nerdfont
            .get(level)
            .map(|s| s.as_str())
            .unwrap_or("?")
    } else {
        config
            .icons
            .ascii
            .get(level)
            .map(|s| s.as_str())
            .unwrap_or("?")
    };

    let tag = TagFactory::create_tag(config, level);

    let color_hex = config
        .theme
        .colors
        .get(level)
        .or_else(|| config.theme.colors.get("fg"))
        .map(|s| s.as_str())
        .unwrap_or("#ffffff");

    let color = ColorResolver::hex_to_color(color_hex);

    let mut output = config.layout.structure.terminal.clone();
    output = output.replace("{icon}", icon);
    output = output.replace("{tag}", &tag);
    output = output.replace("{scope}", scope);
    output = output.replace("{msg}", msg);

    println!("{}", output.custom_color(color));
}

fn log_to_file(config: &HyprConfig, level: &str, scope: &str, msg: &str) -> Result<()> {
    let now = Local::now();

    let tag = TagFactory::create_tag(config, level);
    let timestamp = now
        .format(&config.layout.logging.timestamp_format)
        .to_string();

    let mut content = config.layout.structure.file.clone();
    content = content.replace("{timestamp}", &timestamp);
    content = content.replace("{tag}", &tag);
    content = content.replace("{msg}", msg);
    content = content.replace("{scope}", scope);

    let base_dir_str = &config.layout.logging.base_dir;
    let base_dir = if base_dir_str.starts_with("~") {
        let home = directories::UserDirs::new().context("Could not find home dir")?;
        PathBuf::from(base_dir_str.replace("~", home.home_dir().to_str().unwrap()))
    } else {
        PathBuf::from(base_dir_str)
    };

    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    let day = now.format("%d").to_string();

    let mut rel_path = config.layout.logging.path_structure.clone();
    rel_path = rel_path.replace("{year}", &year);
    rel_path = rel_path.replace("{month}", &month);
    rel_path = rel_path.replace("{scope}", scope);

    let mut filename = config.layout.logging.filename_structure.clone();
    filename = filename.replace("{level}", level);
    filename = filename.replace("{year}", &year);
    filename = filename.replace("{month}", &month);
    filename = filename.replace("{day}", &day);

    let full_dir = base_dir.join(rel_path);
    fs::create_dir_all(&full_dir)?;

    let file_path = full_dir.join(filename);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(file, "{}", content)?;

    Ok(())
}
