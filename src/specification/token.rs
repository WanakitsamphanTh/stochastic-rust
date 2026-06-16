use std::string::String;

#[derive(Debug,PartialEq,Eq)]
pub enum TokenType {
    // Model types
    Dtmc,
    Ctmc,

    // Spaces
    NewLine,

    // Symbols
    Arrow,
    Comma,
    LeftParen,
    RightParen,
    Equal,
    Colon,

    //Arithmetic
    Plus,
    Minus,
    Mult,
    Div,

    // Keywords
    Init,
    Def,
    Model,

    // Value
    Variable,
    Identifier,
    Value,

    // EOF
    Eof
}

#[derive(Debug)]
pub enum Literal {
    Null,
    Float(f32), // real number literal
    Name(String) // string literal (variable name, node name)
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
    pub pos: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize, pos: usize) -> Self {
        Self {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
            pos: pos
        }
    }
    pub fn map_keyword(keyword: &str) -> TokenType {
        match keyword {
            "dtmc" => TokenType::Dtmc,
            "ctmc" => TokenType::Ctmc,
            "init" => TokenType::Init,
             _ => TokenType::Identifier
        }
    }
}


/*
binary: unary binaryOp unary
unary: [`-`]? primary
primary: grouping | number | variable
grouping: `(` expression `)`
number: '$' [0 | [1-9][0-9]*][.[1-9]*]?
variable: `$` [alphanumeric | _]
*/