use anyhow::{Context, Result};
use log::debug;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tera::{Context as TeraContext, Tera};

use crate::config::Config;
use crate::logger;
use crate::template::Template;

use std::collections::HashMap;
use tera::{Value, to_value, try_get_value};

pub fn apply(template: &Template, config: &Config, _force: bool) -> Result<bool> {
    debug!("Applying template: {}", template.manifest.name);
    let mut tera = Tera::default();
    tera.register_filter("hex_to_rgb", hex_to_rgb);
    tera.register_filter("hex_to_godot_color", hex_to_godot_color);

    let mut ctx = TeraContext::new();

    // Context Setup
    ctx.insert("colors", &config.theme.colors);
    ctx.insert("fonts", &config.theme.fonts);

    let active_icons = if config.theme.active_icons == "nerdfont" {
        &config.icons.nerdfont
    } else {
        &config.icons.ascii
    };
    ctx.insert("icons", active_icons);

    if log::log_enabled!(log::Level::Debug) {
        debug!(
            "Tera Context available for '{}': {:#?}",
            template.manifest.name, ctx
        );
    }

    process_template(template, &mut tera, &mut ctx, config)
}

/// Tera filter: hex_to_rgb
fn hex_to_rgb(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("hex_to_rgb", "value", String, value);
    let hex = s.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(tera::Error::msg(format!("Invalid hex color: {}", s)));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| tera::Error::msg("Invalid hex"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| tera::Error::msg("Invalid hex"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| tera::Error::msg("Invalid hex"))?;

    Ok(to_value(vec![r, g, b]).unwrap())
}

/// Tera filter: hex_to_godot_color
fn hex_to_godot_color(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("hex_to_godot_color", "value", String, value);
    let hex = s.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(tera::Error::msg(format!("Invalid hex color: {}", s)));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| tera::Error::msg("Invalid hex"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| tera::Error::msg("Invalid hex"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| tera::Error::msg("Invalid hex"))?;

    let r_float = (r as f32) / 255.0;
    let g_float = (g as f32) / 255.0;
    let b_float = (b as f32) / 255.0;

    let godot_color = format!("Color({:.3}, {:.3}, {:.3}, 1)", r_float, g_float, b_float);
    Ok(to_value(godot_color).unwrap())
}

fn process_template(
    tpl: &Template,
    tera: &mut Tera,
    ctx: &mut TeraContext,
    _config: &Config,
) -> Result<bool> {
    debug!(
        "Processing template targets and hooks for: {}",
        tpl.manifest.name
    );

    // Render Targets
    for target in &tpl.targets {
        render_and_write(&target.target, &target.content, tera, ctx)?;
    }

    // Render Files
    for file in &tpl.files {
        render_and_write(&file.target, &file.content, tera, ctx)?;
    }

    let mut hooks_success = true;

    // Run Hooks
    if let Some(cmd) = &tpl.hooks.reload {
        debug!("Found reload hook requested: '{}'", cmd);

        let name = &tpl.manifest.name;

        // Log hook execution
        let run_msg = format!("[{}] running: {}", name, cmd);
        logger::info("HOOK", &run_msg);

        debug!("Executing hook via 'sh -c': {}", cmd);
        let start = std::time::Instant::now();

        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .context("Failed to execute hook")?;

        let duration = start.elapsed();
        debug!(
            "Hook completed in {:?} with exit code: {}",
            duration, output.status
        );

        if !output.stdout.is_empty() {
            let s = String::from_utf8_lossy(&output.stdout);
            debug!("Hook stdout:\n{}", s.trim());
            for line in s.lines() {
                let out_msg = format!("[{}] {}", name, line);
                logger::info("HOOK", &out_msg);
            }
        }

        if !output.stderr.is_empty() {
            let s = String::from_utf8_lossy(&output.stderr);
            debug!("Hook stderr:\n{}", s.trim());
            for line in s.lines() {
                let err_msg = format!("[{}] {}", name, line);
                logger::error("HOOK", &err_msg);
            }
        }

        if output.status.success() {
            let ok_msg = format!("[{}] ok: {}", name, cmd);
            logger::info("HOOK", &ok_msg);
        } else {
            let exit_code = output.status.code().unwrap_or(-1);
            let fail_msg = format!("[{}] failed (exit {}): {}", name, exit_code, cmd);
            logger::error("HOOK", &fail_msg);
            hooks_success = false;
        }
    }

    Ok(hooks_success)
}

fn render_and_write(target: &str, content: &str, tera: &mut Tera, ctx: &TeraContext) -> Result<()> {
    debug!("Rendering target: {}", target);

    let target_expanded = if target.starts_with("~") {
        let home = directories::UserDirs::new()
            .context("Could not determine home directory")?
            .home_dir()
            .to_string_lossy()
            .to_string();
        target.replace("~", &home)
    } else {
        target.to_string()
    };

    debug!("Expanded target path: {}", target_expanded);

    let path = PathBuf::from(&target_expanded);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let rendered = tera
        .render_str(content, ctx)
        .context("Failed to render template")?;

    fs::write(path, rendered)?;
    Ok(())
}
