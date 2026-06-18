use std::collections::HashMap;

use crate::markov::{MarkovChain, ModelType, StateDist, State};
use crate::specification::command::{Command, DefCommand, InitCommand, ModelCommand};
use crate::specification::errors::CodeError;
use crate::specification::{expression::Value};

pub struct Simulator {
    model_type: ModelType,
    variable_map: HashMap<String, Value>,
    state_map: HashMap<String, usize>,
    state_map_rev: HashMap<usize, String>,
    transitions: HashMap<(usize,usize), f32>,
    init: Vec<f32>,
    curr: State,
    model: MarkovChain,
    steps: usize
}

impl Simulator {
    pub fn new(model_type: ModelType, def: DefCommand, model: ModelCommand, init: InitCommand) -> Result<Self, Box<dyn CodeError>> {
        let mut sim = Self {
            model_type: model_type,
            variable_map: HashMap::new(),
            state_map: HashMap::new(),
            state_map_rev: HashMap::new(),
            transitions: HashMap::new(),
            init: Vec::new(),
            model: MarkovChain::new(),
            curr: 0,
            steps: 0
        };
        def.run(&mut sim)?;
        model.run(&mut sim)?;
        init.run(&mut sim)?;
        return Result::Ok(sim);
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
        let index: usize = self.state_map.len();
        if !self.state_map.contains_key(name) {
            self.state_map.insert(name.clone(), index);
            self.state_map_rev.insert(index, name.clone());
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

    pub fn generate(&mut self) -> &MarkovChain {
        let n = self.init.len();
        self.model = MarkovChain::from_pairs(self.model_type, n, self.init.clone(), self.curr, &self.transitions);
        return &self.model;
    }

    pub fn next(&mut self) -> &String{
        self.steps += 1;
        let s= self.model.step();
        return &self.state_map_rev[&s];
    }

    pub fn reset(&mut self) {
        self.model.reassign_state(self.init.clone(), self.init);
        self.steps = 0;
    }

    pub fn stationary(&mut self, eps: f32, max_T: usize) -> StateDist {
        let prev = self.model.state();
        for i in 1..=max_T {
            self.model.step();
            let curr = self.model.state();
            let err = (curr - &prev).abs().mean().unwrap();
            if err < eps {
                break
            }
        }
        return self.dist();
    }

    pub fn dist(&self) -> StateDist{
        return self.model.state();
    }

    pub fn dist_of<'a, const i: usize>(&self, states: [&'a str; i]) -> HashMap<&'a str, f32> {
        let mut dist: HashMap<&'a str, f32> = HashMap::new();
        let curr_dist = self.dist();
        for s in states {
            let state = String::from(s);
            let st = self.state_map[&state];
            let p = curr_dist[st];
            dist.insert(s, p);
        }
        return dist;
    }
}