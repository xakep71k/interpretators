use crate::var_type::VarType;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Symbol {
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
            Symbol::BuiltIn { name } => format!("<BuiltIn(name='{}')>", name),
            Symbol::Var { name, kind } => format!("<Var(name='{}', type='{}')>", name, kind),
        };
        write!(fmt, "{}", line)
    }
}

pub struct SymbolTable {
    table: HashMap<String, Symbol>,
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut lines: Vec<String> = self
            .table
            .iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect();
        lines.sort();
        let string = lines.iter().fold(
            format!(
                "\n{}\n{}\n",
                "Symbol table contents",
                vec!['_'; 21].iter().collect::<String>(),
            ),
            |mut result, line| {
                result.push_str(&format!("{}\n", line));
                result
            },
        );
        write!(fmt, "{}", string)?;
        Ok(())
    }
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut new = SymbolTable {
            table: HashMap::new(),
        };
        vec![VarType::INTEGER, VarType::REAL].iter().for_each(|t| {
            new.insert(Symbol::BuiltIn { name: t.name() });
        });
        new
    }

    pub fn insert(&mut self, symbol: Symbol) {
        println!("Insert: {}", symbol.name());
        self.table.insert(symbol.name(), symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        println!("Lookup: {}", name);
        if let Some(symbol) = self.table.get(name) {
            Some(symbol.clone())
        } else {
            None
        }
    }
}
