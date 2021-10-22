#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    #[allow(non_camel_case_types)]
    INTEGER_CONST(i32),
    #[allow(non_camel_case_types)]
    REAL_CONST(f32),
    PLUS,
    MINUS,
    MUL,
    COMMA,
    COLON,
    LPAREN,
    RPAREN,
    ID(String),
    ASSIGN,
    BEGIN,
    PROGRAM,
    VAR,
    #[allow(non_camel_case_types)]
    INTEGER_DIV,
    #[allow(non_camel_case_types)]
    FLOAT_DIV,
    TYPE(VarType),
    END,
    SEMI,
    DOT,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    INTEGER,
    REAL,
}
