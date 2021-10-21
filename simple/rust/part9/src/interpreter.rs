use crate::parser::{Parser, AST};
use crate::token;
use std::collections::HashMap;

pub struct Interpreteter {
    global_scope: HashMap<String, i32>,
}

impl Interpreteter {
    pub fn new() -> Interpreteter {
        let global_scope = HashMap::new();
        Interpreteter { global_scope }
    }

    pub fn interpret(mut self, parser: Parser) -> Result<(), String> {
        let tree = parser.parse()?;
        println!("{:?}", tree);
        self.visit_node(tree)?;
        println!("GLOBAL SCOPE: {:?}", self.global_scope);
        Ok(())
    }

    fn visit_node(&mut self, node: AST) -> Result<Option<i32>, String> {
        match node {
            AST::Num { value } => Ok(Some(value)),
            AST::BinOp { left, right, op } => match op {
                token::Kind::MUL => {
                    let res = self.visit_node(*left)?.unwrap() * self.visit_node(*right)?.unwrap();
                    Ok(Some(res))
                }
                token::Kind::DIV => {
                    let res = self.visit_node(*left)?.unwrap() / self.visit_node(*right)?.unwrap();
                    Ok(Some(res))
                }
                token::Kind::PLUS => {
                    let res = self.visit_node(*left)?.unwrap() + self.visit_node(*right)?.unwrap();
                    Ok(Some(res))
                }
                token::Kind::MINUS => {
                    let res = self.visit_node(*left)?.unwrap() - self.visit_node(*right)?.unwrap();
                    Ok(Some(res))
                }
                any => panic!("impossible kind {:?}", any),
            },
            AST::UnaryOp { op, expr } => match op {
                token::Kind::PLUS => self.visit_node(*expr),
                token::Kind::MINUS => Ok(Some(-self.visit_node(*expr)?.unwrap())),
                any => panic!("impossible kind {:?}", any),
            },
            AST::Compound { children } => {
                for child in children {
                    let _res = self.visit_node(*child);
                }
                Ok(None)
            }
            AST::Assign { id, right } => {
                let res = self.visit_node(*right)?;
                self.global_scope.insert(id, res.unwrap());
                Ok(None)
            }
            AST::Var { id } => {
                if let Some(value) = self.global_scope.get(&id) {
                    Ok(Some(*value))
                } else {
                    Err(format!("varible '{}' not declared", id))
                }
            }
            AST::NoOp => Ok(None),
        }
    }
}
