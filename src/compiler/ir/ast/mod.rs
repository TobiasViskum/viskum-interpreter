use std::fmt::Debug;

use expr::Expr;
use stmt::{ BlockStmt, ExprStmt, GotoNodeIds, IfStmt, Stmt };
use typed_arena::Arena;

use crate::compiler::{
    ds::ssa_ident::SSAIdent,
    error_handler::ErrorHandler,
    traits::{ AstDissasemble, Dissasemble, StmtTrait },
    ProgramSymbolTablePhase1,
};

use super::icfg::{ cfg::CFG, icfg_builder::{ CFGBuilder, ICFGBuilder }, ICFG };

pub mod expr;
pub mod stmt;

pub const AST_DISSASEMBLE_INDENTATION: usize = 4;

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
    main_scope: BlockStmt<'ast>,
}

impl<'ast> Ast<'ast> {
    pub fn new(main_scope: BlockStmt<'ast>) -> Self {
        Self { main_scope }
    }

    pub fn construct_icfg(self) -> ICFG {
        let mut goto_node_ids = GotoNodeIds::new();

        let mut icfg_builder = ICFGBuilder::new();
        let mut cfg_builder = CFGBuilder::new_global();
        self.main_scope.compile_into_icfg(&mut icfg_builder, &mut cfg_builder, &mut goto_node_ids);

        let cfg = cfg_builder.end_entry_cfg();
        let entry_cfg_id = icfg_builder.push_cfg(cfg);

        icfg_builder.set_entry_cfg(entry_cfg_id);

        icfg_builder.take_cfg()
    }

    fn expect_main_function(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        if let Err(compile_err) = program_symbol_table.expect_main_function() {
            error_handler.report_compile_error(compile_err);
            return;
        }

        let main_fn_stmt = self.main_scope
            .iter_mut_stmts()
            .filter_map(|stmt| {
                if let Stmt::FunctionStmt(fn_stmt) = stmt { Some(fn_stmt) } else { None }
            })
            .find(|fn_stmt| {
                if fn_stmt.get_ssa_ident().is("main", 1) && fn_stmt.get_args().len() == 0 {
                    true
                } else {
                    false
                }
            })
            .unwrap();

        main_fn_stmt.get_mut_ssa_ident().set_subscript_to_zero();
    }

    pub fn type_check_and_constant_fold(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        self.main_scope.validate_stmt(program_symbol_table, error_handler);
        self.expect_main_function(program_symbol_table, error_handler);
    }

    pub fn print(&self) {
        let mut string_builder = "\n----- AST -----\n\n".to_string();
        string_builder += self.dissasemble().as_str();
        string_builder += "\n---------------\n";
        println!("{}", string_builder);
    }

    pub fn ast_print(&self, program_symbol_table: &mut ProgramSymbolTablePhase1) {
        let mut string_builder = "\n----- AST -----\n\n".to_string();
        string_builder += "Symbol table:\n\n";
        string_builder += program_symbol_table.dissasemble().as_str();
        string_builder += "\n";
        string_builder += self.ast_dissasemble(program_symbol_table, 0).as_str();
        string_builder += "\n---------------\n";
        program_symbol_table.set_current_symbol_table_id(0);
        println!("{}", string_builder);
    }
}

impl<'ast> AstDissasemble for Ast<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        let mut string_builder = String::new();
        for stmt in self.main_scope.iter_stmts() {
            string_builder += stmt.ast_dissasemble(program_symbol_table, scope_depth).as_str();
        }
        string_builder
    }
}

impl<'ast> Dissasemble for Ast<'ast> {
    fn dissasemble(&self) -> String {
        self.main_scope.dissasemble()
    }
}
