#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let stdin = io::stdin();
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let _ = stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => std::process::exit(0),
            _ => println!("{}: command not found", input.trim()),
        };
    }
}
