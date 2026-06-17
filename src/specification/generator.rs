use std::collections::HashMap;
use ndarray::Array2;

use crate::markov::{CTMC, DTMC, MarkovChain, MarkovChainModel};
use crate::specification::{expression::Value, parser::ModelType};

pub struct Generator {
    model_type: ModelType,
    variable_map: HashMap<String, Value>,
    state_map: HashMap<String, usize>,
    transitions: HashMap<(usize,usize), f32>,
    init: Vec<f32>
}

impl Generator {
    pub fn new(model_type: ModelType) -> Self {
        Self {
            model_type: model_type,
            variable_map: HashMap::new(),
            state_map: HashMap::new(),
            transitions: HashMap::new(),
            init: Vec::new()
        }
    }
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
        let index = self.state_map.len();
        if !self.state_map.contains_key(name) {
            self.state_map.insert(name.clone(), index);
            self.init.push(0.0);
        }
    }
    pub fn set_transition(&mut self, current: &String, next: &String, transition_value: f32) {
        let current_id = self.state_map[current];
        let next_id = self.state_map[next];
        self.transitions.insert((current_id, next_id), transition_value);
    }
    pub fn initialize_node(&mut self, node: &String, value: f32) -> bool {
        if self.state_map.contains_key(node) {
            let node_id = self.state_map[node];
            self.init[node_id] = value;
            true
        } else {
            false
        }
    }
    pub fn generate(&self) -> MarkovChainModel {
        let n = self.init.len();
        let mut matrix = Array2::<f32>::from_shape_fn((n,n), |_| 0.0);
        for (&(curr,next), &val) in self.transitions.iter() {
            matrix[[curr,next]] = val;
        }
        match self.model_type {
            ModelType::Ctmc => MarkovChainModel::Ctmc(CTMC::from_arr(self.init.clone(), matrix)),
            ModelType::Dtmc => MarkovChainModel::Dtmc(DTMC::from_arr(self.init.clone(), matrix))
        }
    }
}