extern crate rulis;
extern crate env_logger;

use std::io::{self, Write};
use std::env;

fn flush() {
    io::stdout().flush().expect("failed to flush");
}

fn main() {
    env::set_var("RUST_LOG", env::var("LOG").unwrap_or("info".into()));
    env_logger::init().unwrap();

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
