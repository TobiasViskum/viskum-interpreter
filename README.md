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

- Move constant fold to CFG instead of AST (this makes it possible to use references in recursive structs instead of mutable raw pointers). This also makes it possible to change validate_stmt(&mut self) -> validate_stmt(&self) which also makes it possible that validate_stmt receives a reference to SymbolTableRef instead of a bitwise copy

- Rewrite the way a directed acyclic graph works. All basic blocks (linear sequence of stmts that don't jump) should be in its own cfg node which is called the Process node. Right now each stmt gets its own node, which makes optimizations harder

- Dead code analysis also have to scan for unnused variables

- Remove: PrecAssignment and other useless precedences (probably the only one) because '=' or ':=' is not part of a "expresison" but more of a way to structure an assignment statement.

- Fix "unnused" expressions like: 1 + 2 will permanently take up a register. That should be detected in the control-flow graph and therefore the register should be released or "unnused" expression should be ignored.

- Use an Arena for the AST instead to avoid countless heap allocations: https://users.rust-lang.org/t/is-there-a-better-way-to-represent-an-abstract-syntax-tree/9549/4

- Use just one symbol table datastructure that's usable for all intermediate representations: Ast, Icfg (cfg + dag)

## Machine code???

- MLIR: https://github.com/raviqqe/melior

- I should probalby use LLVM. Write my own backend for LLVM which means I have to learn LLVM syntax (it would be cool if the language could both be interpreted and compiled :))