use crate::token::{Token, Type};

pub struct Lexer {
    line: Vec<char>,
    pos: usize,
    err: &'static str,
}

impl Lexer {
    pub fn new(line: &str) -> Result<Lexer, String> {
        let lex = Lexer {
            pos: 0,
            line: line.chars().collect(),
            err: "Invalid character",
        };
        Ok(lex)
    }

    fn integer(&mut self, current_char: char) -> i32 {
        let mut result = current_char as i32 - '0' as i32;
        while self.pos < self.line.len() && self.line[self.pos].is_ascii_digit() {
            result = result * 10 + (self.line[self.pos] as i32 - '0' as i32);
            self.pos += 1;
        }
        result
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        while self.pos < self.line.len() {
            let current_char = self.line[self.pos];
            self.pos += 1;
            if current_char.is_ascii_whitespace() {
                continue;
            }

            if current_char.is_ascii_digit() {
                return Ok(Token::new(Type::INTEGER, self.integer(current_char)));
            }

            let value = current_char as i32;
            let kind = match current_char {
                '*' => Type::MUL,
                '-' => Type::MINUS,
                '+' => Type::PLUS,
                '/' => Type::DIV,
                '(' => Type::LPAREN,
                ')' => Type::RPAREN,
                _ => return Err(self.err.to_string()),
            };
            return Ok(Token::new(kind, value));
        }
        return Ok(Token::new(Type::EOF, 0));
    }
}
