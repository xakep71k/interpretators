use crate::ast::{Param, AST};
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::token;
use crate::var_type::VarType;

pub struct Parser {
    lexer: Lexer,
    current_token: token::Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Result<Parser, Error> {
        let mut parser = Parser {
            current_token: token::Token {
                kind: token::Kind::EOF,
                column: 0,
                lineno: 0,
            },
            lexer,
        };
        parser.current_token = parser.lexer.next_token()?;
        Ok(parser)
    }

    fn eat(&mut self, kind: token::Kind) -> Result<(), Error> {
        // compare the current token type with the passed token
        // type and if they match then "eat" the current token
        // and assign the next token to the self.current_token,
        // otherwise raise an exception.
        if std::mem::discriminant(&self.current_token.kind) == std::mem::discriminant(&kind) {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(Error::UNEXPECTED_TOKEN(self.current_token.clone()))
        }
    }
    fn program(&mut self) -> Result<AST, Error> {
        // program : compound_statement DOT
        self.eat(token::Kind::PROGRAM)?;
        let program_name = match self.variable()? {
            AST::Var { id, .. } => id,
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

    fn block(&mut self) -> Result<AST, Error> {
        // block : declarations compound_statement
        let declaration_nodes = self.declarations()?;
        let compound_statement_node = self.compound_statement()?;
        let node = AST::Block {
            declaration_nodes,
            compound_nodes: Box::new(compound_statement_node),
        };
        Ok(node)
    }

    fn declarations(&mut self) -> Result<Vec<AST>, Error> {
        // declarations : VAR (variable_declaration SEMI)+
        //             | (PROCEDURE ID SEMI block SEMI)*
        //             | empty
        let mut declarations: Vec<AST> = Vec::new();

        if self.current_token.kind == token::Kind::VAR {
            self.eat(token::Kind::VAR)?;
            while let token::Kind::ID(_) = self.current_token.kind {
                let mut var_decl = self.variable_declaration()?;
                declarations.append(&mut var_decl);
                self.eat(token::Kind::SEMI)?;
            }
        }

        while let token::Kind::PROCEDURE = self.current_token.kind {
            let proc_decl = self.procedure_declaration()?;
            declarations.push(proc_decl);
        }
        Ok(declarations)
    }

    fn formal_parameters(&mut self) -> Result<Vec<Param>, Error> {
        // formal_parameters : ID (COMMA ID)* COLON type_spec
        let mut ids: Vec<String> = Vec::new();
        loop {
            let id = match self.current_token.kind.clone() {
                token::Kind::ID(id) => id,
                _ => String::new(),
            };
            self.eat(token::Kind::ID(id.clone()))?;
            ids.push(id);
            match self.current_token.kind.clone() {
                kind @ token::Kind::COMMA => {
                    self.eat(kind)?;
                    continue;
                }
                _ => break,
            };
        }
        self.eat(token::Kind::COLON)?;
        let ttype = self.type_spec()?;
        let result = ids
            .into_iter()
            .map(|id| Param {
                id,
                ttype: ttype.clone(),
            })
            .collect();
        Ok(result)
    }

    fn formal_parameter_list(&mut self) -> Result<Vec<Param>, Error> {
        // formal_parameter_list : formal_parameters
        //                           | formal_parameters SEMI formal_parameter_list
        if let token::Kind::ID(_) = self.current_token.kind.clone() {
        } else {
            return Ok(Vec::new());
        }

        let mut params: Vec<Param> = Vec::new();
        loop {
            params.append(&mut self.formal_parameters()?);
            match self.current_token.kind {
                token::Kind::SEMI => self.eat(token::Kind::SEMI)?,
                _ => break,
            };
        }
        Ok(params)
    }

    fn variable_declaration(&mut self) -> Result<Vec<AST>, Error> {
        // variable_declaration : ID (COMMA ID)* COLON type_spec
        let mut var_ids: Vec<String> = Vec::new();
        if let token::Kind::ID(id) = self.current_token.kind.clone() {
            var_ids.push(id);
        }
        self.eat(self.current_token.kind.clone())?;

        loop {
            match self.current_token.kind.clone() {
                comma @ token::Kind::COMMA => {
                    self.eat(comma)?;
                    if let token::Kind::ID(id) = self.current_token.kind.clone() {
                        var_ids.push(id);
                    }
                    self.eat(self.current_token.kind.clone())?;
                }
                _ => break,
            }
        }

        self.eat(token::Kind::COLON)?;
        let var_type = self.type_spec()?;
        Ok(var_ids
            .into_iter()
            .map(|id| AST::VarDecl {
                id,
                var_type: var_type.clone(),
                token: self.current_token.clone(),
            })
            .collect())
    }

    fn procedure_declaration(&mut self) -> Result<AST, Error> {
        // PROCEDURE ID (LPAREN formal_parameter_list RPAREN)? SEMI block SEMI
        self.eat(token::Kind::PROCEDURE)?;
        let id = match self.current_token.kind.clone() {
            token::Kind::ID(id) => id,
            _ => String::new(),
        };
        self.eat(token::Kind::ID(String::new()))?;
        let mut params = Vec::new();

        if let token::Kind::LPAREN = self.current_token.kind {
            self.eat(token::Kind::LPAREN)?;
            params = self.formal_parameter_list()?;
            self.eat(token::Kind::RPAREN)?;
        }

        self.eat(token::Kind::SEMI)?;
        let proc_decl = AST::ProcedureDecl {
            id,
            params,
            block_node: Box::new(self.block()?),
        };
        self.eat(token::Kind::SEMI)?;
        Ok(proc_decl)
    }

    fn type_spec(&mut self) -> Result<VarType, Error> {
        let var_type = match self.current_token.kind.clone() {
            token::Kind::TYPE(var_type) => var_type,
            _ => VarType::INTEGER,
        };
        self.eat(token::Kind::TYPE(var_type.clone()))?;
        Ok(var_type)
    }

    fn compound_statement(&mut self) -> Result<AST, Error> {
        // compound_statement: BEGIN statement_list END
        self.eat(token::Kind::BEGIN)?;
        let children = self.statement_list()?;
        self.eat(token::Kind::END)?;
        Ok(AST::Compound { children })
    }

    fn statement_list(&mut self) -> Result<Vec<AST>, Error> {
        /*
        statement : compound_statement
                 | proccall_statement
                 | assignment_statement
                 | empty
        */
        let node = self.statement()?;
        let mut results = vec![node];

        while let token::Kind::SEMI = self.current_token.kind {
            self.eat(token::Kind::SEMI)?;
            results.push(self.statement()?);
        }

        Ok(results)
    }

    fn statement(&mut self) -> Result<AST, Error> {
        // statement : compound_statement
        //           | assignment_statement
        //           | empty
        let token = self.current_token.clone();
        let node;
        if let token::Kind::BEGIN = token.kind {
            node = self.compound_statement()?;
        } else if std::mem::discriminant(&token.kind)
            == std::mem::discriminant(&token::Kind::ID(String::new()))
            && self.lexer.current_char().unwrap_or('0') == '('
        {
            node = self.proccall_statement()?;
        } else if let token::Kind::ID(id) = token.kind {
            node = self.assignment_statement(id)?;
        } else {
            node = AST::NoOp;
        }
        Ok(node)
    }

    fn proccall_statement(&mut self) -> Result<AST, Error> {
        // proccall_statement : ID LPAREN (expr (COMMA expr)*)? RPAREN
        let token = self.current_token.clone();
        let id = match self.current_token.kind.clone() {
            token::Kind::ID(id) => id,
            _ => String::new(),
        };
        self.eat(token::Kind::ID(String::new()))?;
        self.eat(token::Kind::LPAREN)?;
        let mut ast_params = Vec::new();
        loop {
            let node = self.expr()?;
            ast_params.push(node);
            match self.current_token.kind.clone() {
                kind @ token::Kind::COMMA => self.eat(kind)?,
                _ => break,
            };
        }

        self.eat(token::Kind::RPAREN)?;
        Ok(AST::ProcedureCall {
            id,
            ast_params,
            params: Vec::new(),
            token,
            body: None,
        })
    }

    fn assignment_statement(&mut self, left_id: String) -> Result<AST, Error> {
        // assignment_statement : variable ASSIGN expr
        let left = Box::new(self.variable()?);
        self.eat(token::Kind::ASSIGN)?;
        let right = Box::new(self.expr()?);
        Ok(AST::Assign {
            left_id,
            left,
            right,
        })
    }

    fn variable(&mut self) -> Result<AST, Error> {
        // variable : ID
        let id_token = self.current_token.clone();
        let id = match self.current_token.kind.clone() {
            token::Kind::ID(id) => id,
            _ => String::new(),
        };
        self.eat(token::Kind::ID(id.clone()))?;
        Ok(AST::Var {
            id,
            token: id_token,
        })
    }

    fn expr(&mut self) -> Result<AST, Error> {
        // expr : term ((PLUS | MINUS) term)*
        let mut left_node = self.term()?;

        loop {
            let token = self.current_token.clone();
            match token.kind.clone() {
                kind @ (token::Kind::PLUS | token::Kind::MINUS) => {
                    self.eat(kind)?;
                }
                _ => break,
            }
            left_node = AST::BinOp {
                left: Box::new(left_node),
                op: token.kind,
                right: Box::new(self.term()?),
            };
        }
        Ok(left_node)
    }

    fn term(&mut self) -> Result<AST, Error> {
        // term : factor ((MUL | INTEGER_DIV | FLOAT_DIV) factor)*
        let mut left_node = self.factor()?;

        loop {
            let token = self.current_token.clone();
            match token.kind.clone() {
                kind @ (token::Kind::MUL | token::Kind::INTEGER_DIV | token::Kind::FLOAT_DIV) => {
                    self.eat(kind)?;
                }
                _ => {
                    break;
                }
            };
            left_node = AST::BinOp {
                left: Box::new(left_node),
                op: token.kind,
                right: Box::new(self.factor()?),
            };
        }
        Ok(left_node)
    }

    fn factor(&mut self) -> Result<AST, Error> {
        // factor : PLUS factor
        //           | MINUS factor
        //           | INTEGER
        //           | LPAREN expr RPAREN
        //           | variable
        let kind = self.current_token.kind.clone();
        match kind {
            token::Kind::PLUS | token::Kind::MINUS => {
                self.eat(kind.clone())?;
                Ok(AST::UnaryOp {
                    op: kind,
                    expr: Box::new(self.factor()?),
                })
            }
            token::Kind::INTEGER_CONST(value) => {
                self.eat(kind)?;
                Ok(AST::NumInteger { value })
            }
            token::Kind::REAL_CONST(value) => {
                self.eat(kind)?;
                Ok(AST::NumReal { value })
            }
            token::Kind::LPAREN => {
                self.eat(kind)?;
                let node = self.expr()?;
                self.eat(token::Kind::RPAREN)?;
                Ok(node)
            }
            token::Kind::ID(_) => self.variable(),
            _ => Err(Error::UNEXPECTED_TOKEN(self.current_token.clone())),
        }
    }

    pub fn parse(mut self) -> Result<AST, Error> {
        /*
        program : PROGRAM variable SEMI block DOT
        block : declarations compound_statement
        declarations : (VAR (variable_declaration SEMI)+)? procedure_declaration*
        variable_declaration : ID (COMMA ID)* COLON type_spec
        procedure_declaration :
             PROCEDURE ID (LPAREN formal_parameter_list RPAREN)? SEMI block SEMI
        formal_params_list : formal_parameters
                           | formal_parameters SEMI formal_parameter_list
        formal_parameters : ID (COMMA ID)* COLON type_spec
        type_spec : INTEGER | REAL
        compound_statement : BEGIN statement_list END
        statement_list : statement
                       | statement SEMI statement_list
        statement : compound_statement
                  | proccall_statement
                  | assignment_statement
                  | empty
        proccall_statement : ID LPAREN (expr (COMMA expr)*)? RPAREN
        assignment_statement : variable ASSIGN expr
        empty :
        expr : term ((PLUS | MINUS) term)*
        term : factor ((MUL | INTEGER_DIV | FLOAT_DIV) factor)*
        factor : PLUS factor
               | MINUS factor
               | INTEGER_CONST
               | REAL_CONST
               | LPAREN expr RPAREN
               | variable
        variable: ID
        */
        let node = self.program()?;
        if std::mem::discriminant(&self.current_token.kind)
            != std::mem::discriminant(&token::Kind::EOF)
        {
            Err(Error::UNEXPECTED_TOKEN(self.current_token.clone()))
        } else {
            Ok(node)
        }
    }
}
