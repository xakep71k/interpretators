#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    INTEGER(i32),
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    ID(String),
    ASSIGN,
    BEGIN,
    END,
    SEMI,
    DOT,
    EOF,
}
