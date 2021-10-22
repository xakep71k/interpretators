use interpretator::Interpretator;
use std::io::{self, stdout, Write};

mod interpretator;
mod lexer;
mod token;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input.clear();
        print!("calc> ");
        stdout().flush().expect("flush failed");
        stdin.read_line(&mut input).expect("read line failed");
        if input.len() == 0 {
            println!();
            return;
        }
        if input.chars().next() == Some('\n') {
            continue;
        }
        let mut interpreter = match Interpretator::new(&input) {
            Ok(interpreter) => interpreter,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };
        match interpreter.expr() {
            Err(err) => eprintln!("{}", err),
            Ok(result) => println!("{}", result),
        }
    }
}
