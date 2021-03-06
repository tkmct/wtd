extern crate chrono;

use chrono::{Date, Local};

#[derive(Debug)]
pub struct TodoItem {
    title: String,
    description: String,
    done: bool,
}

impl TodoItem {
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str("TodoItem {\n");
        out.push_str(&format!("  title: {},\n", self.title));
        out.push_str(&format!("  description: {},\n", self.description));
        out.push_str(&format!("  done: {},\n", self.done));
        out.push_str("}\n");

        out
    }
}

pub struct TodoItemBuilder {
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

#[derive(Debug)]
pub struct TodoSet {
    date: Date<Local>,
    pub list: Vec<TodoItem>,
}

impl TodoSet {
    pub fn new() -> Self {
        TodoSet {
            date: Local::today(),
            list: vec![],
        }
    }

    pub fn add_todo(&mut self, item: TodoItem) {
        self.list.push(item);
    }
}
