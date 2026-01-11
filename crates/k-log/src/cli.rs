use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "k-log", version, about = "Kitchn Logging Tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Preset key from dictionary.toml (when not using subcommands)
    pub preset: Option<String>,

    /// Optional message override
    #[arg(trailing_var_arg = true)]
    pub msg: Option<Vec<String>>,

    /// Optional app name override
    #[arg(long)]
    pub app: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show log statistics
    Stats,

    /// Clean up old log files
    Cleanup {
        /// Max age in days (overrides config)
        #[arg(long)]
        max_age: Option<u32>,

        /// Max total size (e.g. "500M", "1G")
        #[arg(long)]
        max_size: Option<String>,

        /// Dry run - show what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,
    },
}
