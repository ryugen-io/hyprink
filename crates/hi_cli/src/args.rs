use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "hyprink",
    version,
    about = "hyprink - System-wide theming manager"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable debug mode with verbose logging in a separate terminal
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add .tpl templates or .pkg packages to the store
    Add { path: PathBuf },
    /// Pack .tpl templates from a directory into a .pkg package
    Pack {
        /// Directory containing .tpl files
        input: PathBuf,
        /// Output .pkg file (optional, defaults to <dirname>.pkg)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Apply all templates from store to the system
    Apply {
        /// Persistently toggle force mode (always overwrite)
        #[arg(long)]
        toggle_force: bool,
        /// Force overwrite for this run only
        #[arg(long)]
        force: bool,
    },
    /// List stored templates
    List {
        #[command(subcommand)]
        command: Option<ListCommands>,
    },
    /// Compile config into binary cache for faster startup
    Compile,
    /// Internal command to watch logs via socket (Hidden)
    #[command(hide = true)]
    InternalWatch { socket_path: PathBuf },
}

#[derive(Subcommand, Debug)]
pub enum ListCommands {
    /// Remove all templates from the store
    Clear,
    /// Enable a template (remove ignored status)
    Enable { name: String },
    /// Disable a template (set ignored status)
    Disable { name: String },
}
