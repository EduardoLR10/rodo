mod cli;
use cli::Cli;
use cli::RodoCommands;
use clap::{Parser};
mod commands;
use commands::catalog;

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        RodoCommands::Catalog { filepath } => {
	    let todos = catalog::catalog_path(filepath.as_str()).unwrap();
	    println!("{:?}", todos)		
        }
    }
}
