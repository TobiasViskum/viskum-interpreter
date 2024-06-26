use std::fmt::Debug;

use expr::Expr;
use stmt::{ IfStmt, ScopeStmt, Stmt, StmtTrait };
use typed_arena::Arena;

use crate::compiler::{ error_handler::ErrorHandler, Dissasemble };

pub mod expr;
pub mod stmt;

enum AstArenaItem<'ast> {
    Expr(Expr<'ast>),
    Stmt(Stmt<'ast>),
}

pub struct AstArena<'ast> {
    arena: Arena<AstArenaItem<'ast>>,
}

impl<'ast> Debug for AstArena<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<AstArena>")
    }
}

impl<'ast> AstArena<'ast> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
        }
    }

    pub fn alloc_expr(&self, expr: Expr<'ast>) -> &'ast Expr {
        let allocated_expr = self.arena.alloc(AstArenaItem::Expr(expr));

        match allocated_expr {
            AstArenaItem::Expr(expr) => expr,
            _ => panic!("Expected expression in alloc_expr"),
        }
    }

    pub fn alloc_mut_expr(&self, expr: Expr<'ast>) -> &'ast mut Expr {
        let allocated_expr = self.arena.alloc(AstArenaItem::Expr(expr));

        match allocated_expr {
            AstArenaItem::Expr(expr) => expr,
            _ => panic!("Expected expression in alloc_expr"),
        }
    }

    pub fn alloc_if_stmt(&self, if_stmt: IfStmt<'ast>) -> &'ast IfStmt {
        let allocated_if_stmt = self.arena.alloc(AstArenaItem::Stmt(Stmt::IfStmt(if_stmt)));

        match allocated_if_stmt {
            AstArenaItem::Stmt(stmt) =>
                match stmt {
                    Stmt::IfStmt(if_stmt) => if_stmt,
                    _ => panic!("Expected if_stmt in alloc_if_stmt"),
                }
            _ => panic!("Expected stmt in alloc_if_stmt"),
        }
    }
}

#[derive(Debug)]
pub struct Ast<'ast> {
    // ast_arena: Option<AstArena<'ast>>,
    main_scope: ScopeStmt<'ast>,
}

impl<'ast> Ast<'ast> {
    pub fn new(main_scope: ScopeStmt<'ast>) -> Self {
        Self {
            // ast_arena: None,
            main_scope,
        }
    }

    pub fn construct_icfg(mut self) -> () {}

    pub fn type_check_and_constant_fold(&mut self, error_handler: &mut ErrorHandler) {
        self.main_scope.validate_stmt(&self.main_scope.get_symbol_table_ref(), error_handler);
    }

    pub fn print(&self) {
        let mut string_builder = "\n----- AST -----\n\n".to_string();
        string_builder += self.dissasemble().as_str();
        string_builder += "\n---------------\n";
        println!("{}", string_builder);
    }
}

impl<'ast> Dissasemble for Ast<'ast> {
    fn dissasemble(&self) -> String {
        self.main_scope.get_stmts().dissasemble()
    }
}
