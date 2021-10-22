use crate::parser::{Parser, AST};
use crate::token::Type;

pub fn interpret(parser: Parser) -> Result<i32, String> {
    let tree = parser.parse()?;
    Ok(visit_node(tree))
}

fn visit_node(node: AST) -> i32 {
    match node {
        AST::Num { token } => token.value(),
        AST::BinOp { left, right, op } => match op.kind() {
            Type::MUL => visit_node(*left) * visit_node(*right),
            Type::DIV => visit_node(*left) / visit_node(*right),
            Type::PLUS => visit_node(*left) + visit_node(*right),
            Type::MINUS => visit_node(*left) - visit_node(*right),
            any => panic!("impossible kind {:?}", any),
        },
    }
}
