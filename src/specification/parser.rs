use crate::{markov::MarkovChain, token::Token};
use std::collections::HashMap;
use std::string::String;
enum ModelType {
    Ctmc,
    Dtmc
}

trait Command {
    
}

pub struct Parser {
    tokens: Vec<Token>,
    commands: Vec<dyn Command>
}

impl Parser {

}

pub struct Model {
    node: HashMap<String, usize>
}

impl Model {
    pub fn generate(&self) -> impl MarkovChain {}
}