mod markov;
mod specification;

use crate::markov::{DTMC, CTMC, MarkovChain, Time};
use crate::specification::errors::CodeError;
use crate::specification::{token, scanner};
use std::fs;

fn knuth_die() {
    let transition = |curr: usize, next: usize| -> f32  {
        match curr {
            0 => match next {
                1 => 0.5,
                2 => 0.5,
                _ => 0.0
            },
            1 => match next {
                3 => 0.5,
                4 => 0.5,
                _ => 0.0
            },
            2 => match next {
                5 => 0.5,
                6 => 0.5,
                _ => 0.0
            },
            3 => match next {
                1 => 0.5,
                7 => 0.5,
                _ => 0.0
            }
            4 => match next {
                8 => 0.5,
                9 => 0.5,
                _ => 0.0
            }
            5 => match next {
                2 => 0.5,
                10 => 0.5,
                _ => 0.0
            }
            6 => match next {
                11 => 0.5,
                12 => 0.5,
                _ => 0.0
            }
            7..=12 => if next == curr {1.0} else {0.0},
            _ => 0.0
        }
    };

    let init_prob = vec![1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];

    let mut dtmc: DTMC = DTMC::from_fn(init_prob,13, transition);

    print!("Knuth die algorithm: \n");
    dtmc.simulate(Time::Dt(50));
    let states = dtmc.state();
    print!("after 50 steps: {0}\n", states);
}

fn queue_system() {
    let init = vec![1.0,0.0,0.0,0.0,0.0,0.0,0.0];
    let mu = 2.0;
    let lambda = 5.0;
    let rate_fn = |curr: usize, next: usize| -> f32 {
        if curr == 0 {
            if next == 1 {
                mu
            } else {
                0.0
            }
        } else {
            if next == curr + 1 {
                lambda
            } else if next == curr - 1 {
                mu
            } else {
                0.0
            }
        }
    };
    let mut ctmt: CTMC = CTMC::from_fn(init, 7, rate_fn);
    print!("Queue system: \n");
    ctmt.simulate(Time::Ct(100.0));
    let states = ctmt.state();
    print!("after t=100.0: {0}\n", states);
}


/*
fn main() {
    knuth_die();
    queue_system();
} */

fn main() {

    let mut reader = scanner::Reader::new(fs::read_to_string("./input/model.txt").unwrap());
    let result = reader.tokens();
    match result {
        Ok(tokens) => for token in tokens {
            print!("{:?}\n",token);
        }
        Err(error) => {
            print!("{}", error.what()); 
        }   
    }
} 