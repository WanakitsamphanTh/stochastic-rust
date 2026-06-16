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
 node_name_1 = value
 ...

```

`{model_type}` : specify model type (`dcmc` or `dtmc`) \
`value` : transition probability within $[0,1]$ for discrete time markov chain / transition rate within $(-\infty,\infty)$ for continuous time markov chain.

## Preliminary conditions
- All transitions not specified in the code is assigned to rate/problability 0.0 by default
- All initial states not mentioned in the code is assigned to 0.0 by default
- In DTMC, all transition values must sum up to 1.0