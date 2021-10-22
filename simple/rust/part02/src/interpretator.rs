use crate::token::{Token, Type};

pub struct Interpretator {
    line: Vec<char>,
    index: usize,
    current_token: Token,
    err: &'static str,
}

impl Interpretator {
    pub fn new(line: &str) -> Interpretator {
        Interpretator {
            index: 0,
            line: line.chars().collect(),
            current_token: Token::new(Type::EOF, 0),
            err: "Error parsing input",
        }
    }

    pub fn expr(&mut self) -> Result<i32, String> {
        self.current_token = self.next_token()?;
        let left = self.current_token;
        self.eat(Type::INTEGER)?;
        let op = self.current_token;
        if self.current_token.kind() == Type::PLUS {
            self.eat(Type::PLUS)?;
        } else {
            self.eat(Type::MINUS)?;
        }
        let right = self.current_token;
        if op.kind() == Type::PLUS {
            Ok(left.value() + right.value())
        } else {
            Ok(left.value() - right.value())
        }
    }

    fn integer(&mut self, current_char: char) -> i32 {
        let mut result = current_char as i32 - '0' as i32;
        while self.index < self.line.len() && self.line[self.index].is_ascii_digit() {
            result = result * 10 + (self.line[self.index] as i32 - '0' as i32);
            self.index += 1;
        }
        result
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        while self.index < self.line.len() {
            let current_char = self.line[self.index];
            self.index += 1;
            if current_char.is_ascii_whitespace() {
                continue;
            }

            if current_char.is_ascii_digit() {
                return Ok(Token::new(Type::INTEGER, self.integer(current_char)));
            }

            let value = current_char as i32;
            if current_char == '+' {
                return Ok(Token::new(Type::PLUS, value));
            }

            if current_char == '-' {
                return Ok(Token::new(Type::MINUS, value));
            }

            return Err(self.err.to_string());
        }
        return Ok(Token::new(Type::EOF, 0));
    }

    fn eat(&mut self, kind: Type) -> Result<(), String> {
        if self.current_token.kind() == kind {
            self.current_token = self.next_token()?;
            return Ok(());
        }
        Err(self.err.to_string())
    }
}
