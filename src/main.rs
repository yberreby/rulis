extern crate rulis;

use std::io::{self, Write};

fn flush() {
    io::stdout().flush().expect("failed to flush");
}

fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        flush();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nReceived EOF, exiting");
                return;
            }
            Ok(_) => handle_input(&input),
            Err(e) => {
                println!("Failed to read input: {}", e);
                return;
            }
        }

        input.clear();
    }
}

fn handle_input(input: &str) {
    match rulis::eval(input) {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err),
    }
}
