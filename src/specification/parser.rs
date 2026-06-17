use ndarray::Array2;
use scirs2_linalg::eigen::generalized;

use crate::specification::command::{DefCommand, InitCommand, Initialization, ModelCommand, Section, TransitionDeclaration, VarDeclaration, Command};
use crate::specification::errors::{SyntaxError,CodeError};
use crate::specification::generator::Generator;
use crate::specification::token::TokenType::Model;
use crate::{markov::MarkovChain, token::{Token,TokenType}};
use crate::specification::expression::{self, BinaryExpression, Expression, GroupingExpression, UnaryExpression, VariableExpression};
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
        if !self.match_token([token_type]) {
            return Result::Err(SyntaxError::new(self.peek(), expect));
        } else {
            return Result::Ok(self.advance());
        }
    }

    fn match_token<const i: usize>(&self, token_types: [TokenType; i]) -> bool {
        return token_types.contains(&self.peek().token_type);
    }

    fn match_and_advance<const i: usize>(&mut self, token_types: [TokenType; i]) -> bool {
        if token_types.contains(&self.peek().token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn is_section_end(&self) -> bool {
        self.match_token([TokenType::Def, TokenType::Init, TokenType::Model, TokenType::Eof])
    }

    fn scan_section(&mut self) -> Result<String, SyntaxError> {
        let section = String::new();
        return Result::Ok(section);
    }

    fn read_model_type(&mut self) -> Result<ModelType, SyntaxError>{
        let token = self.advance();
        return match &token.token_type {
            TokenType::Ctmc => Result::Ok(ModelType::Ctmc),
            TokenType::Dtmc => Result::Ok(ModelType::Dtmc),
            _ => Result::Err(SyntaxError::new(token, "Expect model type (ctmc or dtmc)"))
        }
    }

    fn scan_def_section(&mut self) -> Result<DefCommand, SyntaxError>{
        let mut def = DefCommand::new();
        while !self.is_section_end() {
            while(self.peek().token_type == TokenType::NewLine) {
                self.consume(TokenType::NewLine, "");
            }
            match self.scan_var_decl() {
                Ok(cmd) => {
                    self.consume(TokenType::NewLine, "Expect new line");
                    def.push(cmd);
                },
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        return Result::Ok(def);
    }

    fn scan_var_decl(&mut self) -> Result<VarDeclaration, SyntaxError> {
        let var_name = self.consume(TokenType::Identifier, "Expect variable name")?.clone();
        self.consume(TokenType::Equal, "Expect '=' after variable name")?;
        let val = self.parse_expression()?;
        self.consume(TokenType::NewLine, "Expect new line after variable declaration")?;
        return Result::Ok(VarDeclaration::new(var_name, val));
    }

    fn scan_model_section(&mut self) -> Result<ModelCommand, SyntaxError>{
        let mut def = ModelCommand::new();
        while !self.is_section_end() {
            while(self.peek().token_type == TokenType::NewLine) {
                self.consume(TokenType::NewLine, "");
            }
            match self.scan_transition_decl() {
                Ok(cmd) => {
                    self.consume(TokenType::NewLine, "Expect new line");
                    def.push(cmd);
                },
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        return Result::Ok(def);
    }

    fn scan_transition_decl(&mut self) -> Result<TransitionDeclaration, SyntaxError> {
        let start = self.advance().clone();
        self.consume(TokenType::Arrow, "Expect arrow after start node");
        let mut transitions = TransitionDeclaration::new(start);

        loop {
            self.consume(TokenType::LeftParen, "Expect '('")?;
            let expr = self.parse_expression()?;
            self.consume(TokenType::LeftParen, "Expect ')'")?;
            let next_node = self.consume(TokenType::Identifier, "Expect next state after transition rate/problability")?.clone();
            transitions.push(next_node, expr);
            if self.peek().token_type == TokenType::NewLine {
                break
            }
            self.consume(TokenType::Comma, "Expect ',' between transition");
        } 

        return Result::Ok(transitions);
    }


    fn scan_init_section(&mut self) -> Result<InitCommand, SyntaxError>{
        let mut def = InitCommand::new();
        while !self.is_section_end() {
            while(self.peek().token_type == TokenType::NewLine) {
                self.consume(TokenType::NewLine, "");
            }
            match self.scan_init() {
                Ok(cmd) => {
                    self.consume(TokenType::NewLine, "Expect new line");
                    def.push(cmd);
                },
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        return Result::Ok(def);
    }

    fn scan_init(&mut self) -> Result<Initialization, SyntaxError> {
        let node_name = self.consume(TokenType::Identifier, "Expect node name in initialization")?.clone();
        self.consume(TokenType::Equal, "Expect '=' after node name")?;
        let expr = self.parse_expression()?;
        self.consume(TokenType::NewLine, "Expect new line after each declaration")?;
        return Result::Ok(Initialization::new(node_name, expr));
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>, SyntaxError> {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Box<dyn Expression>, SyntaxError> {
        let left = self.parse_factor()?;
        let op: Token;
        if !self.match_token([TokenType::Plus, TokenType::Minus]) {
            return Result::Ok(left);
        }
        else {
            op = self.advance().clone();
        }
        let right = self.parse_factor()?;
        Result::Ok(Box::new(BinaryExpression::new(op, left, right)))
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Expression>, SyntaxError> {
        let left = self.parse_unary()?;
        let op: Token;
        if !self.match_token([TokenType::Mult, TokenType::Div]) {
            return Result::Ok(left);
        }
        else {
            op = self.advance().clone();
        }
        let right = self.parse_unary()?;
        Result::Ok(Box::new(BinaryExpression::new(op, left, right)))
    }

    fn parse_unary(&mut self) -> Result<Box<dyn Expression>, SyntaxError> {
        let op: Token;
        if !self.match_token([TokenType::Minus]) {
            self.parse_primary()
        } else {
            op = self.advance().clone();
            let right = self.parse_primary()?;
            Result::Ok(Box::new(UnaryExpression::new(op, right)))
        }
    }

    fn parse_primary(&mut self) -> Result<Box<dyn Expression>, SyntaxError> {
        if self.match_and_advance([TokenType::LeftParen]) {
            self.parse_grouping()
        } else {
            self.parse_literal()
        }
    }

    fn parse_literal(&mut self) -> Result<Box<dyn Expression>, SyntaxError> { //parse Number and variable
        let token = self.advance();
        match token.token_type {
            TokenType::Variable => Result::Ok(Box::new(VariableExpression::new(token.clone()))),
            _ => Result::Err(SyntaxError::new(token, "Expect variable or literal value"))
        }
    }

    fn parse_grouping(&mut self) -> Result<Box<dyn Expression>, SyntaxError> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::RightParen, "Expect closing ')' after an expression")?;
        Result::Ok(Box::new(GroupingExpression::new(expr)))
    }

    pub fn parse(&mut self) -> Result<(ModelType, DefCommand, ModelCommand, InitCommand), SyntaxError> {
        let model_type: ModelType = self.read_model_type()?;
        let mut def_section: DefCommand = DefCommand::new();
        let mut model_section: ModelCommand = ModelCommand::new();
        let mut init_section: InitCommand = InitCommand::new();

        self.consume(TokenType::NewLine, "Expect new line after model type")?;

        // scan sections
        while !self.match_token([TokenType::Eof]) {
            let token = self.advance(); 
            match token.token_type {
                TokenType::Def => {
                    while(self.peek().token_type == TokenType::NewLine) {
                        self.consume(TokenType::NewLine, "");
                    }
                    match self.scan_def_section() {
                        Ok(section) => {
                            def_section = section;
                        },
                        Err(err) => {
                            return Result::Err(err);
                        }
                    }
                }
                TokenType::Init => {
                    while(self.peek().token_type == TokenType::NewLine) {
                        self.consume(TokenType::NewLine, "");
                    }
                    match self.scan_init_section() {
                        Ok(section) => {
                            init_section = section;
                        },
                        Err(err) => {
                            return Result::Err(err);
                        }
                    }
                }
                TokenType::Model => {
                    while(self.peek().token_type == TokenType::NewLine) {
                        self.consume(TokenType::NewLine, "");
                    }
                    match self.scan_model_section() {
                        Ok(section) => {
                            model_section = section;
                        },
                        Err(err) => {
                            return Result::Err(err);
                        }
                    }
                }
                TokenType::NewLine => {
                    continue;
                }
                _ => {
                    return Result::Err(SyntaxError::new(&token, "Expected section name"));
                }
            }
        }
        return Result::Ok((model_type,def_section, model_section, init_section));
    }
}
