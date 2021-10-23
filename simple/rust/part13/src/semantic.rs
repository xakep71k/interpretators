use crate::parser::AST;
use crate::symbols::Symbol;
use crate::symbols::SymbolTable;

pub struct SemanticAnalyzer {
    symtab: SymbolTable,
    undef_err: &'static str,
}

impl SemanticAnalyzer {
    pub fn new() -> SemanticAnalyzer {
        SemanticAnalyzer {
            symtab: SymbolTable::new(),
            undef_err: "undefined symbol",
        }
    }

    pub fn print_symbol_table(&self) {
        println!("{}", self.symtab);
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
            AST::Program { name: _, block } => self.visit_node(*block)?,
            AST::Compound { children } => {
                for child in children {
                    self.visit_node(child)?;
                }
            }
            AST::NumInteger { value: _ }
            | AST::NumReal { value: _ }
            | AST::NoOp
            | AST::ProcedureDecl {
                id: _,
                declaration_nodes: _,
                compound_nodes: _,
            }
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
                self.symtab.lookup(&var_type.name()); // to make output the same as original python script
                if let Some(_) = self.symtab.lookup(&id) {
                    return Err(format!("Error: Duplicate identifier '{}' found", id));
                }
                self.symtab.insert(Symbol::Var {
                    name: id,
                    kind: var_type,
                });
            }
            AST::Var { id } => {
                let sym = self.symtab.lookup(&id);
                if sym.is_none() {
                    return Err(format!("{} '{}'", self.undef_err, id));
                }
            }
        }
        Ok(())
    }
}
