use crate::lexer::Lexer;
use crate::token::{Token, Type};

pub struct Interpretator {
    lexer: Lexer,
    current_token: Token,
    err: &'static str,
}

impl Interpretator {
    pub fn new(line: &str) -> Result<Interpretator, String> {
        let lexer = Lexer::new(line)?;
        let mut interpretator = Interpretator {
            current_token: Token::new(Type::EOF, 0),
            err: "Invalid syntax",
            lexer: lexer,
        };
        interpretator.current_token = interpretator.lexer.next_token()?;
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

    fn factor(&mut self) -> Result<i32, String> {
        let token = self.current_token;
        if token.kind() == Type::INTEGER {
            self.eat(Type::INTEGER)?;
            return Ok(token.value());
        } else if token.kind() == Type::LPAREN {
            self.eat(Type::LPAREN)?;
            let result = self.expr();
            self.eat(Type::RPAREN)?;
            return result;
        }
        Err(self.err.to_string())
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

    fn eat(&mut self, kind: Type) -> Result<(), String> {
        if self.current_token.kind() == kind {
            self.current_token = self.lexer.next_token()?;
            return Ok(());
        }
        Err(self.err.to_string())
    }
}
