use crate::specification::errors::{CodeError, TypeError};
use crate::specification::simulator::Simulator;
use crate::specification::expression::{Expression, Value};
use crate::specification::token::Token;
use std::collections::HashMap;

pub trait Command {
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>>;
}

pub trait Section<T> {
    fn new() -> Self;
    fn push(&mut self, stmt: T);
}

pub struct DefCommand {
    dec: Vec<VarDeclaration>
}

impl Command for DefCommand {
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>>{
        for dec in self.dec.iter() {
            match dec.run(simulator){
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

impl Section<VarDeclaration> for DefCommand {
    fn new() -> Self {
        Self {
            dec: Vec::new()
        }
    }
    fn push(&mut self, stmt: VarDeclaration) {
        self.dec.push(stmt);
    }
}

pub struct VarDeclaration {
    name: Token,
    val: Box<dyn Expression>
}

impl VarDeclaration {
    pub fn new(token: Token, val: Box<dyn Expression>) -> Self {
        Self {
            name: Token::from(token),
            val: val
        }
    }
}

impl Command for VarDeclaration {
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>>{
        let name = &self.name.lexeme;
        return match self.val.eval(simulator) {
            Ok(value) => {
                simulator.add_variable(name, value);
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
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>> {
        for transition in self.transitions.iter() {
            match transition.run(simulator){
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

impl Section<TransitionDeclaration> for ModelCommand {
    fn new() -> Self {
        Self {
            transitions: Vec::new()
        }
    }
    fn push(&mut self, stmt: TransitionDeclaration) {
        self.transitions.push(stmt);
    }
}

pub struct TransitionDeclaration {
    current: Token,
    next: HashMap<String, Box<dyn Expression>>
}

impl TransitionDeclaration {
    pub fn new(current: Token) -> Self{
        Self {
            current: current,
            next: HashMap::new()
        }
    }
    pub fn push(&mut self, next: Token, expr: Box<dyn Expression>) {
        self.next.insert(next.lexeme, expr);
    }
}

impl Command for TransitionDeclaration {
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>> {
        let current = &self.current.lexeme;
        simulator.add_node(current);
        for (k,v) in self.next.iter() {
            let next = k;
            simulator.add_node(next);
            match v.eval(simulator)? {
                Value::Number(val) => {
                    simulator.set_transition(current, next, val);
                }
                Value::Boolean(_) => {
                    return Result::Err(Box::new(TypeError::new(String::from("Float"), String::from("Boolean"))));
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
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>> {
        for initialization in self.inits.iter() {
            match initialization.run(simulator){
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

impl Section<Initialization> for InitCommand {
    fn new() -> Self {
        Self {
            inits: Vec::new()
        }
    }
    fn push(&mut self, stmt: Initialization) {
        self.inits.push(stmt);
    }
}

pub struct Initialization {
    name: Token,
}

impl Initialization {
    pub fn new(name: Token) -> Self {
        Self {
            name: name
        }
    }
}

impl Command for Initialization {
    fn run(&self, simulator: &mut Simulator) -> Result<(), Box<dyn CodeError>> {
        simulator.initialize_node(&self.name.lexeme, 1.0);
        Result::Ok(())
    }
}