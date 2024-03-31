mod cli;
use cli::Cli;
use cli::RodoCommands;
use clap::{Parser};
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
        },
	RodoCommands::List { opt_filepath } => {
	    let filepath = opt_filepath.to_owned().unwrap_or(current_dir_str);
	    let todos = list::list_path(filepath.as_str()).unwrap();
	    display_todos(todos)
        }
    }
}

#[test]
fn test_list () {
    let cli = Cli::parse_from(vec!["", "list", "."]);
    assert_eq!(cli, Cli{ command: RodoCommands::List {opt_filepath: Some(String::from("."))}});

    let cli = Cli::parse_from(vec!["", "list"]);
    assert_eq!(cli, Cli{ command: RodoCommands::List {opt_filepath: None}});
}

#[test]
fn test_catalog () {
    let cli = Cli::parse_from(vec!["", "catalog", "."]);
    assert_eq!(cli, Cli{ command: RodoCommands::Catalog {opt_filepath: Some(String::from("."))}});
    let cli = Cli::parse_from(vec!["", "catalog"]);
    assert_eq!(cli, Cli{ command: RodoCommands::Catalog {opt_filepath: None}});
}
