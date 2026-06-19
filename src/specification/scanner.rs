use std::string::String;
use crate::specification::errors::{ScanError};
use crate::token::{Token, TokenType, Literal};

pub struct Reader {
    source: Vec<char>,
    line: usize,
    pos: usize,
    current: usize,
    start: usize
}

impl Reader {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            line: 0,
            current: 0,
            pos: 0
        }
    }
    fn previous(&self) -> char {
        return self.source[self.current - 1]
    }
    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let c = self.source[self.current];
        self.current += 1;
        self.pos += 1;
        c
    }
    fn next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source[self.current + 1];
        }
    }
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if expected != self.peek() {
            return false;
        } else {
            self.advance();
            return true;
        }
    }
    pub fn scan_variable(&mut self) -> Result<Token, ScanError> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            _ = self.advance()
        }
        let line = self.line;
        let current = self.current;
        let start = self.start;
        let pos = self.pos - (current - start);
        let lexeme: String = self.source[start..current].iter().collect();
        return Result::Ok(Token::new(TokenType::Variable, lexeme, Literal::Null, line, pos))
    }
    pub fn scan_keyword(&mut self) -> Result<Token, ScanError> {
        while self.peek().is_alphanumeric() {
            _ = self.advance()
        }
        let line = self.line;
        let current = self.current;
        let start = self.start;
        let pos = self.pos - (current - start);
        let lexeme: String = self.source[start..current].iter().collect();
        return match lexeme.as_str() {
            "init" => Result::Ok(Token::new(TokenType::Init, lexeme, Literal::Null, line, pos)),
            "ctmc" => Result::Ok(Token::new(TokenType::Ctmc, lexeme, Literal::Null, line, pos)),
            "dtmc" => Result::Ok(Token::new(TokenType::Dtmc, lexeme, Literal::Null, line, pos)),
            "def" => Result::Ok(Token::new(TokenType::Def, lexeme, Literal::Null, line, pos)),
            "model" => Result::Ok(Token::new(TokenType::Model, lexeme, Literal::Null, line, pos)),
            _ => Result::Ok(Token::new(TokenType::Identifier, lexeme.clone(), Literal::Null, line, pos))
        }
    }
    pub fn scan_number(&mut self) -> Result<Token, ScanError> {
        while self.peek().is_numeric() {
            _ = self.advance()
        }
        if self.peek() == '.' && self.next().is_ascii_digit() {
            self.advance(); // consume '.'

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let line = self.line;
        let current = self.current;
        let start = self.start;
        let pos = self.pos - (current - start);
        let lexeme: String = self.source[start..current].iter().collect();
        let val: f64 = lexeme.parse::<f64>().unwrap();
        return Result::Ok(Token::new(TokenType::Value, lexeme, Literal::Float(val), line, pos))
    }
    pub fn scan_token(&mut self) -> Result<Token, ScanError> {
        let line = self.line;
        let pos = self.pos;
        let c = self.advance();
        let lexeme = String::from(c);
        match c {
            '(' => Result::Ok(Token::new(TokenType::LeftParen, lexeme, Literal::Null, line, pos)),
            ')' => Result::Ok(Token::new(TokenType::RightParen, lexeme, Literal::Null, line, pos)),
            ':' => Result::Ok(Token::new(TokenType::Colon, lexeme, Literal::Null, line, pos)),
            ',' => Result::Ok(Token::new(TokenType::Comma, lexeme, Literal::Null, line, pos)),
            '=' => Result::Ok(Token::new(TokenType::Equal, lexeme, Literal::Null, line, pos)),
            '-' => if self.match_next('>') {
                Result::Ok(Token::new(TokenType::Arrow, format!("{}{}", c, self.previous()), Literal::Null, line, pos))
            } else {
                Result::Ok(Token::new(TokenType::Minus, lexeme, Literal::Null, line, pos))
            },
            '+' => Result::Ok(Token::new(TokenType::Plus, lexeme, Literal::Null, line, pos)),
            '*' => Result::Ok(Token::new(TokenType::Mult, lexeme, Literal::Null, line, pos)),
            '/' => Result::Ok(Token::new(TokenType::Div, lexeme, Literal::Null, line, pos)),
            '\n' => {
                self.pos = 0;
                self.line += 1;
                Result::Ok(Token::new(TokenType::NewLine, lexeme, Literal::Null, line, pos))
            },
            ' ' | '\r' | '\t' => {
                Result::Ok(Token::new(TokenType::Skip, lexeme, Literal::Null, line, pos))
            },
            '$' => if self.peek().is_alphanumeric() {
                self.scan_variable()
            } else {
                Result::Err(ScanError::new(line, pos, lexeme, "Expect an identifier after $"))
            },
            '\0' => Result::Ok(Token::new(TokenType::Eof, lexeme, Literal::Null, line, pos)),
            _ => {
                if c.is_ascii_digit() {
                    self.scan_number()
                } else if c.is_alphabetic() {
                    self.scan_keyword()
                } else {
                    Result::Err(ScanError::new(line, pos, lexeme, "Invalid character(s)"))
                }
            }
        }
    }
    pub fn scan(&mut self) -> Result<Vec<Token>, ScanError> {
        let mut prev_type: TokenType = TokenType::Eof;
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            let result = self.scan_token();
            match result {
                Ok(token) => {
                    if token.token_type == TokenType::Skip || (prev_type == TokenType::NewLine && token.token_type == TokenType::NewLine) {
                        continue;
                    } else {
                        prev_type = token.token_type;
                        tokens.push(token);
                    }
                },
                Err(error) => return Result::Err(error)
            }
        }
        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::Null,
            self.line,
            self.pos,
        ));
        return Result::Ok(tokens);
    }
}