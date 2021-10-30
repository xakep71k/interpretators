use crate::ast::AST;
use crate::calc::CalcResult;
use crate::callstack::{ARType, ActiveRecord, CallStack};
use crate::errors::Error;
use crate::token;

impl CalcResult {
    fn as_f32(&self) -> f32 {
        match self {
            CalcResult::FLOAT(value) => *value,
            CalcResult::INTEGER(value) => *value as f32,
        }
    }
}

fn at_least_one_float(a: CalcResult, b: CalcResult) -> bool {
    match a {
        CalcResult::FLOAT(_) => true,
        _ => matches!(b, CalcResult::FLOAT(_)),
    }
}

fn arithmetic<T>(a: CalcResult, b: CalcResult, op: T) -> CalcResult
where
    T: Fn(f32, f32) -> f32,
{
    let result = op(a.as_f32(), b.as_f32());
    if at_least_one_float(a, b) {
        CalcResult::FLOAT(result)
    } else {
        CalcResult::INTEGER(result as i32)
    }
}

pub struct Interpreteter {
    callstack: CallStack,
    debug_stack: bool,
}

impl Interpreteter {
    pub fn new(debug_stack: bool) -> Interpreteter {
        let callstack = CallStack::new();
        Interpreteter {
            callstack,
            debug_stack,
        }
    }

    fn log(&self, s: String) {
        if self.debug_stack {
            println!("{}", s);
        }
    }

    pub fn interpret(mut self, tree: AST) -> Result<(), Error> {
        //println!("*** Tree: ***\n{:?}", tree);
        self.visit_node(tree)?;
        Ok(())
    }

    fn visit_node(&mut self, node: AST) -> Result<Option<CalcResult>, Error> {
        match node {
            AST::Program { block, name } => {
                self.log(format!("ENTER: PROGRAM {}", name));
                let ar = ActiveRecord::new(name.clone(), ARType::PROGRAM, 1);
                self.callstack.push(ar);
                self.log(self.callstack.to_string());
                let res = self.visit_node(*block);
                self.log(format!("LEAVE: PROGRAM {}", name));
                self.log(self.callstack.to_string());
                self.callstack.pop();
                res
            }
            AST::Block {
                declaration_nodes,
                compound_nodes,
            } => {
                for declaration in declaration_nodes {
                    self.visit_node(declaration)?;
                }
                return self.visit_node(*compound_nodes);
            }
            AST::VarDecl { .. } => Ok(None),
            AST::NumInteger { value } => Ok(Some(CalcResult::INTEGER(value))),
            AST::NumReal { value } => Ok(Some(CalcResult::FLOAT(value))),
            AST::BinOp { left, right, op } => match op {
                token::Kind::MUL => Ok(Some(arithmetic(
                    self.visit_node(*left)?.unwrap(),
                    self.visit_node(*right)?.unwrap(),
                    |a, b| a * b,
                ))),
                token::Kind::FLOAT_DIV => {
                    let a = self.visit_node(*left)?.unwrap();
                    let b = self.visit_node(*right)?.unwrap();
                    let c = a.as_f32() / b.as_f32();
                    Ok(Some(CalcResult::FLOAT(c)))
                }
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
                        CalcResult::INTEGER(value) => CalcResult::INTEGER(-value),
                        CalcResult::FLOAT(value) => CalcResult::FLOAT(-value),
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
            AST::Assign { left_id, right, .. } => {
                let res = self.visit_node(*right)?;
                self.callstack.top().insert(left_id, res.unwrap());
                Ok(None)
            }
            AST::Var { id, token } => {
                if let Some(value) = self.callstack.top().get(&id) {
                    Ok(Some(value.clone()))
                } else {
                    Err(Error::ID_NOT_FOUND(token))
                }
            }
            AST::ProcedureCall {
                id,
                params,
                ast_params,
                body,
                ..
            } => {
                let mut ar = ActiveRecord::new(id.clone(), ARType::PROCEDURE, 2);
                for (formal, actual) in params.iter().zip(ast_params) {
                    let res = self.visit_node(actual)?;
                    ar.insert(formal.id.clone(), res.unwrap());
                }
                self.callstack.push(ar);
                self.log(format!("ENTER: PROCEDURE {}", id));
                self.log(format!("{}", self.callstack));

                self.visit_node(*body.unwrap())?;

                self.log(format!("LEAVE: PROCEDURE {}", id));
                self.log(format!("{}", self.callstack));
                self.callstack.pop();
                Ok(None)
            }
            AST::NoOp | AST::ProcedureDecl { .. } => Ok(None),
        }
    }
}
