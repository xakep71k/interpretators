#[derive(Debug, Clone)]
pub enum CalcResult {
    FLOAT(f32),
    INTEGER(i32),
}

impl std::fmt::Display for CalcResult {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalcResult::FLOAT(v) => write!(fmt, "{}", v),
            CalcResult::INTEGER(v) => write!(fmt, "{}", v),
        }
    }
}
