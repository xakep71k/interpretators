#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Type {
    EOF,
    PLUS,
    INTEGER,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    kind: Type,
    integer: i32,
}

impl Token {
    pub fn new(kind: Type) -> Token {
        Token {
            kind: kind,
            integer: 0,
        }
    }

    pub fn kind(&self) -> Type {
        self.kind
    }

    pub fn value(&self) -> i32 {
        self.integer
    }

    pub fn from_char(ch: char) -> Option<Token> {
        if ch.is_ascii_digit() {
            return Some(Token {
                kind: Type::INTEGER,
                integer: ch.to_digit(10).unwrap() as i32,
            });
        }
        if ch == '+' {
            return Some(Token {
                kind: Type::PLUS,
                integer: 0,
            });
        }
        None
    }
}
