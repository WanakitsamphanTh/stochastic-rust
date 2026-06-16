use ndarray::Array2;

use crate::{markov::MarkovChain, token::Token};
use std::collections::HashMap;
use std::string::String;
enum ModelType {
    Ctmc,
    Dtmc
}

trait Command {

}

pub struct SectionCommand {

}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
}

pub struct generator {
    model_type: ModelType,
    node_map: HashMap<String, i32>,
    transitions: Array2<f32>,
}