use anyhow::Result;
use k_lib::config::Cookbook;
use k_lib::logger;

pub fn run(config: &Cookbook) -> Result<()> {
    let stats = logger::stats(config)?;

    println!("Log Statistics:");
    println!("  Total files: {}", stats.total_files);
    println!("  Total size:  {}", logger::format_size(stats.total_size));

    if let Some(oldest) = &stats.oldest_file {
        println!("  Oldest:      {}", oldest);
    }
    if let Some(newest) = &stats.newest_file {
        println!("  Newest:      {}", newest);
    }

    Ok(())
}
