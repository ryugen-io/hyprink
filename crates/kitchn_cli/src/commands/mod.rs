pub mod bake;
pub mod cook;
pub mod pantry;
pub mod stock;
pub mod wrap;

use crate::args::Commands;
use crate::logging::log_msg;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use k_lib::config::Cookbook;
use k_lib::db::Pantry;
use k_lib::processor;

pub fn process_command(cmd: Commands) -> Result<()> {
    let dirs = ProjectDirs::from("", "", "kitchn").context("Could not determine project dirs")?;
    let data_dir = dirs.data_dir();
    let db_path = data_dir.join("pantry.db");
    let mut db = Pantry::load(&db_path)?;
    let config = Cookbook::load().context("Failed to load Kitchn cookbook")?;

    match cmd {
        Commands::Stock { path } => {
            let installed = stock::stock_pantry(&path, &mut db, &config)?;
            db.save()?;

            for pkg in installed {
                log_msg(
                    &config,
                    "cook_start",
                    &format!("simmering {}", pkg.meta.name),
                );
                let _ = processor::apply(&pkg, &config)?;
            }
        }
        Commands::Wrap { input, output } => {
            wrap::execute(input, output, &config)?;
        }
        Commands::Cook => {
            cook::execute(&db, &config)?;
        }
        Commands::Pantry { command } => {
            pantry::execute(command, &mut db, &config)?;
        }
        Commands::Bake => {
            bake::execute(&dirs, &config)?;
        }
        Commands::InternalWatch { .. } => {}
    }
    Ok(())
}
