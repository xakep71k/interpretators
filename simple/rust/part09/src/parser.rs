use crate::lexer::Lexer;
use crate::token;

#[derive(Debug)]
pub enum AST {
    Num {
        value: i32,
    },
    BinOp {
        left: Box<AST>,
        right: Box<AST>,
        op: token::Kind,
    },
    UnaryOp {
        op: token::Kind,
        expr: Box<AST>,
    },
    Compound {
        children: Vec<Box<AST>>,
    },
    Assign {
        id: String,
        right: Box<AST>,
    },
    Var {
        id: String,
    },
    NoOp,
}

pub struct Parser {
    lexer: Lexer,
    current_token: token::Kind,
    err: &'static str,
}

impl Parser {
    pub fn new(line: &str) -> Result<Parser, String> {
        let mut parser = Parser {
            current_token: token::Kind::EOF,
            err: "Invalid syntax",
            lexer: Lexer::new(line)?,
        };
        parser.current_token = parser.lexer.next_token()?;
        Ok(parser)
    }

    fn eat(&mut self, kind: token::Kind) -> Result<(), String> {
        // compare the current token type with the passed token
        // type and if they match then "eat" the current token
        // and assign the next token to the self.current_token,
        // otherwise raise an exception.
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&kind) {
            self.current_token = self.lexer.next_token()?;
            return Ok(());
        }
        Err(self.err.to_string())
    }
    fn program(&mut self) -> Result<AST, String> {
        // program : compound_statement DOT
        let node = self.compound_statement()?;
        self.eat(token::Kind::DOT)?;
        Ok(node)
    }

    fn compound_statement(&mut self) -> Result<AST, String> {
        // compound_statement: BEGIN statement_list END
        self.eat(token::Kind::BEGIN)?;
        let nodes = self.statement_list()?;
        self.eat(token::Kind::END)?;

        let mut children = Vec::new();
        for node in nodes {
            children.push(Box::new(node));
        }
        Ok(AST::Compound { children })
    }

    fn statement_list(&mut self) -> Result<Vec<AST>, String> {
        // statement_list : statement
        //                | statement SEMI statement_list
        let node = self.statement()?;
        let mut results = vec![node];

        while std::mem::discriminant(&self.current_token)
            == std::mem::discriminant(&token::Kind::SEMI)
        {
            self.eat(token::Kind::SEMI)?;
            results.push(self.statement()?);
        }

        if std::mem::discriminant(&self.current_token)
            == std::mem::discriminant(&token::Kind::ID(String::new()))
        {
            Err(self.err.to_string())
        } else {
            Ok(results)
        }
    }

    fn statement(&mut self) -> Result<AST, String> {
        // statement : compound_statement
        //           | assignment_statement
        //           | empty
        let token = self.current_token.clone();
        match token {
            token::Kind::BEGIN => return self.compound_statement(),
            token::Kind::ID(name) => return self.assignment_statement(name),
            _ => return Ok(AST::NoOp),
        }
    }

    fn assignment_statement(&mut self, id: String) -> Result<AST, String> {
        // assignment_statement : variable ASSIGN expr
        self.eat(token::Kind::ID(id.clone()))?;
        self.eat(token::Kind::ASSIGN)?;
        let right = Box::new(self.expr()?);
        Ok(AST::Assign { id, right })
    }

    fn variable(&mut self, id: String) -> Result<AST, String> {
        // variable : ID
        self.eat(token::Kind::ID(id.clone()))?;
        Ok(AST::Var { id })
    }

    fn expr(&mut self) -> Result<AST, String> {
        // expr : term ((PLUS | MINUS) term)*
        let mut left_node = self.term()?;

        while std::mem::discriminant(&self.current_token)
            == std::mem::discriminant(&token::Kind::PLUS)
            || std::mem::discriminant(&self.current_token)
                == std::mem::discriminant(&token::Kind::MINUS)
        {
            let token = self.current_token.clone();
            if std::mem::discriminant(&token) == std::mem::discriminant(&token::Kind::PLUS) {
                self.eat(token::Kind::PLUS)?;
            } else {
                self.eat(token::Kind::MINUS)?;
            }
            left_node = AST::BinOp {
                left: Box::new(left_node),
                op: token,
                right: Box::new(self.term()?),
            };
        }
        Ok(left_node)
    }

    fn term(&mut self) -> Result<AST, String> {
        // term : factor ((MUL | DIV) factor)*
        let mut left_node = self.factor()?;

        while std::mem::discriminant(&self.current_token)
            == std::mem::discriminant(&token::Kind::MUL)
            || std::mem::discriminant(&self.current_token)
                == std::mem::discriminant(&token::Kind::DIV)
        {
            let token = self.current_token.clone();
            if std::mem::discriminant(&token) == std::mem::discriminant(&token::Kind::MUL) {
                self.eat(token::Kind::MUL)?;
            } else {
                self.eat(token::Kind::DIV)?;
            }

            left_node = AST::BinOp {
                left: Box::new(left_node),
                op: token,
                right: Box::new(self.factor()?),
            };
        }
        Ok(left_node)
    }

    fn factor(&mut self) -> Result<AST, String> {
        // factor : PLUS factor
        //           | MINUS factor
        //           | INTEGER
        //           | LPAREN expr RPAREN
        //           | variable
        let token = self.current_token.clone();
        match token {
            token::Kind::PLUS | token::Kind::MINUS => {
                self.eat(token.clone())?;
                Ok(AST::UnaryOp {
                    op: token,
                    expr: Box::new(self.factor()?),
                })
            }
            token::Kind::INTEGER(value) => {
                self.eat(token)?;
                Ok(AST::Num { value })
            }
            token::Kind::LPAREN => {
                self.eat(token)?;
                let node = self.expr()?;
                self.eat(token::Kind::RPAREN)?;
                Ok(node)
            }
            token::Kind::ID(id) => self.variable(id),
            _ => Err(self.err.to_string()),
        }
    }

    pub fn parse(mut self) -> Result<AST, String> {
        // program : compound_statement DOT
        // compound_statement : BEGIN statement_list END
        // statement_list : statement
        //                | statement SEMI statement_list
        // statement : compound_statement
        //           | assignment_statement
        //           | empty
        // assignment_statement : variable ASSIGN expr
        // empty :
        // expr: term ((PLUS | MINUS) term)*
        // term: factor ((MUL | DIV) factor)*
        // factor : PLUS factor
        //        | MINUS factor
        //        | INTEGER
        //        | LPAREN expr RPAREN
        //        | variable
        // variable: ID
        let node = self.program()?;
        if std::mem::discriminant(&self.current_token) != std::mem::discriminant(&token::Kind::EOF)
        {
            Err(self.err.to_string())
        } else {
            Ok(node)
        }
    }
}
