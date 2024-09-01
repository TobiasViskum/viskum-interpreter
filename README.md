# Viskum Interpreter

## Another rewrite (hopefully last)
- Make it expression based. Right now if-stmt, loop-stmt, (match-stmt) and other stmts can't be used inside expressions.
- Make the rewrite while making the compiler self hosted

Stmt(
  ItemStmt(
    FnDeclarationStmt(),
    StructDeclarationStmt(),
    EnumDeclarationStmt(),
    ...
  ),
  DefStmt(
    IdentExpr, ExprStmt
  )
  BlockExpr(
    Vec<Stmt>
  )
  ExprStmt(
    ExprWithBlock(
      BlockExpr(),
      LoopExpr(),
      IfExpr(),
      IfDefExpr(),
      MatchExpr(),
    ),
    ExprWithoutBlock(
      Expr(
        PlaceExpr(
          PathExpr(),
          IdentExpr(),
          ArrayIndexExpr(),
          FieldExpr(),
          GroupExpr(),
          CallExpr()
        ),
        ValueExpr(
          ConstExpr(),
          CallExpr(),
          ...
        ),
        AsigneeExpr(
          PlaceExpr(),
          TupleExpr(),
        ),
        AsignExpr(
          AsigneeExpr, ExprStmt,
        )
      )
    )
  )
)

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

Other optimizations here:
https://en.wikipedia.org/wiki/Static_single-assignment_form#Benefits

## TODO

- Rewrite the way a directed acyclic graph works. All basic blocks (linear sequence of stmts that don't jump) should be in its own cfg node which is called the Process node. Right now each stmt gets its own node, which makes optimizations harder

- Dead code analysis also have to scan for unnused variables

- Remove: PrecAssignment and other useless precedences (probably the only one) because '=' or ':=' is not part of a "expresison" but more of a way to structure an assignment statement.

## Machine code???

- MLIR: https://github.com/raviqqe/melior

- I should probalby use LLVM. Write my own backend for LLVM which means I have to learn LLVM syntax (it would be cool if the language could both be interpreted and compiled :))



## Project structure conventions (still deciding)

- src/
    - main.rs
    - /big_struct
        - mod.rs (This imports all dependencies in struct, and controls visibility)
        - traits.rs
        - types
