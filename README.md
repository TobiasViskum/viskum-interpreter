# Viskum Interpreter

## Useful links

- SSA: https://en.wikipedia.org/wiki/Static_single-assignment_form
- CFG: https://en.wikipedia.org/wiki/Control-flow_graph
  - Graph: https://en.wikipedia.org/wiki/Graph_(abstract_data_type)
  - https://groups.seas.harvard.edu/courses/cs153/2018fa/lectures/Lec17-CFG-dataflow.pdf
- Constant folding: https://en.wikipedia.org/wiki/Constant_folding
- SCCP: https://en.wikipedia.org/wiki/Sparse_conditional_constant_propagation (Constant folding w/ SSA)
- DCE: https://en.wikipedia.org/wiki/Dead-code_elimination

## TODO

- Dead code analysis also have to scan for unnused variables

- Remove: PrecAssignment and other useless precedences (probably the only one) because '=' or ':=' is not part of a "expresison" but more of a way to structure an assignment statement.

- AST environment: Rename to Symbol Table

- Fix "unnused" expressions like: 1 + 2 will permanently take up a register. That should be detected in the control-flow graph and therefore the register should be released or "unnused" expression should be ignored.

- Fix that this: "bool i32 c := 2" is actually valid syntax. It should "emit" the types as soon as it sees one (right now it's skipping them, and then when the identifier comes it resolves the types)
  - Fix that this "i32 := 2" produces the correct error (i32 is a keyword and cannot be used as a definition target)

- FIX: Right now the jmp instructions in the scope is after end scope and jumps to start scope meaning new scopes are created all the time. The jump instructions should be between the StartScpe and EndScope

## Machine code???

- MLIR: https://github.com/raviqqe/melior