use anyhow::{Context, Result};
use k_lib::config::Cookbook;
use k_lib::logger;

pub fn run(config: &Cookbook, preset_name: &str, msg_override: Option<&str>, app: Option<&str>) -> Result<()> {
    let preset = config
        .dictionary
        .presets
        .get(preset_name)
        .context(format!("Preset '{}' not found in dictionary.toml", preset_name))?;

    let level = &preset.level;
    let scope = preset
        .scope
        .as_ref()
        .context("Preset missing 'scope' field")?;

    let msg = msg_override.unwrap_or(&preset.msg);

    logger::log_to_terminal(config, level, scope, msg);

    if config.layout.logging.write_by_default {
        logger::log_to_file(config, level, scope, msg, app)?;
    }

    Ok(())
}
