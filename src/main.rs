#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let stdin = io::stdin();
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();
        // input.pop();

        println!("{}: command not found", input.trim());
    }
}
