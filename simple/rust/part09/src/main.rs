use interpreter::Interpreteter;
use parser::Parser;

mod interpreter;
mod lexer;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <source file>", args[0]);
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(args[1].clone()).unwrap();
    let parser = match Parser::new(&content) {
        Ok(parser) => parser,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let interpreter = Interpreteter::new();
    match interpreter.interpret(parser) {
        Err(err) => eprintln!("{}", err),
        Ok(_) => {}
    }
}
