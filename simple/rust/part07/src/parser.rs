use crate::lexer::Lexer;
use crate::token::{Token, Type};

#[derive(Debug)]
pub enum AST {
    Num {
        token: Token,
    },
    BinOp {
        left: Box<AST>,
        right: Box<AST>,
        op: Token,
    },
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    err: &'static str,
}

impl Parser {
    pub fn new(line: &str) -> Result<Parser, String> {
        let mut parser = Parser {
            current_token: Token::new(Type::EOF, 0),
            err: "Invalid syntax",
            lexer: Lexer::new(line)?,
        };
        parser.current_token = parser.lexer.next_token()?;
        Ok(parser)
    }

    pub fn parse(mut self) -> Result<AST, String> {
        self.expr()
    }
    fn eat(&mut self, kind: Type) -> Result<(), String> {
        if self.current_token.kind() == kind {
            self.current_token = self.lexer.next_token()?;
            return Ok(());
        }
        Err(self.err.to_string())
    }

    fn factor(&mut self) -> Result<AST, String> {
        let token = self.current_token;
        if token.kind() == Type::INTEGER {
            self.eat(Type::INTEGER)?;
            return Ok(AST::Num { token });
        } else if token.kind() == Type::LPAREN {
            self.eat(Type::LPAREN)?;
            let node = self.expr()?;
            self.eat(Type::RPAREN)?;
            return Ok(node);
        }
        Err(self.err.to_string())
    }

    fn term(&mut self) -> Result<AST, String> {
        let mut left_node = self.factor()?;

        while self.current_token.kind() == Type::MUL || self.current_token.kind() == Type::DIV {
            let token = self.current_token;
            if token.kind() == Type::MUL {
                self.eat(Type::MUL)?;
            } else if token.kind() == Type::DIV {
                self.eat(Type::DIV)?;
            }

            left_node = AST::BinOp {
                left: Box::new(left_node),
                op: token,
                right: Box::new(self.factor()?),
            };
        }
        Ok(left_node)
    }

    fn expr(&mut self) -> Result<AST, String> {
        let mut left_node = self.term()?;

        while self.current_token.kind() == Type::PLUS || self.current_token.kind() == Type::MINUS {
            let token = self.current_token;
            if token.kind() == Type::PLUS {
                self.eat(Type::PLUS)?;
            } else if token.kind() == Type::MINUS {
                self.eat(Type::MINUS)?;
            }
            left_node = AST::BinOp {
                left: Box::new(left_node),
                op: token,
                right: Box::new(self.term()?),
            };
        }
        Ok(left_node)
    }
}
