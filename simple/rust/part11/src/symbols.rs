use crate::parser::AST;
use crate::var_type::VarType;
use std::collections::HashMap;

#[derive(Clone)]
enum Symbol {
    BuiltIn { name: String },
    Var { name: String, kind: VarType },
}

impl Symbol {
    fn name(&self) -> String {
        match self {
            Symbol::BuiltIn { name } => name.clone(),
            Symbol::Var { name, kind: _ } => name.clone(),
        }
    }
}
impl std::fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line = match self {
            Symbol::BuiltIn { name } => format!("{}", name),
            Symbol::Var { name, kind } => format!("<{}:{}>", name, kind),
        };
        write!(fmt, "{}", line)
    }
}

struct SymbolTable {
    table: HashMap<String, Symbol>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        let mut new = SymbolTable {
            table: HashMap::new(),
        };
        vec![VarType::INTEGER, VarType::REAL].iter().for_each(|t| {
            new.define(Symbol::BuiltIn { name: t.name() });
        });
        new
    }

    fn define(&mut self, symbol: Symbol) {
        println!("Define: {}", symbol);
        self.table.insert(symbol.name(), symbol);
    }

    fn lookup(&self, name: &str) -> Option<Symbol> {
        println!("Lookup: {}", name);
        if let Some(symbol) = self.table.get(name) {
            Some(symbol.clone())
        } else {
            None
        }
    }

    fn print_table(&self) {
        let mut lines: Vec<String> = self
            .table
            .iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect();
        lines.sort();
        println!("{}", "Symbol Table contents:");
        lines.iter().for_each(|line| println!("{}", line));
    }
}

pub struct SymbolTableBuilder {
    symtab: SymbolTable,
    undef_err: &'static str,
}

impl SymbolTableBuilder {
    pub fn new() -> SymbolTableBuilder {
        SymbolTableBuilder {
            symtab: SymbolTable::new(),
            undef_err: "undefined symbol",
        }
    }

    pub fn print_table(&self) {
        self.symtab.print_table();
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
            AST::BinOp { left, right, op: _ } => {
                self.visit_node(*left)?;
                self.visit_node(*right)?;
            }
            AST::NumInteger { value: _ } | AST::NumReal { value: _ } | AST::NoOp => {}
            AST::UnaryOp { op: _, expr } => self.visit_node(*expr)?,
            AST::Compound { children } => {
                for child in children {
                    self.visit_node(child)?;
                }
            }
            AST::VarDecl { id, var_type } => {
                self.symtab.define(Symbol::Var {
                    name: id,
                    kind: var_type,
                });
            }
            AST::Assign { id, right: _ } => {
                let sym = self.symtab.lookup(&id);
                if sym.is_none() {
                    return Err(format!("{} '{}'", self.undef_err, id));
                }
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

/*
class SymbolTableBuilder(NodeVisitor):
    def visit_VarDecl(self, node):
        type_name = node.type_node.value
        type_symbol = self.symtab.lookup(type_name)
        var_name = node.var_node.value
        var_symbol = VarSymbol(var_name, type_symbol)
        self.symtab.define(var_symbol)

    def visit_Assign(self, node):
        var_name = node.left.value
        var_symbol = self.symtab.lookup(var_name)
        if var_symbol is None:
            raise NameError(repr(var_name))

        self.visit(node.right)

    def visit_Var(self, node):
        var_name = node.value
        var_symbol = self.symtab.lookup(var_name)

        if var_symbol is None:
            raise NameError(repr(var_name))
*/
