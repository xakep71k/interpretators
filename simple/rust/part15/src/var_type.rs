#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    INTEGER,
    REAL,
}

impl std::fmt::Display for VarType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl VarType {
    pub fn name(&self) -> String {
        format!("{}", self)
    }
}
