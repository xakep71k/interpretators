use crate::errors::Error;
use crate::token;
use crate::var_type::VarType;
use std::collections::HashMap;

pub struct Lexer {
    reserved_keywords: HashMap<&'static str, token::Kind>,
    pos: usize,
    line: Vec<char>,
    current_char: Option<char>,
    lineno: usize,
    column: usize,
}

impl Lexer {
    pub fn new(line: &str) -> Result<Lexer, Error> {
        let line: Vec<char> = line.chars().collect();
        let lex = Lexer {
            current_char: Some(line[0]),
            pos: 0,
            line,
            reserved_keywords: [
                ("PROGRAM", token::Kind::PROGRAM),
                ("VAR", token::Kind::VAR),
                ("DIV", token::Kind::INTEGER_DIV),
                ("INTEGER", token::Kind::TYPE(VarType::INTEGER)),
                ("REAL", token::Kind::TYPE(VarType::REAL)),
                ("BEGIN", token::Kind::BEGIN),
                ("END", token::Kind::END),
                ("PROCEDURE", token::Kind::PROCEDURE),
            ]
            .iter()
            .cloned()
            .collect(),
            lineno: 1,
            column: 0,
        };
        Ok(lex)
    }

    fn advance(&mut self) {
        if let Some('\n') = self.current_char {
            self.lineno += 1;
            self.column = 0;
        }
        self.pos += 1;
        if self.pos < self.line.len() {
            self.current_char = Some(self.line[self.pos]);
            self.column += 1;
        } else {
            self.current_char = None;
        }
    }

    fn peek(&self) -> Option<char> {
        let peek_pos = self.pos + 1;
        if peek_pos < self.line.len() {
            Some(self.line[peek_pos])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(current_char) = self.current_char {
            if !current_char.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while let Some(current_char) = self.current_char {
            if current_char == '}' {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn number(&mut self) -> token::Kind {
        let mut result = String::new();
        let mut is_real = false;
        while let Some(current_char) = self.current_char {
            if current_char == '.' {
                if is_real {
                    break;
                }
                is_real = true;
            } else if !current_char.is_ascii_digit() {
                break;
            }
            result.push(current_char);
            self.advance();
        }
        if is_real {
            token::Kind::REAL_CONST(result.parse().unwrap())
        } else {
            token::Kind::INTEGER_CONST(result.parse().unwrap())
        }
    }

    fn id(&mut self) -> Result<token::Kind, Error> {
        let mut result = String::new();
        while let Some(current_char) = self.current_char {
            if !current_char.is_ascii_alphanumeric() {
                break;
            }
            result.push(current_char);
            self.advance();
        }

        if let Some(kind) = self.reserved_keywords.get(&result.to_uppercase()[..]) {
            Ok(kind.clone())
        } else {
            Ok(token::Kind::ID(result))
        }
    }

    pub fn new_token(&self, kind: token::Kind) -> token::Token {
        token::Token {
            kind,
            lineno: self.lineno,
            column: self.column,
        }
    }

    pub fn next_token(&mut self) -> Result<token::Token, Error> {
        while let Some(current_char) = self.current_char {
            if current_char == '{' {
                self.advance();
                self.skip_comment();
                continue;
            }
            if current_char.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if current_char.is_ascii_alphabetic() {
                let id = self.id()?;
                return Ok(self.new_token(id));
            }

            if current_char.is_ascii_digit() {
                let num = self.number();
                return Ok(self.new_token(num));
            }

            if current_char == ':' && self.peek().is_some() && self.peek().unwrap() == '=' {
                self.advance();
                self.advance();
                return Ok(self.new_token(token::Kind::ASSIGN));
            }

            let kind = match current_char {
                ';' => token::Kind::SEMI,
                '*' => token::Kind::MUL,
                '-' => token::Kind::MINUS,
                ',' => token::Kind::COMMA,
                '+' => token::Kind::PLUS,
                '/' => token::Kind::FLOAT_DIV,
                ':' => token::Kind::COLON,
                '(' => token::Kind::LPAREN,
                ')' => token::Kind::RPAREN,
                '.' => token::Kind::DOT,
                current_char => return Err(Error::INVALID_CHARACTER(current_char)),
            };
            let token = self.new_token(kind);
            self.advance();
            return Ok(token);
        }
        Ok(self.new_token(token::Kind::EOF))
    }
}
