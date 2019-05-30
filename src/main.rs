extern crate chrono;

use chrono::{Date, Utc};
use std::io;
use std::io::prelude::*;

struct TodoItem {
    title: String,
    description: String,
    done: bool,
}

impl TodoItem {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str("TodoItem {");
        out.push_str(&format!("  title: {},", self.title));
        out.push_str(&format!("  description: {},", self.description));
        out.push_str(&format!("  done: {},", self.done));
        out.push_str("}");

        out
    }
}

struct TodoItemBuilder {
    title: Option<String>,
    description: Option<String>,
    done: Option<bool>,
}

impl TodoItemBuilder {
    pub fn new() -> Self {
        TodoItemBuilder {
            title: None,
            description: None,
            done: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn done(mut self, done: bool) -> Self {
        self.done = Some(done);
        self
    }

    pub fn build(self) -> TodoItem {
        TodoItem {
            title: match self.title {
                Some(t) => t.clone(),
                None => String::new(),
            },
            description: match self.description {
                Some(d) => d.clone(),
                None => String::new(),
            },
            done: match self.done {
                Some(d) => d,
                None => false,
            },
        }
    }
}

struct TodoSet {
    date: Date<Utc>,
    list: Vec<TodoItem>,
}

fn handle(input: String) {
    match input {
        _ => println!("No command matched for input. Please input help to see available command."),
    }
}

fn main() {
    let todo = TodoItemBuilder::new()
        .title("Todo Sample".to_owned())
        .description("this is sample todo.".to_owned())
        .build();
    println!("{}", todo.to_string());
    //     let stdin = io::stdin();
    //     loop {
    //         print!("wtd> ");
    //         io::stdout().flush().unwrap();
    //         let mut input = String::new();
    //         match stdin.lock().read_line(&mut input) {
    //             Ok(_) => handle(input),
    //             Err(error) => println!("error: {}", error),
    //         }
    //     }
}
