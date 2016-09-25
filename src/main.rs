extern crate rulis;
extern crate env_logger;
extern crate rustyline;

use std::env;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rulis::Interpreter;

fn main() {
    env::set_var("RUST_LOG", env::var("LOG").unwrap_or("info".into()));
    env_logger::init().unwrap();

    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();
    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);

                match interpreter.evaluate(input) {
                    Ok(res) => println!("{}", res),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Received EOF, exiting");
                return;
            }
            Err(e) => {
                println!("Failed to read input: {}", e);
                return;
            }
        }
    }
}
