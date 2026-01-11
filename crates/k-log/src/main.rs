mod cli;
mod commands;

use anyhow::{Context, Result};
use clap::Parser;
use k_lib::config::Cookbook;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Cookbook::load().context("Failed to load Kitchn config")?;

    match cli.command {
        Some(Commands::Stats { app }) => commands::stats::run(&config, app.as_deref()),

        Some(Commands::Cleanup {
            max_age,
            max_size,
            app,
            all,
            dry_run,
        }) => commands::cleanup::run(&config, max_age, max_size, app, all, dry_run),

        None => {
            let preset = cli.preset.context("Preset name required")?;
            let msg = cli.msg.as_ref().map(|v| v.join(" "));
            commands::log::run(&config, &preset, msg.as_deref(), cli.app.as_deref())
        }
    }
}
