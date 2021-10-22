use crate::parser::{Parser, AST};
use crate::token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum CaclResult {
    FLOAT(f32),
    INTEGER(i32),
}

impl CaclResult {
    fn as_f32(&self) -> f32 {
        match self {
            CaclResult::FLOAT(value) => *value,
            CaclResult::INTEGER(value) => *value as f32,
        }
    }
}

fn at_least_one_float(a: CaclResult, b: CaclResult) -> bool {
    match a {
        CaclResult::FLOAT(_) => true,
        _ => match b {
            CaclResult::FLOAT(_) => true,
            _ => false,
        },
    }
}

fn arithmetic<T>(a: CaclResult, b: CaclResult, op: T) -> CaclResult
where
    T: Fn(f32, f32) -> f32,
{
    let result = op(a.as_f32(), b.as_f32());
    if at_least_one_float(a, b) {
        CaclResult::FLOAT(result)
    } else {
        CaclResult::INTEGER(result as i32)
    }
}

pub struct Interpreteter {
    global_scope: HashMap<String, CaclResult>,
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
        // println!("GLOBAL SCOPE: {:?}", self.global_scope);
        // for k, v in self.global_scope {
        //     println!("{} = {}", k, v);
        // }
        let mut result = Vec::new();
        self.global_scope.iter().for_each(|(k, v)| {
            result.push(format!("{} = {:?}", k, v));
        });
        result.sort();
        result.iter().for_each(|s| println!("{}", s));
        Ok(())
    }

    fn visit_node(&mut self, node: AST) -> Result<Option<CaclResult>, String> {
        match node {
            AST::Program { name: _, block } => return self.visit_node(*block),
            AST::Block {
                declaration_nodes,
                compound_nodes,
            } => {
                for declaration in declaration_nodes {
                    self.visit_node(declaration)?;
                }
                return self.visit_node(*compound_nodes);
            }
            AST::VarDecl { id: _, var_type: _ } => Ok(None),
            AST::NumInteger { value } => Ok(Some(CaclResult::INTEGER(value))),
            AST::NumReal { value } => Ok(Some(CaclResult::FLOAT(value))),
            AST::BinOp { left, right, op } => match op {
                token::Kind::MUL => Ok(Some(arithmetic(
                    self.visit_node(*left)?.unwrap(),
                    self.visit_node(*right)?.unwrap(),
                    |a, b| a * b,
                ))),
                token::Kind::FLOAT_DIV => Ok(Some(arithmetic(
                    self.visit_node(*left)?.unwrap(),
                    self.visit_node(*right)?.unwrap(),
                    |a, b| a / b,
                ))),
                token::Kind::INTEGER_DIV => Ok(Some(arithmetic(
                    self.visit_node(*left)?.unwrap(),
                    self.visit_node(*right)?.unwrap(),
                    |a, b| a / b,
                ))),
                token::Kind::PLUS => Ok(Some(arithmetic(
                    self.visit_node(*left)?.unwrap(),
                    self.visit_node(*right)?.unwrap(),
                    |a, b| a + b,
                ))),
                token::Kind::MINUS => Ok(Some(arithmetic(
                    self.visit_node(*left)?.unwrap(),
                    self.visit_node(*right)?.unwrap(),
                    |a, b| a - b,
                ))),
                any => panic!("impossible kind {:?}", any),
            },
            AST::UnaryOp { op, expr } => match op {
                token::Kind::PLUS => self.visit_node(*expr),
                token::Kind::MINUS => {
                    let res = match self.visit_node(*expr)?.unwrap() {
                        CaclResult::INTEGER(value) => CaclResult::INTEGER(-value),
                        CaclResult::FLOAT(value) => CaclResult::FLOAT(-value),
                    };
                    Ok(Some(res))
                }
                any => panic!("impossible kind {:?}", any),
            },
            AST::Compound { children } => {
                for child in children {
                    let _res = self.visit_node(child);
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
                    Ok(Some(value.clone()))
                } else {
                    Err(format!("varible '{}' not declared", id))
                }
            }
            AST::NoOp => Ok(None),
        }
    }
}
