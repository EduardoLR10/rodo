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
    for file_path in WalkDir::new(directory_path)
        .into_iter()
        // TODO: let the user decide to skip hidden dirs in the config file
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|file| file.ok()) {

            if file_path.metadata().unwrap().is_file() {
	              let file = File::open(file_path.path()).unwrap();
	              let file_buffer = io::BufReader::new(file);

                for line in file_buffer.lines() {
                    match line.as_deref() {
                        Ok("") | Err(_) => (),
                        Ok(source_code_line) => {
                            match parse_line(source_code_line) {
                                Ok(todo_line) => todos.push(todo_line.to_string()),
                                Err(_) => ()
                            }
                        }
                    }
                }
            }
        }
    Ok(todos)
}

fn parse_line(line: &str) -> Result<&str, ()> {

    parser::todo()(line.as_bytes())
        .map_err(|_| ())
        .map(|(_,y)|
             str::from_utf8(y)
             .map_err(|_| ())
        )?
}


#[test]
fn test_list_path() {
    //Criar um /tmp
}
