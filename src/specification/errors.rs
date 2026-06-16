use scirs2_linalg::simd_ops::neural::Experience;

use crate::token::{Token};
use std::string::String;
pub trait CodeError {
    fn what(&self) -> String;
}

pub struct ScanError {
    line: usize,
    pos: usize,
    lexeme: String,
    msg: &'static str
}

impl ScanError {
    pub fn new(line: usize, pos: usize, lexeme: String, msg: &'static str) -> Self {
        Self {
            line: line,
            pos: pos,
            msg: msg,
            lexeme: lexeme
        }
    }
}

impl CodeError for ScanError {
    fn what(&self) -> String {
        return String::from(format!("{} at {}:{}, {}",self.lexeme, self.line, self.pos, self.msg));
    }
}

pub struct SyntaxError {
    line: usize,
    pos: usize,
    lexeme: String,
    msg: &'static str
}

impl SyntaxError {
    pub fn new(token: &Token, msg: &'static str) -> Self {
        Self {
            line: token.line,
            pos: token.pos,
            lexeme: token.lexeme.clone(),
            msg: msg
        }
    }
}

impl CodeError for SyntaxError {
    fn what(&self) -> String {
        return String::from(format!("Syntax Error! {} at {}:{}, {} ",self.lexeme, self.line, self.pos, self.msg));
    }
}

pub struct TypeError {
    expected: String,
    given: String
}

impl TypeError {
    pub fn new(expected: String, given: String) -> Self {
        Self {
            expected: expected,
            given: given
        }
    }
}

impl CodeError for TypeError {
    fn what(&self) -> String {
        return String::from(format!("Type Error: expected {} but given {}", self.expected, self.given));
    }
}