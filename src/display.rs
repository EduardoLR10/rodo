use colored::Colorize;

use crate::commands::parser::Todo;

impl Todo {
    pub fn display(todo: Todo) -> String {
        format!("{} {}", todo.tag.bold().blue().underline(), todo.text)
    }
}

pub fn display_todos(todos: Vec<Todo>) {
    for todo in todos {
        println!("{}", Todo::display(todo));
    }
}
