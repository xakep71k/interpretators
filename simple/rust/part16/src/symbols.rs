use crate::ast::Param;
use crate::var_type::VarType;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Symbol {
    BuiltIn { name: String },
    Var { name: String, kind: VarType },
    Procedure { name: String, params: Vec<Param> },
}

impl Symbol {
    fn name(&self) -> String {
        match self {
            Symbol::BuiltIn { name } => name.clone(),
            Symbol::Var { name, kind: _ } => name.clone(),
            Symbol::Procedure { name, params: _ } => name.clone(),
        }
    }
}
impl std::fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let line = match self {
            Symbol::BuiltIn { name } => format!("<BuiltIn(name='{}')>", name),
            Symbol::Var { name, kind } => format!("<Var(name='{}', type='{}')>", name, kind),
            Symbol::Procedure { name, params } => {
                format!(
                    "<Procedural(name='{}', parameters=[{}])>",
                    name,
                    params
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        };
        write!(fmt, "{}", line)
    }
}

pub struct ScopedSymbolTable {
    table: HashMap<String, Symbol>,
    scope_name: String,
    scope_level: usize,
    enclosing_scope: Option<Box<ScopedSymbolTable>>,
    debug_scope: bool,
}

impl std::fmt::Display for ScopedSymbolTable {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.scope_name == "None" {
            write!(fmt, "None")?;
            return Ok(());
        }
        let header = "SCOPE (SCOPED SYMBOL TABLE)";
        let mut output = format!(
            "\n{}\n{}\n",
            header,
            vec!['='; header.len()].iter().collect::<String>()
        );
        vec![
            format!("Scope name: {}\n", self.scope_name),
            format!("Scope level: {}\n", self.scope_level),
            format!(
                "Enclosing scope: {}\n",
                self.enclosing_scope
                    .as_ref()
                    .map(|x| x.scope_name.clone())
                    .unwrap_or("None".to_string())
            ),
        ]
        .into_iter()
        .for_each(|line| output.push_str(&line));
        output.push('\n');
        let header = "Scope (Scoped symbol table) contents";
        output.push_str(&header);
        output.push('\n');
        output.push_str(&vec!['-'; header.len()].iter().collect::<String>());
        output.push('\n');
        let mut key_values: Vec<String> = self
            .table
            .iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect();
        key_values.sort();
        output = key_values.iter().fold(output, |mut output, line| {
            output.push_str(&format!("{}\n", line));
            output
        });
        write!(fmt, "{}", output)?;
        Ok(())
    }
}

impl ScopedSymbolTable {
    pub fn new(scope_name: String, scope_level: usize, debug_scope: bool) -> ScopedSymbolTable {
        let mut new = ScopedSymbolTable {
            table: HashMap::new(),
            scope_level,
            scope_name,
            enclosing_scope: None,
            debug_scope,
        };
        if scope_level == 1 {
            vec![VarType::INTEGER, VarType::REAL].iter().for_each(|t| {
                new.insert(Symbol::BuiltIn { name: t.name() });
            });
        }
        new
    }

    pub fn enclosing_scope(&mut self) -> ScopedSymbolTable {
        let enclosing_scope = std::mem::replace(&mut self.enclosing_scope, None);
        *enclosing_scope.unwrap()
    }

    pub fn set_enclosing_scope(&mut self, enclosing_scope: ScopedSymbolTable) {
        self.enclosing_scope = Some(Box::new(enclosing_scope));
    }

    pub fn scope_level(&self) -> usize {
        self.scope_level
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.log(format!("Insert: {}", symbol.name()));
        self.table.insert(symbol.name(), symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<Symbol> {
        self.log(format!(
            "Lookup: {} (Scope name: {})",
            name, self.scope_name
        ));
        if let Some(symbol) = self.table.get(name) {
            Some(symbol.clone())
        } else if self.enclosing_scope.as_ref().unwrap().scope_level != 0 {
            self.enclosing_scope.as_ref().unwrap().lookup(name)
        } else {
            None
        }
    }

    pub fn lookup_current_only(&self, name: &str) -> Option<Symbol> {
        self.log(format!(
            "Lookup: {} (Scope name: {})",
            name, self.scope_name
        ));
        if let Some(symbol) = self.table.get(name) {
            Some(symbol.clone())
        } else {
            None
        }
    }

    fn log(&self, line: String) {
        if self.debug_scope {
            println!("{}", line);
        }
    }
}
