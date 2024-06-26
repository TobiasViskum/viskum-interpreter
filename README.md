# Viskum Interpreter

## Useful links

- SSA: https://en.wikipedia.org/wiki/Static_single-assignment_form
- CFG: https://en.wikipedia.org/wiki/Control-flow_graph
  - Graph: https://en.wikipedia.org/wiki/Graph_(abstract_data_type)
  - https://groups.seas.harvard.edu/courses/cs153/2018fa/lectures/Lec17-CFG-dataflow.pdf
- Constant folding: https://en.wikipedia.org/wiki/Constant_folding
- SCCP: https://en.wikipedia.org/wiki/Sparse_conditional_constant_propagation (Constant folding w/ SSA)
- DCE: https://en.wikipedia.org/wiki/Dead-code_elimination
- More optimizations:
  - https://www.javatpoint.com/machine-independent-optimization

## TODO

- Rewrite the way a directed acyclic graph works. All basic blocks (linear sequence of stmts that don't jump) should be in its own cfg node which is called the Process node. Right now each stmt gets its own node, which makes optimizations harder

- Dead code analysis also have to scan for unnused variables

- Remove: PrecAssignment and other useless precedences (probably the only one) because '=' or ':=' is not part of a "expresison" but more of a way to structure an assignment statement.

## Machine code???

- MLIR: https://github.com/raviqqe/melior

- I should probalby use LLVM. Write my own backend for LLVM which means I have to learn LLVM syntax (it would be cool if the language could both be interpreted and compiled :))