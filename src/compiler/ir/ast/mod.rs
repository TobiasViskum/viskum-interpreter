use std::fmt::Debug;

use expr::Expr;
use stmt::{ GotoNodeIds, IfStmt, BasicBlockStmt, Stmt };
use typed_arena::Arena;

use crate::compiler::{
    error_handler::ErrorHandler,
    ir::icfg::icfg_builder::ICFGBuilder,
    traits::{ Dissasemble, StmtTrait },
};

use super::icfg::{ cfg::CFG, ICFG };

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

    pub fn alloc_expr(&self, expr: Expr<'ast>) -> &'ast mut Expr {
        let allocated_expr = self.arena.alloc(AstArenaItem::Expr(expr));

        match allocated_expr {
            AstArenaItem::Expr(expr) => expr,
            _ => panic!("Expected expression in alloc_expr"),
        }
    }

    pub fn alloc_if_stmt(&self, if_stmt: IfStmt<'ast>) -> &'ast mut IfStmt {
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
    main_scope: BasicBlockStmt<'ast>,
}

impl<'ast> Ast<'ast> {
    pub fn new(main_scope: BasicBlockStmt<'ast>) -> Self {
        Self {
            main_scope,
        }
    }

    pub fn construct_icfg(self) -> ICFG {
        let mut icfg_builder = ICFGBuilder::new();
        let mut goto_node_ids = GotoNodeIds::new();
        self.main_scope.compile_into_icfg(&mut icfg_builder, &mut goto_node_ids);

        icfg_builder.take_icfg()
    }

    pub fn type_check_and_constant_fold(&mut self, error_handler: &mut ErrorHandler) {
        println!("1");

        self.main_scope.validate_stmt(&mut self.main_scope.get_symbol_table_ref(), error_handler);
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
