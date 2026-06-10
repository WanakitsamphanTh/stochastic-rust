## Knuth's Die Algorithm
```mermaid
stateDiagram-v2
    direction LR
    classDef hidden display: none;
    start:::hidden --> s0
    s0 --> s1 : 0.5
    s1 --> s3 : 0.5
    s1 --> s4 : 0.5
    s3 --> s1 : 0.5
    s3 --> 1 : 0.5
    1 --> 1 : 1
    s4 --> 2 : 0.5
    2 --> 2 : 1
    s4 --> 3 : 0.5
    3 --> 3 : 1
    s0 --> s2 : 0.5
    s2 --> s5 : 0.5
    s5 --> s2 : 0.5
    s5 --> 4 : 0.5
    4 --> 4 : 2
    s2 --> s6 : 0.5
    s6 --> 5 : 0.5
    5 --> 5 : 1
    s6 --> 6 : 0.5
    6 --> 6 : 1
