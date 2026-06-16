use std::collections::HashMap;
use ndarray::Array2;

use crate::markov::MarkovChain;
use crate::specification::command;
use crate::specification::errors::CodeError;
use crate::specification::{expression::Value, parser::ModelType};
pub struct Generator {
    model_type: ModelType,
    variable_map: HashMap<String, Value>,
    node_map: HashMap<String, usize>,
    transitions: HashMap<(usize,usize), f32>,
    init: Vec<f32>
}

impl Generator {
    pub fn add_variable(&mut self, name: &String, value: Value) {
        self.variable_map.insert(name.clone(), value);
    }
    pub fn lookup_variable(&mut self, name: &String) -> Result<Value, bool> {
        if self.variable_map.contains_key(name) {
            Result::Ok(self.variable_map[name])
        } else {
            Result::Err(true)
        }
    }
    pub fn add_node(&mut self, name: &String) {
        let index = self.node_map.len();
        if !self.node_map.contains_key(name) {
            self.node_map.insert(name.clone(), index);
            self.init.push((0.0));
        }
    }
    pub fn set_transition(&mut self, current: &String, next: &String, transition_value: f32) {
        let current_id = self.node_map[current];
        let next_id = self.node_map[next];
        self.transitions.insert((current_id, next_id), transition_value);
    }
    pub fn initialize_node(&mut self, node: &String, value: f32) -> bool {
        if self.node_map.contains_key(node) {
            let node_id = self.node_map[node];
            self.init[node_id] = value;
            true
        } else {
            false
        }
    }
    pub fn generate(&self) {
        let state_number = self.init.len();
    }
}