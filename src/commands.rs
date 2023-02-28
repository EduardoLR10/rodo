use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum RodoCommand {
    Catalog    
}

pub type Description = String;
pub type UsageCase = String;

pub trait Cli {
    fn to_string(&self) -> String;
    fn from_string(from: String) -> Self;
    fn single_usage(&self) -> (UsageCase, Description);
    fn full_usage();    
}

impl Cli for RodoCommand {
    fn to_string(&self) -> String {
	match &self {
	    RodoCommand::Catalog => String::from("catalog")
	}
    }
    fn from_string(from: String) -> Self {
	match from.as_str() {
	    "catalog" => RodoCommand::Catalog,
		_other => todo!("Missing implementation of {_other}")
	}
    }
    fn single_usage(&self) -> (UsageCase, Description){
	match &self {
	    RodoCommand::Catalog => (String::from("catalog <path-to-folder>"), String::from("catalog all the TODOs in a given folder"))
	}
    }
    fn full_usage() {
	eprintln!("Usage: [COMMAND] [OPTIONS]");	
	eprintln!("Commands:");
	for command in RodoCommand::iter() {
	    let usage = Self::single_usage(&command);
            eprintln!("    {:?}        {:?}", usage.0, usage.1);
	}
    }    
}
