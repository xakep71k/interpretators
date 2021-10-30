use errors::Error;
use interpreter::Interpreteter;
use lexer::Lexer;
use opts::Opts;
use parser::Parser;
use semantic::SemanticAnalyzer;

mod ast;
mod calc;
mod callstack;
mod errors;
mod interpreter;
mod lexer;
mod opts;
mod parser;
mod semantic;
mod symbols;
mod token;
mod var_type;

fn execute() -> Result<(), Error> {
    let opts = Opts::from_args();
    let content = std::fs::read_to_string(opts.filename).unwrap();
    let lexer = Lexer::new(&content)?;
    let parser = Parser::new(lexer)?;
    let mut tree = parser.parse()?;
    let mut semantic_analyzer = SemanticAnalyzer::new(opts.debug_scope);
    semantic_analyzer.visit_node(&mut tree)?;
    let interpreter = Interpreteter::new(opts.debug_stack);
    interpreter.interpret(tree)?;
    Ok(())
}

fn main() {
    if let Err(err) = execute() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
