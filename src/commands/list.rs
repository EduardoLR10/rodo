use crate::commands::parser;
extern crate walkdir;
use walkdir::{DirEntry, WalkDir};
use std::fs::File;
use std::io::{ self, BufRead};
use std::str;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn list_path(directory_path: &str) -> Result<Vec<String>, ()> {
    let mut todos = Vec::new();
    for file_path in WalkDir::new(directory_path).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|file| file.ok()) {
        if file_path.metadata().unwrap().is_file() {
	          let file = File::open(file_path.path()).unwrap();
	          let mut file_buffer = io::BufReader::new(file);
	          let mut content_buffer = String::new();
	          loop {
	              match file_buffer.read_line(&mut content_buffer) {
		                Ok(0) => break,
		                Ok(_) => {
		                    content_buffer.pop(); // This is to remove the new line at the end
		                    match parser::todo()(content_buffer.as_str().as_bytes()) {
			                      Ok(x) => {
			                          match str::from_utf8(x.1) {
				                            Ok(v) => {
				                                todos.push(v.to_string());
				                            },
				                            Err(_e) => (),
			                          };
			                      },
			                      Err(_e) => ()
		                    }
		                    content_buffer.clear();
		                },
		                Err(_) => ()
	              }
	          }
	          // println!("Parsed TODOs from file {:?}", file_path.path().display());
        }
    }
    Ok(todos)
}
