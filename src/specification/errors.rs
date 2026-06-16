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
    token: Token,
    msg: String
}
