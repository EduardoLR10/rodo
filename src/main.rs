mod cli;
use clap::Parser;
use cli::Cli;
use cli::RodoCommands;
mod commands;
use commands::catalog;
use commands::list;
use std::env::current_dir;
mod display;
use display::display_todos;

fn main() {
    let cli = Cli::parse();
    let current_dir = current_dir().unwrap();
    let current_dir_str = current_dir.into_os_string().into_string().unwrap();
    match &cli.command {
        RodoCommands::Catalog { opt_filepath } => {
            let filepath = opt_filepath.to_owned().unwrap_or(current_dir_str);
            let _todos = catalog::catalog_path(filepath.as_str()).unwrap();
        }
        RodoCommands::List { opt_filepath } => {
            let filepath = opt_filepath.to_owned().unwrap_or(current_dir_str);
            let todos = list::list_path_todos(filepath.as_str());
            display_todos(todos)
        }
    }
}
