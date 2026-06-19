# Markov Chain Simulator
This project aims to develop a markov chain simulator in Rust which generates and simulates the model according to model specification. DTMC and CTMC is already implemented. The next step is to create the model generator. 

**Progress** 
* [X] DTMC
* [X] CTMC
* [X] Tokenizer
* [X] Parser
* [X] Model Generator
* [X] Apply value constraint
    - Row sums in CTMC must equal 0.0
    - Row sums in DTMC must equal 1.0
* [X] Simulator object
* [ ] Code refactoring
* [ ] More control statements to reduce repetitive lines


## Model Specification Format
```
model_type

def:
    $var = value

model:
    node_name_1 -> (value) node_name_2, (value) node_name_3, ...
    node_name_2 -> (value) node_name_1, (value) node_name_3, ...
    ...

init:
    node_name


```

`{model_type}` : specify model type (`dcmc` or `dtmc`) \
`value` : transition probability within $[0,1]$ for discrete time markov chain / transition rate within $(-\infty,\infty)$ for continuous time markov chain. \
In DTMC, states whose transitions are not defined are absorbing states by default

## Preliminary conditions
- All transitions not specified in the code is assigned to rate/problability 0.0 by default
- Only one initial state with problability of 1.0 and if not specified, the first state in model section becomes the initial state by default.
- In DTMC, all transition values must sum up to 1.0

## Dependency
```
ndarray = "0.17.2"
rand = "0.10.1"
rand_distr = "0.6.0"
scirs2-linalg = "0.5.0"
```