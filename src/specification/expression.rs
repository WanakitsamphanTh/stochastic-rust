use crate::specification::errors::{CodeError, IdentifierNotFoundError, SyntaxError, TypeError, ValueError};
use crate::specification::simulator::Simulator;
use crate::specification::token::{Literal, Token, TokenType};

macro_rules! numeric_binary_op {
    ($left:expr, $right:expr, $op:tt) => {{
        match ($left, $right) {
            (Value::Number(a), Value::Number(b)) => {
                return Result::Ok(Value::Number(a $op b))
            }
            _ => {
                return Result::Err(Box::new(TypeError::new(String::from("Number"), String::from("Boolean"))))
            }
        }
    }};
}

macro_rules! numeric_unary_op {
    ($right:expr, $op:tt) => {{
        match $right {
            Value::Number(a) => {
                return Result::Ok(Value::Number($op a))
            }
            _ => {
                return Result::Err(Box::new(TypeError::new(String::from("Number"), String::from("Boolean"))))
            }
        }
    }};
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool)
}

pub trait Expression {
    fn eval<'a>(&'a self, simulator: &mut Simulator) -> Result<Value, Box<dyn CodeError>>;
}


pub struct LiteralExpression {
    token: Token
}

impl LiteralExpression {
    pub fn new(token: Token) -> Self {
        Self {
            token: token
        }
    }
}

impl Expression for LiteralExpression {
    fn eval<'a>(&'a self, _: &mut Simulator) -> Result<Value, Box<dyn CodeError>>{
        match self.token.literal {
            Literal::Float(f) => Result::Ok(Value::Number(f)),
            _ => Result::Err(Box::new(ValueError::new(self.token.line, self.token.pos)))
        }
    }
}

pub struct VariableExpression {
    name: Token,
}

impl VariableExpression {
    pub fn  new(token: Token) -> Self {
        Self {
            name: token
        }
    }
}

impl Expression for VariableExpression {
    fn eval<'a>(&'a self, simulator: &mut Simulator) -> Result<Value, Box<dyn CodeError>>{
        match simulator.lookup_variable(&self.name.lexeme) {
            Ok(val) => Result::Ok(val),
            Err(_) => Result::Err(Box::new(IdentifierNotFoundError::new(&self.name)))
        }
    }
}

pub struct GroupingExpression {
    expression: Box<dyn Expression>
}

impl GroupingExpression {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        return Self {
            expression: expr
        }
    }
}

impl Expression for GroupingExpression {
    fn eval<'a>(&'a self, simulator: &mut Simulator) -> Result<Value, Box<dyn CodeError>>{
        return self.expression.eval(simulator);
    }
}

pub struct BinaryExpression {
    operator: Token,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>
}

impl BinaryExpression {
    pub fn new(operator: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self {
            operator: operator,
            left: left,
            right: right
        }
    }
}

impl Expression for BinaryExpression {
    fn eval<'a>(&'a self, simulator: &mut Simulator) -> Result<Value, Box<dyn CodeError>>{
        let left: Value;
        let right: Value;
        match self.left.eval(simulator) {
            Ok(v) => {
                left = v;
            },
            Err(err) => {
                return Result::Err(err);
            }
        }
        match self.right.eval(simulator) {
            Ok(v) => {
                right = v;
            },
            Err(err) => {
                return Result::Err(err);
            }
        }
        match self.operator.token_type {
            TokenType::Plus => {
                numeric_binary_op!(left,right,+);
            },
            TokenType::Minus => {
                numeric_binary_op!(left,right,-);
            }
            TokenType::Mult => {
                numeric_binary_op!(left,right,*);
            },
            TokenType::Div => {
                numeric_binary_op!(left,right,/);
            }
            _ => {
                return Result::Err(Box::new(SyntaxError::new(&self.operator, "Invalid operator")));
            }
        }
    }
}

pub struct UnaryExpression {
    operator: Token,
    right: Box<dyn Expression>
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Self {
            operator: operator,
            right: right
        }
    }
}

impl Expression for UnaryExpression {
    fn eval<'a>(&'a self, simulator: &mut Simulator) -> Result<Value, Box<dyn CodeError>>{
        let right: Value;
        match self.right.eval(simulator) {
            Ok(v) => {
                right = v;
            },
            Err(err) => {
                return Result::Err(err);
            }
        }
        match self.operator.token_type {
            TokenType::Minus => {
                 numeric_unary_op!(right,-);
            },
            _ => {
                return Result::Err(Box::new(SyntaxError::new(&self.operator, "Invalid operator")));
            }
        }
    }
}