use anyhow::Result;
use k_lib::config::Cookbook;
use k_lib::logger::{self, CleanupOptions};

pub fn run(
    config: &Cookbook,
    max_age: Option<u32>,
    max_size: Option<String>,
    dry_run: bool,
) -> Result<()> {
    let options = CleanupOptions {
        max_age_days: max_age,
        max_total_size: max_size,
        dry_run,
    };

    let result = logger::cleanup(config, options)?;

    if dry_run {
        if result.would_delete.is_empty() {
            println!("Nothing to clean up.");
        } else {
            println!("Would delete {} files:", result.would_delete.len());
            for f in &result.would_delete {
                println!("  {}", f);
            }
            println!("Would free: {}", logger::format_size(result.would_free));
        }
    } else if result.deleted.is_empty() {
        println!("Nothing to clean up.");
    } else {
        println!("Deleted {} files:", result.deleted.len());
        for f in &result.deleted {
            println!("  {}", f);
        }
        println!("Freed: {}", logger::format_size(result.freed));
    }

    Ok(())
}
