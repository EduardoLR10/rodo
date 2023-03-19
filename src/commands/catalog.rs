use crate::commands::parser;
extern crate walkdir;
use walkdir::WalkDir;
use std::fs::File;
use std::io::{ self, BufRead};
use std::str;

pub fn catalog_path(directory_path: &str) -> Result<Vec<String>, ()> {
    let mut todos = Vec::new();
    for file_path in WalkDir::new(directory_path).into_iter().filter_map(|file| file.ok()) {
	if file_path.metadata().unwrap().is_file() {
	    let file = File::open(file_path.path()).unwrap();
	    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| l.expect("Could not parse line")).collect();
	    for line in lines {
		match parser::todo()(line.as_str().as_bytes()) {
		    Ok(x) => {
			let s = match str::from_utf8(x.1) {
			    Ok(v) => v.to_string(),
			    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
			};
			todos.push(s);
		    },
		    Err(_e) => ()
		}
	    }
	    println!("Parsed TODOs from file {:?}", file_path.path().display());
	}
    }
    Ok(todos)
}
