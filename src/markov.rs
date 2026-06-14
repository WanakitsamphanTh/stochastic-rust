use ndarray::{self, Array, Array2, Axis};
use scirs2_linalg::expm;

pub type TransitionMatrix = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>, f32>;
pub type States = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 1]>, f32>;
pub type State = usize;


pub trait MarkovChain<T> {
    fn state(&self) -> States;
    fn matrix(&self) -> TransitionMatrix;
    fn simulate(&mut self, t: T);
}

pub struct DTMC {
    s: States,
    p: TransitionMatrix,
}

impl DTMC {
    pub fn new(s0: State, init_prob: f32, n_state: usize, transition_fn: impl Fn(usize, usize) -> f32) -> Self {
        let p: TransitionMatrix = Array2::from_shape_fn((n_state,n_state), |(s_curr, s_next)| transition_fn(s_curr,s_next));
        let mut init =  Array::from_vec(vec![0.0; n_state]);
        init[s0] = init_prob;
        Self {
            s: init,
            p: p
        }
    }
    fn next_step(&mut self)  {
        self.s = self.s.dot(&self.p);
    }
}

impl MarkovChain<i32> for DTMC {
    fn state(&self) -> States {
        return self.s.clone();
    }
    fn matrix(&self) -> TransitionMatrix {
        return self.p.clone();
    }
    fn simulate(&mut self, times: i32) {
        for _ in 1..=times {
            self.next_step();
        }
    }
}

pub struct CTMC {
    s: States,
    rate: TransitionMatrix,
}

impl CTMC {
    pub fn new(init_rate: Vec<f32>, n_state: usize, rate_fn: impl Fn(usize, usize) -> f32) -> Self {
        let mut p: TransitionMatrix = Array2::from_shape_fn((n_state,n_state), |(s_curr, s_next)| rate_fn(s_curr,s_next));
        let diag = p.sum_axis(Axis(1));
        for i in 0..n_state {
            p[[i,i]] = -diag[i];
        }
        let init =  Array::from_vec(init_rate);
        Self {
            s: init,
            rate: p,
        }
    }
}

impl MarkovChain<f32> for CTMC {
    fn state(&self) -> States {
        return self.s.clone();
    }
    fn matrix(&self) -> TransitionMatrix {
        return self.rate.clone();
    }
    fn simulate(&mut self, t: f32){
        let p = expm(&(&self.rate * t).view(),None).expect("Simulation failed");
        self.s = self.s.dot(&p);
    }
}
