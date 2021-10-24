use crate::parser::AST;
use crate::symbols::ScopedSymbolTable;
use crate::symbols::Symbol;

pub struct SemanticAnalyzer {
    current_scope: ScopedSymbolTable,
    undef_err: &'static str,
}

impl SemanticAnalyzer {
    pub fn new() -> SemanticAnalyzer {
        SemanticAnalyzer {
            current_scope: ScopedSymbolTable::new("None".to_string(), 0),
            undef_err: "undefined symbol",
        }
    }

    pub fn print_symbol_table(&self) {
        println!("{}", self.current_scope);
    }

    pub fn visit_node(&mut self, node: AST) -> Result<(), String> {
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
                println!("ENTER scope: global");
                let prev_scope = std::mem::replace(
                    &mut self.current_scope,
                    ScopedSymbolTable::new("global".to_string(), 1),
                );
                self.current_scope.set_enclosing_scope(prev_scope);

                self.visit_node(*block)?;

                println!("{}", self.current_scope);
                self.current_scope = self.current_scope.enclosing_scope();
                println!("LEAVE scope: global");
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
                println!("ENTER scope: {}", id);
                let current_scope_level = self.current_scope.scope_level();
                let prev_scope = std::mem::replace(
                    &mut self.current_scope,
                    ScopedSymbolTable::new(id.clone(), current_scope_level + 1),
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

                println!("{}", self.current_scope);
                self.current_scope = self.current_scope.enclosing_scope();
                println!("LEAVE scope: {}", id);
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
                self.visit_node(*left)?;
                self.visit_node(*right)?;
            }
            AST::BinOp { left, right, op: _ } => {
                self.visit_node(*right)?;
                self.visit_node(*left)?;
            }
            AST::VarDecl { id, var_type } => {
                // use to see same output as origial python implementation
                self.current_scope.lookup(&var_type.name());
                if let Some(_) = self.current_scope.lookup_current_only(&id) {
                    return Err(format!("Error: Duplicate identifier '{}' found", id));
                }
                self.current_scope.insert(Symbol::Var {
                    name: id,
                    kind: var_type,
                });
            }
            AST::Var { id } => {
                let sym = self.current_scope.lookup(&id);
                if sym.is_none() {
                    return Err(format!("{} '{}'", self.undef_err, id));
                }
            }
        }
        Ok(())
    }
}
