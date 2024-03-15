# Viskum Interpreter

## Useful links

- SSA: https://en.wikipedia.org/wiki/Static_single-assignment_form
- CFG: https://en.wikipedia.org/wiki/Control-flow_graph
- Constant folding: https://en.wikipedia.org/wiki/Constant_folding
- SCCP: https://en.wikipedia.org/wiki/Sparse_conditional_constant_propagation (Constant folding w/ SSA)
- DCE: https://en.wikipedia.org/wiki/Dead-code_elimination

## TODO

Fix "unnused" expressions like: 1 + 2 will permanently take up a register. That should be detected in the control-flow graph and therefore the register should be released or "unnused" expression should be ignored.

Make assignment possible by doing: "mut i32 d" or "i32 d". ValueType is know Empty
