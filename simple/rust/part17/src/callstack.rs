use crate::calc::CalcResult;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ARType {
    PROGRAM,
}

#[derive(Debug, Clone)]
pub struct ActiveRecord {
    name: String,
    kind: ARType,
    nesting_level: usize,
    members: HashMap<String, CalcResult>,
}

pub struct CallStack {
    stack: Vec<ActiveRecord>,
}

impl std::fmt::Display for ARType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)?;
        Ok(())
    }
}

impl CallStack {
    pub fn new() -> CallStack {
        CallStack { stack: Vec::new() }
    }

    pub fn push(&mut self, ar: ActiveRecord) {
        self.stack.push(ar);
    }

    pub fn pop(&mut self) -> Option<ActiveRecord> {
        self.stack.pop()
    }

    pub fn top(&mut self) -> &mut ActiveRecord {
        let last = self.stack.len() - 1;
        &mut self.stack[last]
    }
}

impl std::fmt::Display for ActiveRecord {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}: {} {}", self.nesting_level, self.kind, self.name)?;
        let mut lines = self
            .members
            .iter()
            .map(|(a, b)| format!("{:>4}:{: >20}", a, b.to_string()))
            .collect::<Vec<String>>();
        lines.sort();
        if !lines.is_empty() {
            writeln!(fmt)?;
        }
        write!(fmt, "{}", lines.join("\n"))?;
        Ok(())
    }
}

impl ActiveRecord {
    pub fn new(name: String, kind: ARType, nesting_level: usize) -> ActiveRecord {
        ActiveRecord {
            name,
            kind,
            nesting_level,
            members: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, res: CalcResult) {
        self.members.insert(id, res);
    }

    pub fn get(&self, id: &String) -> Option<&CalcResult> {
        self.members.get(id)
    }
}

impl std::fmt::Display for CallStack {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lines = self
            .stack
            .iter()
            .clone()
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        writeln!(fmt, "CALL STACK")?;
        writeln!(fmt, "{}", lines.join("\n"))?;
        Ok(())
    }
}
