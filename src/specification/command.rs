use crate::specification::errors::{CodeError, IdentifierNotFoundError, TypeError};
use crate::specification::expression::Value::Number;
use crate::specification::generator::Generator;
use crate::specification::expression::{Expression, Value};
use crate::specification::token::Token;
use std::collections::HashMap;

trait Command {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>>;
}

pub struct DefCommand {
    dec: Vec<VarDeclaration>
}

impl Command for DefCommand {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>>{
        for dec in self.dec.iter() {
            match dec.run(generator){
                Ok(_) => {
                    continue;
                },
                Err(err) => {
                    return Result::Err(err)
                }
            }
        }
        return Result::Ok(());
    }
}

pub struct VarDeclaration {
    name: Token,
    val: Box<dyn Expression>
}

impl Command for VarDeclaration {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>>{
        let name = &self.name.lexeme;
        return match self.val.eval(generator) {
            Ok(value) => {
                generator.add_variable(name, value);
                Result::Ok(())
            },
            Err(err) => Result::Err(err)
        }
    }
}

pub struct ModelCommand {
    transitions: Vec<TransitionDeclaration>
}

impl Command for ModelCommand {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>> {
        for transition in self.transitions.iter() {
            match transition.run(generator){
                Ok(_) => {
                    continue;
                },
                Err(err) => {
                    return Result::Err(err)
                }
            }
        }
        return Result::Ok(());
    }
}

pub struct TransitionDeclaration {
    current: Token,
    next: HashMap<Token, Box<dyn Expression>>
}

impl Command for TransitionDeclaration {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>> {
        let current = &self.current.lexeme;
        generator.add_node(current);
        for (k,v) in self.next.iter() {
            let next = &k.lexeme;
            match v.eval(generator){
                Ok(value) => match value {
                        Value::Number(val) => {
                            generator.set_transition(current, next, val);
                        },
                        Value::Boolean(_) => {
                            return Result::Err(Box::new(TypeError::new(String::from("Float"), String::from("Boolean"))));
                        }
                    }
                Err(err) => {
                    return Result::Err(err);
                }
            }
        }
        return Result::Ok(());
    }
}

pub struct InitCommand {
    inits: Vec<Initialization>
}

impl Command for InitCommand {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>> {
        for initialization in self.inits.iter() {
            match initialization.run(generator){
                Ok(_) => {
                    continue;
                },
                Err(err) => {
                    return Result::Err(err)
                }
            }
        }
        return Result::Ok(());
    }
}


pub struct Initialization {
    name: Token,
    val: Box<dyn Expression>
}

impl Command for Initialization {
    fn run(&self, generator: &mut Generator) -> Result<(), Box<dyn CodeError>> {
        match self.val.eval(generator) {
            Ok(val) => {
                match val {
                    Value::Number(val) => {
                        if generator.initialize_node(&self.name.lexeme, val) {
                            Result::Ok(())
                        } else {
                            Result::Err(Box::new(IdentifierNotFoundError::new(&self.name)))
                        }
                    }, 
                    Value::Boolean(_) => {
                        Result::Err(Box::new(TypeError::new(String::from("Number"), String::from("Bool"))))
                    }
                }
            },
            Err(err) => {
                Result::Err(err)
            }
        }
    }
}