use ndarray::{self, Array};

pub type TransitionMatrix = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>, f32>;
pub type States = ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 1]>, f32>;
pub type State = usize;

pub trait MarkovChain {
    fn next_step(&mut self);
    fn simulate(&mut self, times: i32);
    fn reset(&mut self);
    fn state(&self) -> States;
}

pub struct DTMC {
    s0: States,
    s: States,
    p: TransitionMatrix,
}

impl DTMC {
    pub fn new(p: TransitionMatrix, s0: State, n_state: usize) -> Self {
        let mut init =  Array::from_vec(vec![0.0; n_state]);
        init[s0] = 1.0;
        Self {
            s0: init.clone(),
            s: init.clone(),
            p: p.clone()
        }
    }
}

impl MarkovChain for DTMC {
    fn next_step(&mut self)  {
        self.s = self.s.dot(&self.p);
    }
    fn reset(&mut self) {
        self.s = self.s0.clone();
    }
    fn simulate(&mut self, times: i32) {
        for _ in 1..=times {
            self.next_step();
        }
    }
    fn state(&self) -> States {
        return self.s.clone();
    }
}

pub struct CTMC {
    s0: States,
    s: States,
    p: TransitionMatrix,
}