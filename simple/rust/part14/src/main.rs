use interpreter::Interpreteter;
use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;

mod interpreter;
mod lexer;
mod parser;
mod semantic;
mod symbols;
mod token;
mod var_type;

fn execute() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} <source file>", args[0]);
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(args[1].clone()).unwrap();
    let lexer = Lexer::new(&content)?;
    let parser = Parser::new(lexer)?;
    let tree = parser.parse()?;
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.visit_node(tree.clone())?;
    semantic_analyzer.print_symbol_table();
    let interpreter = Interpreteter::new();
    interpreter.interpret(tree)?;
    Ok(())
}

fn main() {
    match execute() {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        _ => {}
    }
}
