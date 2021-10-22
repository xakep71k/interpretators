use crate::token::{Token, Type};

pub struct Interpretator {
    line: Vec<char>,
    index: usize,
    current_token: Token,
    err: &'static str,
}

impl Interpretator {
    pub fn new(line: &str) -> Result<Interpretator, String> {
        let mut interpretator = Interpretator {
            index: 0,
            line: line.chars().collect(),
            current_token: Token::new(Type::EOF, 0),
            err: "Invalid syntax",
        };
        interpretator.current_token = interpretator.next_token()?;
        Ok(interpretator)
    }

    pub fn expr(&mut self) -> Result<i32, String> {
        let mut result = self.term()?;

        while self.current_token.kind() == Type::PLUS || self.current_token.kind() == Type::MINUS {
            let token = self.current_token;
            if token.kind() == Type::PLUS {
                self.eat(Type::PLUS)?;
                result = result + self.term()?;
            } else if token.kind() == Type::MINUS {
                self.eat(Type::MINUS)?;
                result = result - self.term()?;
            }
        }
        Ok(result)
    }

    fn integer(&mut self, current_char: char) -> i32 {
        let mut result = current_char as i32 - '0' as i32;
        while self.index < self.line.len() && self.line[self.index].is_ascii_digit() {
            result = result * 10 + (self.line[self.index] as i32 - '0' as i32);
            self.index += 1;
        }
        result
    }

    fn factor(&mut self) -> Result<i32, String> {
        let token = self.current_token;
        self.eat(Type::INTEGER)?;
        Ok(token.value())
    }

    fn term(&mut self) -> Result<i32, String> {
        let mut result = self.factor()?;

        while self.current_token.kind() == Type::MUL || self.current_token.kind() == Type::DIV {
            let token = self.current_token;
            if token.kind() == Type::MUL {
                self.eat(Type::MUL)?;
                result = result * self.factor()?;
            } else if token.kind() == Type::DIV {
                self.eat(Type::DIV)?;
                result = result / self.factor()?;
            }
        }
        Ok(result)
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
            let kind = match current_char {
                '*' => Type::MUL,
                '-' => Type::MINUS,
                '+' => Type::PLUS,
                '/' => Type::DIV,
                _ => return Err(self.err.to_string()),
            };
            return Ok(Token::new(kind, value));
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
