extern crate walkdir;
use std::fs::File;
use std::io::{self, Read};
use std::str;
use walkdir::{DirEntry, WalkDir};

use super::parser::{parse_file, Todo};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn list_path_todos(directory_path: &str) -> Vec<Todo> {
    let mut todos = Vec::new();

    // recursively enumerate files
    for file_path in WalkDir::new(directory_path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|file| file.ok())
    {
        let file = File::open(file_path.path()).unwrap();
        let mut file_buffer = io::BufReader::new(file);
        let mut content_buffer = String::new();

        // file doesn't contain valid string (binary file): skip
        file_buffer.read_to_string(&mut content_buffer).ok();

        // look for todos in file
        let mut file_todos = parse_file(content_buffer.as_str());

        todos.append(&mut file_todos)
    }
    todos
}
