use crate::lexer::Lexer;
use crate::token;

#[derive(Debug)]
pub enum AST {
    Program {
        name: String,
        block: Box<AST>,
    },
    Block {
        declaration_nodes: Vec<AST>,
        compound_nodes: Box<AST>,
    },
    VarDecl {
        id: String,
        var_type: token::VarType,
    },
    NumInteger {
        value: i32,
    },
    NumReal {
        value: f32,
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
        children: Vec<AST>,
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
            err: "Invalid syntax: unexpected character",
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
        // panic!();
        Err(format!("{}: {:?}", self.err, self.current_token))
    }
    fn program(&mut self) -> Result<AST, String> {
        // program : compound_statement DOT
        self.eat(token::Kind::PROGRAM)?;
        let program_name = match self.variable()? {
            AST::Var { id } => id,
            _ => panic!("impossible"),
        };
        self.eat(token::Kind::SEMI)?;
        let block_node = self.block()?;
        let program_node = AST::Program {
            name: program_name,
            block: Box::new(block_node),
        };
        self.eat(token::Kind::DOT)?;
        Ok(program_node)
    }

    fn block(&mut self) -> Result<AST, String> {
        // block : declarations compound_statement
        let declaration_nodes = self.declarations()?;
        let compound_statement_node = self.compound_statement()?;
        let node = AST::Block {
            declaration_nodes: declaration_nodes,
            compound_nodes: Box::new(compound_statement_node),
        };
        Ok(node)
    }

    fn declarations(&mut self) -> Result<Vec<AST>, String> {
        // declarations : VAR (variable_declaration SEMI)+
        //             | empty
        let mut declarations: Vec<AST> = Vec::new();
        if self.current_token == token::Kind::VAR {
            self.eat(token::Kind::VAR)?;
            loop {
                match self.current_token {
                    token::Kind::ID(_) => {
                        let mut var_decl = self.variable_declaration()?;
                        declarations.append(&mut var_decl);
                        self.eat(token::Kind::SEMI)?;
                    }
                    _ => break,
                }
            }
        }
        Ok(declarations)
    }

    fn variable_declaration(&mut self) -> Result<Vec<AST>, String> {
        // variable_declaration : ID (COMMA ID)* COLON type_spec
        let mut var_ids: Vec<String> = Vec::new();
        match self.current_token.clone() {
            token::Kind::ID(id) => {
                var_ids.push(id);
                self.eat(self.current_token.clone())?;
            }
            _ => return Err(self.err.to_string()),
        }

        loop {
            match self.current_token.clone() {
                comma @ token::Kind::COMMA => {
                    self.eat(comma)?;
                    match self.current_token.clone() {
                        token::Kind::ID(id) => var_ids.push(id),
                        _ => return Err(self.err.to_string()),
                    }
                    self.eat(self.current_token.clone())?;
                }
                _ => break,
            }
        }

        self.eat(token::Kind::COLON)?;

        let var_type = match self.current_token.clone() {
            token::Kind::TYPE(var_type) => var_type,
            _ => return Err(self.err.to_string()),
        };
        self.eat(token::Kind::TYPE(var_type.clone()))?;
        Ok(var_ids
            .into_iter()
            .map(|id| AST::VarDecl {
                id: id,
                var_type: var_type.clone(),
            })
            .collect())
    }

    fn compound_statement(&mut self) -> Result<AST, String> {
        // compound_statement: BEGIN statement_list END
        self.eat(token::Kind::BEGIN)?;
        let children = self.statement_list()?;
        self.eat(token::Kind::END)?;
        Ok(AST::Compound { children })
    }

    fn statement_list(&mut self) -> Result<Vec<AST>, String> {
        // statement_list : statement
        //                | statement SEMI statement_list
        let node = self.statement()?;
        let mut results = vec![node];

        loop {
            match self.current_token {
                token::Kind::SEMI => {
                    self.eat(token::Kind::SEMI)?;
                    results.push(self.statement()?);
                }
                _ => break,
            };
        }

        Ok(results)
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

    fn variable(&mut self) -> Result<AST, String> {
        // variable : ID
        match self.current_token.clone() {
            token::Kind::ID(id) => {
                self.eat(token::Kind::ID(id.clone()))?;
                Ok(AST::Var { id })
            }
            _ => Err(self.err.to_string()),
        }
    }

    fn expr(&mut self) -> Result<AST, String> {
        // expr : term ((PLUS | MINUS) term)*
        let mut left_node = self.term()?;

        loop {
            let token = self.current_token.clone();
            match token {
                token::Kind::PLUS | token::Kind::MINUS => {
                    self.eat(token.clone())?;
                }
                _ => break,
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
        // term : factor ((MUL | INTEGER_DIV | FLOAT_DIV) factor)*
        let mut left_node = self.factor()?;

        loop {
            let token = self.current_token.clone();
            match token {
                token::Kind::MUL | token::Kind::INTEGER_DIV | token::Kind::FLOAT_DIV => {
                    self.eat(token.clone())?;
                }
                _ => {
                    break;
                }
            };
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
            token::Kind::INTEGER_CONST(value) => {
                self.eat(token)?;
                Ok(AST::NumInteger { value })
            }
            token::Kind::REAL_CONST(value) => {
                self.eat(token)?;
                Ok(AST::NumReal { value })
            }
            token::Kind::LPAREN => {
                self.eat(token)?;
                let node = self.expr()?;
                self.eat(token::Kind::RPAREN)?;
                Ok(node)
            }
            token::Kind::ID(_) => self.variable(),
            _ => Err(self.err.to_string()),
        }
    }

    pub fn parse(mut self) -> Result<AST, String> {
        // program : PROGRAM variable SEMI block DOT
        // block : declarations compound_statement
        // declarations : VAR (variable_declaration SEMI)+
        //              | empty
        // variable_declaration : ID (COMMA ID)* COLON type_spec
        // type_spec : INTEGER | REAL
        // compound_statement : BEGIN statement_list END
        // statement_list : statement
        //                | statement SEMI statement_list
        // statement : compound_statement
        //           | assignment_statement
        //           | empty
        // assignment_statement : variable ASSIGN expr
        // empty :
        // expr : term ((PLUS | MINUS) term)*
        // term : factor ((MUL | INTEGER_DIV | FLOAT_DIV) factor)*
        // factor : PLUS factor
        //        | MINUS factor
        //        | INTEGER_CONST
        //        | REAL_CONST
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
