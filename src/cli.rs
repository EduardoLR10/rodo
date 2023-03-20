use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: RodoCommands,
}

#[derive(Subcommand)]
pub enum RodoCommands {
    /// Catalog all the TODOs in a given folder
    Catalog { opt_filepath: Option<String> },
    /// List all the TODOs in a given folder
    List { opt_filepath: Option<String> },
    
}
