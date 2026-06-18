use std::collections::HashMap;
use rand::distr::weighted::WeightedIndex;
use rand::{distr, prelude::*};
use rand::distr::{weighted, Distribution};
use rand_distr::Exp;
use ndarray::{self, Array, Array2, Axis};
use scirs2_linalg::{expm};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ModelType {
    Ctmc,
    Dtmc,
    None
}

pub type TransitionMatrix = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>, f32>;
pub type StateDist = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 1]>, f32>;
pub type State = usize;


#[derive(Debug, Clone)]
pub struct MarkovChain {
    curr: State,
    s: StateDist,
    p: TransitionMatrix,
    model_type: ModelType,
    time: f32
}

impl MarkovChain {
    pub fn new() -> Self {
        Self {
            curr: 0,
            s: Array::from_vec(Vec::new()),
            p: Array2::from_shape_vec((1,1),vec![0.0]).unwrap(),
            model_type: ModelType::None,
            time: 0.0
        }
    }

    pub fn from_arr(model_type: ModelType, init_prob: Vec<f32>, curr: State, mut matrix: Array2<f32>) -> Self {
        let s = Array::from_vec(init_prob);

        let row_sums = matrix.sum_axis(Axis(1));
        if model_type == ModelType::Ctmc {
            assert!(row_sums.iter().all(|&p| p == 0.0));
        }

        if model_type == ModelType::Dtmc {
            for (i, &p) in row_sums.iter().enumerate() {
                if p == 0.0 {
                    matrix[[i,i]] = 1.0;
                }
            }
            assert!(matrix.iter().all(|&p| p <= 1.0 && p >= 0.0));
        }

        Self {
            s: s,
            curr: curr,
            p: matrix,
            model_type: model_type,
            time: 0.0
        }
    }

    pub fn from_fn(model_type: ModelType, n_state: usize, init_prob: Vec<f32>, curr: usize, transition_fn: impl Fn(usize, usize) -> f32) -> Self {
        let mut p: TransitionMatrix = Array2::from_shape_fn((n_state,n_state), |(s_curr, s_next)| transition_fn(s_curr,s_next));
        let row_sums = p.sum_axis(Axis(1));

        if model_type == ModelType::Ctmc {
            for i in 0..n_state {
                p[[i,i]] = -row_sums[i];
            }
        }

        return Self::from_arr(model_type, init_prob, curr, p);
    }

    pub fn from_pairs(model_type: ModelType, n_state: usize, init_prob: Vec<f32>, curr: State, transitions: &HashMap<(usize,usize),f32>) -> Self {
        let mut p: TransitionMatrix = Array2::from_shape_fn((n_state,n_state), |_| 0.0);
        for (&(curr,next), &val) in transitions.iter() {
            p[[curr,next]] = val;
        }
        let row_sums = p.sum_axis(Axis(1));
        if model_type == ModelType::Ctmc {
            for i in 0..n_state {
                p[[i,i]] = -row_sums[i];
            }
        }
        return Self::from_arr(model_type, init_prob, curr, p);
    }

    pub fn reassign_state(&mut self, dist: Vec<f32>, s: State) {
        self.s = Array::from_vec(dist);
        self.curr = s;
    }

    pub fn state(&self) -> StateDist {
        return self.s.clone();
    }

    pub fn matrix(&self) -> TransitionMatrix {
        return self.p.clone();
    }

    pub fn time(&self) -> f32 {
        return self.time;
    }

    pub fn step(&mut self) -> State{
        match self.model_type {
            ModelType::Ctmc => {
                let mut rng = rand::rng();

                let rates = self.p.row(self.curr);
                let lambda = -rates[self.curr];
                let tau = Exp::new(lambda).unwrap().sample(&mut rng);

                if lambda != 0.0 { // if the current state is not an absorbing state
                    let probs = (&rates / lambda).map(|&x| if x < 0.0 { 0.0 } else {x}).to_vec();
                    let nexts = WeightedIndex::new(&probs).unwrap();
                    self.curr = rng.sample(nexts);
                }
                
                let p: TransitionMatrix = expm(&(&self.p).view(),None).expect("Simulation failed");
                self.s = self.s.dot(&p);
                self.time += tau;

                return self.curr;
            },
            ModelType::Dtmc => {
                let probs = self.p.row(self.curr).to_vec();
                if probs[self.curr] != 1.0 { //if the current state is not absorbing state
                    let nexts = WeightedIndex::new(&probs).unwrap();
                    let mut rng = rand::rng();
                    self.curr = rng.sample(nexts);
                }
                self.s = self.s.dot(&self.p);
                self.time += 1.0;
                return self.curr;
            },
            ModelType::None => {
                panic!("Not initialized yet");
            }
        }
    }
}