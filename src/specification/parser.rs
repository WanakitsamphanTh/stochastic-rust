use ndarray::Array2;

use crate::specification::errors::{SyntaxError,CodeError};
use crate::{markov::MarkovChain, token::{Token,TokenType}};
use crate::specification::expression;
use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
pub enum ModelType {
    Ctmc,
    Dtmc
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0
        }
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current]
    }
    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        return token;
    }
    fn consume(&mut self, token_type: TokenType, expect: &'static str) -> Result<&Token, SyntaxError> {
        if !self.match_token(token_type) {
            return Result::Err(SyntaxError::new(self.peek(), expect));
        } else {
            return Result::Ok(self.peek());
        }
    }
    fn read_model_type(&mut self) -> Result<ModelType, SyntaxError>{
        let token = self.advance();
        return match &token.token_type {
            TokenType::Ctmc => Result::Ok(ModelType::Ctmc),
            TokenType::Dtmc => Result::Ok(ModelType::Dtmc),
            _ => Result::Err(SyntaxError::new(token, "Expect model type (ctmc or dtmc)"))
        }
    }
    fn match_token(&mut self, token_type: TokenType) -> bool {
        return self.peek().token_type == token_type
    }
    
    fn scan_section(&mut self) -> Result<String, SyntaxError> {
        let section = String::new();
        return Result::Ok(section);
    }

    pub fn parse(&mut self) -> Result<(), SyntaxError> {
        let model_type: ModelType;
        
        // Read model type
        match self.read_model_type() {
            Ok(mod_type) => model_type = mod_type,
            Err(err) => {
                return Result::Err(err)
            }
        }
        self.consume(TokenType::NewLine, "Expect new line after model type").is_err_and(|e| panic!("{}", e.what()));

        while !self.match_token(TokenType::Eof) {
            let result: Result<String, SyntaxError> = self.scan_section();
            match result {
                Ok(_) => {
                    self.advance();
                },
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        
        print!("{:?}", model_type);
        return Result::Ok(());
    }
}
