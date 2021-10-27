use crate::errors::Error;
use crate::parser::AST;
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

    pub fn print_symbol_table(&self) {
        println!("{}", self.current_scope);
    }

    pub fn visit_node(&mut self, node: AST) -> Result<(), Error> {
        match node {
            AST::Block {
                declaration_nodes,
                compound_nodes,
            } => {
                for declaration in declaration_nodes {
                    self.visit_node(declaration)?;
                }
                self.visit_node(*compound_nodes)?;
            }
            AST::Program { name: _, block } => {
                if self.debug_scope {
                    println!("ENTER scope: global");
                }
                let prev_scope = std::mem::replace(
                    &mut self.current_scope,
                    ScopedSymbolTable::new("global".to_string(), 1, self.debug_scope),
                );
                self.current_scope.set_enclosing_scope(prev_scope);

                self.visit_node(*block)?;

                if self.debug_scope {
                    println!("{}", self.current_scope);
                }
                self.current_scope = self.current_scope.enclosing_scope();
                if self.debug_scope {
                    println!("LEAVE scope: global");
                }
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
                });
                if self.debug_scope {
                    println!("ENTER scope: {}", id);
                }
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
                    };
                    self.current_scope.insert(var_symbol);
                });

                self.visit_node(*block_node)?;

                if self.debug_scope {
                    println!("{}", self.current_scope);
                }
                self.current_scope = self.current_scope.enclosing_scope();
                if self.debug_scope {
                    println!("LEAVE scope: {}", id);
                }
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
                self.visit_node(*right)?;
                self.visit_node(*left)?;
            }
            AST::BinOp { left, right, op: _ } => {
                self.visit_node(*right)?;
                self.visit_node(*left)?;
            }
            AST::VarDecl {
                id,
                var_type,
                token,
            } => {
                // use to see same output as origial python implementation
                self.current_scope.lookup(&var_type.name());
                if let Some(_) = self.current_scope.lookup_current_only(&id) {
                    return Err(Error::DUPLICATE_ID(token));
                }
                self.current_scope.insert(Symbol::Var {
                    name: id,
                    kind: var_type,
                });
            }
            AST::Var { id, token } => {
                let sym = self.current_scope.lookup(&id);
                if sym.is_none() {
                    return Err(Error::ID_NOT_FOUND(token));
                }
            }
        }
        Ok(())
    }
}
