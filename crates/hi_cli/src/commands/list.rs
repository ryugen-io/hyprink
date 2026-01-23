use crate::args::ListCommands;
use crate::logging::{error, info};
use anyhow::Result;
use colored::*;
use hi_core::config::Config;
use hi_core::db::Store;

pub fn execute(command: Option<ListCommands>, db: &mut Store, _config: &Config) -> Result<()> {
    match command {
        Some(ListCommands::Clear) => {
            let count = db.list().len();
            if count == 0 {
                info("STORE", "store is already empty");
            } else {
                db.clear();
                db.save()?;
                info("STORE", &format!("removed {} templates", count));
            }
        }
        Some(ListCommands::Enable { name }) => {
            if db.set_ignored(&name, false)? {
                db.save()?;
                info("STORE", &format!("enabled template '{}'", name));
            } else {
                error("STORE", &format!("template '{}' not found", name));
            }
        }
        Some(ListCommands::Disable { name }) => {
            if db.set_ignored(&name, true)? {
                db.save()?;
                info("STORE", &format!("disabled template '{}'", name));
            } else {
                error("STORE", &format!("template '{}' not found", name));
            }
        }
        None => {
            list_store(db);
        }
    }
    Ok(())
}

fn list_store(db: &Store) {
    println!("{}", "\nStored Templates:\n".bold().underline());

    let templates = db.list();
    if templates.is_empty() {
        info("STORE", "No templates installed");
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
