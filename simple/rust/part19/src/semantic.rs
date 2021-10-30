use crate::ast::AST;
use crate::errors::Error;
use crate::symbols::ScopedSymbolTable;
use crate::symbols::Symbol;

pub struct SemanticAnalyzer {
    current_scope: ScopedSymbolTable,
    debug_scope: bool,
}

impl SemanticAnalyzer {
    pub fn new(debug_scope: bool) -> SemanticAnalyzer {
        SemanticAnalyzer {
            current_scope: ScopedSymbolTable::new("None".to_string(), 0, debug_scope),
            debug_scope,
        }
    }

    pub fn log(&self, line: String) {
        if self.debug_scope {
            println!("{}", line);
        }
    }
    pub fn visit_node(&mut self, node: &mut AST) -> Result<(), Error> {
        match node {
            AST::Block {
                declaration_nodes,
                compound_nodes,
            } => {
                for declaration in declaration_nodes {
                    self.visit_node(declaration)?;
                }
                self.visit_node(compound_nodes)?;
            }
            AST::Program { name: _, block } => {
                self.log("ENTER scope: global".to_string());
                let prev_scope = std::mem::replace(
                    &mut self.current_scope,
                    ScopedSymbolTable::new("global".to_string(), 1, self.debug_scope),
                );
                self.current_scope.set_enclosing_scope(prev_scope);

                self.visit_node(block)?;

                self.log(format!("{}", self.current_scope));
                self.current_scope = self.current_scope.enclosing_scope();
                self.log("LEAVE scope: global".to_string());
            }
            AST::Compound { children } => {
                for child in children {
                    self.visit_node(child)?;
                }
            }
            AST::ProcedureDecl {
                id,
                params,
                block_node,
            } => {
                self.current_scope.insert(Symbol::Procedure {
                    name: id.clone(),
                    params: params.clone(),
                    block: *block_node.clone(),
                    scope_level: 0,
                });
                self.log(format!("ENTER scope: {}", id));
                let current_scope_level = self.current_scope.scope_level();
                let prev_scope = std::mem::replace(
                    &mut self.current_scope,
                    ScopedSymbolTable::new(id.clone(), current_scope_level + 1, self.debug_scope),
                );
                self.current_scope.set_enclosing_scope(prev_scope);

                params.iter().for_each(|param| {
                    // use to see same output as origial python implementation
                    self.current_scope.lookup(&param.ttype.to_string());
                    let var_symbol = Symbol::Var {
                        name: param.id.clone(),
                        kind: param.ttype.clone(),
                        scope_level: 0,
                    };
                    self.current_scope.insert(var_symbol);
                });

                self.visit_node(block_node)?;

                self.log(format!("{}", self.current_scope));
                self.current_scope = self.current_scope.enclosing_scope();
                // replace with new block
                self.current_scope.insert(Symbol::Procedure {
                    name: id.clone(),
                    params: params.clone(),
                    block: *block_node.clone(),
                    scope_level: 0,
                });
                self.log(format!("LEAVE scope: {}", id));
            }
            AST::NumInteger { value: _ }
            | AST::NumReal { value: _ }
            | AST::NoOp
            | AST::UnaryOp { op: _, expr: _ } => {}
            AST::Assign {
                left_id: _,
                left,
                right,
            } => {
                self.visit_node(right)?;
                self.visit_node(left)?;
            }
            AST::BinOp { left, right, op: _ } => {
                self.visit_node(right)?;
                self.visit_node(left)?;
            }
            AST::VarDecl {
                id,
                var_type,
                token,
            } => {
                // use to see same output as origial python implementation
                self.current_scope.lookup(&var_type.name());
                if self.current_scope.lookup_current_only(id).is_some() {
                    return Err(Error::DUPLICATE_ID(token.clone()));
                }
                self.current_scope.insert(Symbol::Var {
                    name: id.clone(),
                    kind: var_type.clone(),
                    scope_level: 0,
                });
            }
            AST::Var { id, token } => {
                let sym = self.current_scope.lookup(id);
                if sym.is_none() {
                    return Err(Error::ID_NOT_FOUND(token.clone()));
                }
            }
            AST::ProcedureCall {
                ast_params,
                params,
                ref mut body,
                scope_level,
                id,
                token,
            } => {
                for param in ast_params {
                    self.visit_node(param)?;
                }
                if let Some(s) = self.current_scope.lookup(id) {
                    *params = match s.clone() {
                        Symbol::Procedure { params, .. } => params,
                        _ => panic!("impossible"),
                    };
                    let block = match s.clone() {
                        Symbol::Procedure { block, .. } => block,
                        _ => panic!("impossible"),
                    };
                    *body = Some(Box::new(block));
                    *scope_level = s.scope_level();
                } else {
                    return Err(Error::ID_NOT_FOUND(token.clone()));
                }
            }
        }
        Ok(())
    }
}
