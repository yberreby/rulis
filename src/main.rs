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
        io::stdin().read_line(&mut input).unwrap();
        println!("{}", rulis::eval(&input));
        input.clear();
    }
}
