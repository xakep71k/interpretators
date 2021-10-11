use crate::token::{Token, Type};

pub struct Interpretator {
    line: Vec<char>,
    index: usize,
    current_token: Token,
    err: String,
}

impl Interpretator {
    pub fn new(line: &str) -> Interpretator {
        Interpretator {
            index: 0,
            line: line.chars().collect(),
            current_token: Token::new(Type::EOF),
            err: "Error parsing input".to_string(),
        }
    }

    pub fn expr(&mut self) -> Result<i32, String> {
        self.current_token = self.next_token()?;
        let left = self.current_token;
        self.eat(Type::INTEGER)?;
        self.eat(Type::PLUS)?;
        let right = self.current_token;
        Ok(left.value() + right.value())
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        if self.index >= self.line.len() {
            return Ok(Token::new(Type::EOF));
        }

        let current_char = self.line[self.index];
        self.index += 1;
        if let Some(t) = Token::from_char(current_char) {
            Ok(t)
        } else {
            Err(self.err.clone())
        }
    }

    fn eat(&mut self, kind: Type) -> Result<(), String> {
        if self.current_token.kind() == kind {
            self.current_token = self.next_token()?;
            return Ok(());
        }
        Err(self.err.clone())
    }
}
