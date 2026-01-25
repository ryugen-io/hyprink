pub mod add;
pub mod apply;
pub mod compile;
pub mod list;
pub mod pack;

use super::args::Commands;
use super::logging::{info, warn};
use crate::config::{self, Config};
use crate::db::Store;
use crate::processor;
use anyhow::{Context, Result};

pub fn process_command(cmd: Commands) -> Result<()> {
    let data_dir = config::data_dir();
    let db_path = data_dir.join("store.db");
    let mut db = Store::load(&db_path)?;
    let config = Config::load().context("Failed to load hyprink config")?;

    match cmd {
        Commands::Add { path } => {
            let installed = add::add_template(&path, &mut db, &config)?;
            db.save()?;

            for tpl in installed {
                if tpl.manifest.ignored {
                    info(
                        "APPLY",
                        &format!(
                            "ignoring <secondary>{}</secondary> (disabled)",
                            tpl.manifest.name
                        ),
                    );
                    continue;
                }
                info("APPLY", &format!("applying {}", tpl.manifest.name));
                let _ = processor::apply(&tpl, &config, false)?;
            }
        }
        Commands::Pack { input, output } => {
            pack::execute(input, output, &config)?;
        }
        Commands::Apply {
            toggle_force,
            force,
        } => {
            use super::cli_config::CliConfig;

            let mut current_force = force;

            if toggle_force {
                let mut cli_conf = CliConfig::load().unwrap_or_default();
                cli_conf.force_apply = !cli_conf.force_apply;

                if cli_conf.force_apply {
                    info("APPLY", "FORCE MODE ENABLED (persistent)");
                } else {
                    info("APPLY", "FORCE MODE DISABLED (persistent)");
                }

                if let Err(e) = cli_conf.save() {
                    warn("APPLY", &format!("Failed to save CLI config: {}", e));
                }

                if cli_conf.force_apply {
                    current_force = true;
                }
            } else if CliConfig::load().unwrap_or_default().force_apply {
                current_force = true;
            }

            let final_config = if current_force {
                match Config::load_no_cache() {
                    Ok(c) => c,
                    Err(e) => {
                        warn(
                            "APPLY",
                            &format!("Failed to reload config with no-cache: {}", e),
                        );
                        config
                    }
                }
            } else {
                config
            };

            if current_force {
                warn("APPLY", "APPLYING WITH FORCE (Cache bypassed)");
            }

            apply::execute(&db, &final_config, current_force)?;
        }
        Commands::List { command } => {
            list::execute(command, &mut db, &config)?;
        }
        Commands::Compile => {
            compile::execute(&config)?;
        }
    }
    Ok(())
}
