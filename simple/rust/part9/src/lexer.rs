use crate::token;
use std::collections::HashMap;

pub struct Lexer {
    reserved_keywords: HashMap<&'static str, token::Kind>,
    pos: usize,
    line: Vec<char>,
    err: &'static str,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(line: &str) -> Result<Lexer, String> {
        let line: Vec<char> = line.chars().collect();
        let lex = Lexer {
            current_char: Some(line[0]),
            pos: 0,
            line: line,
            err: "Invalid character",
            reserved_keywords: [("BEGIN", token::Kind::BEGIN), ("END", token::Kind::END)]
                .iter()
                .cloned()
                .collect(),
        };
        Ok(lex)
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.line.len() {
            self.current_char = Some(self.line[self.pos]);
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

    fn integer(&mut self) -> i32 {
        let mut result: i32 = 0;
        while let Some(current_char) = self.current_char {
            if !current_char.is_ascii_digit() {
                break;
            }
            result = result * 10 + (current_char as i32 - '0' as i32);
            self.advance();
        }
        result
    }

    fn id(&mut self) -> Result<token::Kind, String> {
        let mut result = String::new();
        while let Some(current_char) = self.current_char {
            if !current_char.is_ascii_alphanumeric() {
                break;
            }
            result.push(current_char);
            self.advance();
        }

        if let Some(kind) = self.reserved_keywords.get(&result[..]) {
            Ok(kind.clone())
        } else {
            Ok(token::Kind::ID(result))
        }
    }

    pub fn next_token(&mut self) -> Result<token::Kind, String> {
        while let Some(current_char) = self.current_char {
            if current_char.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if current_char.is_ascii_alphabetic() {
                return self.id();
            }

            if current_char.is_ascii_digit() {
                return Ok(token::Kind::INTEGER(self.integer()));
            }

            if current_char == ':' && self.peek().is_some() && self.peek().unwrap() == '=' {
                self.advance();
                self.advance();
                return Ok(token::Kind::ASSIGN);
            }

            let token = match current_char {
                ';' => token::Kind::SEMI,
                '*' => token::Kind::MUL,
                '-' => token::Kind::MINUS,
                '+' => token::Kind::PLUS,
                '/' => token::Kind::DIV,
                '(' => token::Kind::LPAREN,
                ')' => token::Kind::RPAREN,
                '.' => token::Kind::DOT,
                _ => return Err(self.err.to_string()),
            };
            self.advance();
            return Ok(token);
        }
        return Ok(token::Kind::EOF);
    }
}
