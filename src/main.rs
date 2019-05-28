use std::io;
use std::io::prelude::*;

fn handle(input: String) {
    match input {
        _ => println!("No command matched for input. Please input help to see available command."),
    }
}

fn main() {
    let stdin = io::stdin();
    loop {
        print!("wtd> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(_) => handle(input),
            Err(error) => println!("error: {}", error),
        }
    }
}
