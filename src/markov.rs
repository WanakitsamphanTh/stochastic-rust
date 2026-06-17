use ndarray::{self, Array, Array2, Axis};
use scirs2_linalg::expm;

pub type TransitionMatrix = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>, f32>;
pub type States = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 1]>, f32>;
pub type State = usize;

pub enum Time {
    Dt(i32),
    Ct(f32)
}

pub trait MarkovChain {
    fn from_arr(init_prob: Vec<f32>, matrix: Array2<f32>) -> Self;
    fn from_fn(init_prob: Vec<f32>, n_state: usize, transition_fn: impl Fn(usize, usize) -> f32) -> Self;
    fn state(&self) -> States;
    fn matrix(&self) -> TransitionMatrix;
    fn simulate(&mut self, t: Time);
}

pub struct DTMC {
    s: States,
    p: TransitionMatrix,
}

impl DTMC {
    fn next_step(&mut self)  {
        self.s = self.s.dot(&self.p);
    }
}

impl MarkovChain for DTMC {
    fn from_arr(init_prob: Vec<f32>, matrix: Array2<f32>) -> Self {
        let s = Array::from_vec(init_prob);
        Self {
            s: s,
            p: matrix
        }
    }
    fn from_fn(init_prob: Vec<f32>, n_state: usize, transition_fn: impl Fn(usize, usize) -> f32) -> Self {
        let p: TransitionMatrix = Array2::from_shape_fn((n_state,n_state), |(s_curr, s_next)| transition_fn(s_curr,s_next));
        return Self::from_arr(init_prob, p);
    }
    fn state(&self) -> States {
        return self.s.clone();
    }
    fn matrix(&self) -> TransitionMatrix {
        return self.p.clone();
    }
    fn simulate(&mut self, times: Time) {
        match times {
            Time::Dt(steps) => {
                for _ in 1..=steps {
                            self.next_step();
                        }
                },
            Time::Ct(_) => {
                panic!("DTMC only accepts time as integer")
            }
        }
    }
}

pub struct CTMC {
    s: States,
    rate: TransitionMatrix,
}

impl CTMC {
}

impl MarkovChain for CTMC {
    fn from_arr(init_prob: Vec<f32>, matrix: Array2<f32>) -> Self {
        let init =  Array::from_vec(init_prob);
        Self {
            s: init,
            rate: matrix
        }
    }
    fn from_fn(init_rate: Vec<f32>, n_state: usize, rate_fn: impl Fn(usize, usize) -> f32) -> Self {
        let mut p: TransitionMatrix = Array2::from_shape_fn((n_state,n_state), |(s_curr, s_next)| rate_fn(s_curr,s_next));
        let diag = p.sum_axis(Axis(1));
        for i in 0..n_state {
            p[[i,i]] = -diag[i];
        }
        return Self::from_arr(init_rate, p);
    }
    fn state(&self) -> States {
        return self.s.clone();
    }
    fn matrix(&self) -> TransitionMatrix {
        return self.rate.clone();
    }
    fn simulate(&mut self, time: Time){
        match time {
            Time::Ct(t) => {
                let p: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>, f32> = expm(&(&self.rate * t).view(),None).expect("Simulation failed");
                self.s = self.s.dot(&p);
            },
            Time::Dt(ti) => {
                let t = ti as f32;
                let p: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>, f32> = expm(&(&self.rate * t).view(),None).expect("Simulation failed");
                self.s = self.s.dot(&p);
            }
        }
    }
}

pub enum MarkovChainModel {
    Dtmc(DTMC),
    Ctmc(CTMC)
}