mod todos;

use std::io;
use std::io::prelude::*;
use todos::*;

fn handle_add_todo() -> TodoItem {
    TodoItemBuilder::new().build()
}

fn main() {
    let mut todo_set = TodoSet::new();
    let todo = TodoItemBuilder::new()
        .title("Todo Sample".to_owned())
        .description("this is sample todo.".to_owned())
        .build();
    todo_set.list.push(todo);
    let stdin = io::stdin();
    loop {
        print!("wtd> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "add" => {
                        let todo = handle_add_todo();
                        todo_set.add_todo(todo);
                    }
                    "list" => println!("{:#?}", todo_set),
                    _ => println!("Please enter command add or list."),
                };
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
