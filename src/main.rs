mod commands;
use commands::RodoCommand;
use commands::Cli;
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next().expect("something went wrong");

    let mut chosen_command = None;
    
    while let Some(arg) = args.next() {
        match RodoCommand::from_string(arg) {
	    RodoCommand::Catalog => chosen_command = Some(RodoCommand::Catalog)
        }
    }
    match chosen_command {
	Some(c) =>  eprintln!("{:?}", c),
	None => RodoCommand::full_usage()
    }	
}
