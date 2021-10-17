use interpreter::interpret;
use parser::Parser;
use std::io::{self, stdout, Write};

mod interpreter;
mod lexer;
mod parser;
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
        let parser = match Parser::new(&input) {
            Ok(parser) => parser,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        match interpret(parser) {
            Err(err) => eprintln!("{}", err),
            Ok(result) => println!("{:?}", result),
        }
    }
}
