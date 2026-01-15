use crate::args::ListCommands;
use crate::logging::{log, log_msg};
use anyhow::Result;
use colored::*;
use hi_core::config::Config;
use hi_core::db::Store;

pub fn execute(command: Option<ListCommands>, db: &mut Store, config: &Config) -> Result<()> {
    match command {
        Some(ListCommands::Clear) => {
            let count = db.list().len();
            if count == 0 {
                log_msg(config, "store_empty", "store is already empty");
            } else {
                db.clear();
                db.save()?;
                log_msg(
                    config,
                    "store_clear_ok",
                    &format!("removed {} templates", count),
                );
            }
        }
        Some(ListCommands::Enable { name }) => {
            if db.set_ignored(&name, false)? {
                db.save()?;
                log_msg(config, "store_ok", &format!("enabled template '{}'", name));
            } else {
                log_msg(
                    config,
                    "store_fail",
                    &format!("template '{}' not found", name),
                );
            }
        }
        Some(ListCommands::Disable { name }) => {
            if db.set_ignored(&name, true)? {
                db.save()?;
                log_msg(config, "store_ok", &format!("disabled template '{}'", name));
            } else {
                log_msg(
                    config,
                    "store_fail",
                    &format!("template '{}' not found", name),
                );
            }
        }
        None => {
            list_store(db, config);
        }
    }
    Ok(())
}

fn list_store(db: &Store, config: &Config) {
    println!("{}", "\nStored Templates:\n".bold().underline());

    let templates = db.list();
    if templates.is_empty() {
        log(config, "store_empty");
        return;
    }

    for tpl in templates {
        println!(
            "  {} {}\n    {}\n    {}",
            tpl.manifest.name.blue().bold(),
            format!("v{}", tpl.manifest.version).green(),
            tpl.manifest.description.italic(),
            format!("by {}", tpl.manifest.authors.join(", ")).dimmed()
        );
        if tpl.manifest.ignored {
            println!("    {}", "[DISABLED]".red().bold());
        }
        println!();
    }
}
