mod markov;
mod specification;
use crate::markov::{MarkovChain};
use crate::specification::errors::CodeError;
use crate::specification::parser::Parser;
use crate::specification::{token, scanner};
use crate::specification::simulator::Simulator;
use std::fs;
use std::env::args;

fn main() {

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("No input file!");
    }
    let name = args[1].clone();

    let mut reader = scanner::Reader::new(fs::read_to_string(name).unwrap());
    let tokens = reader.scan().unwrap_or_else(|e| panic!("{}", e.what()));
    
    let mut parser = Parser::new(tokens);
    let (model_type, def_sec, model_sec, init_sec) =parser.parse().unwrap();
    
    let mut simulator: Simulator = Simulator::new(model_type, def_sec, model_sec, init_sec).unwrap_or_else(|e| panic!("{}", e.what()));
    let model = simulator.generate();

    println!("Transition matrix: \n{}", model.matrix());

    for i in 1..=20 {
        let s = simulator.next().clone();
        let t = simulator.time();
        println!("step {} t={}, transitioned to {}", i, t,  s);
    }

    println!("State problability distribution after 20 steps: {}", simulator.dist());
} 