#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Type {
    EOF,
    MUL,
    DIV,
    PLUS,
    MINUS,
    INTEGER,
    LPAREN,
    RPAREN,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    kind: Type,
    value: i32,
}

impl Token {
    pub fn new(kind: Type, value: i32) -> Token {
        Token {
            kind: kind,
            value: value,
        }
    }

    pub fn kind(&self) -> Type {
        self.kind
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
