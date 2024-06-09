use std::rc::Rc;

use crate::{
    error_handler::ErrorHandler,
    parser::{ self, token::{ Token, TokenMetadata } },
    value::ValueType,
    Dissasemble,
};
use super::ast_symbol_table::{ self, AstSymbolTable };

mod expr_stmt;
mod break_stmt;
mod var_def_stmt;
mod var_assign_stmt;
mod scope_stmt;
mod continue_stmt;
mod return_stmt;
mod if_stmt;
mod loop_stmt;
mod fn_stmt;

pub use expr_stmt::ExprStmt;
pub use break_stmt::BreakStmt;
pub use var_def_stmt::VarDefStmt;
pub use var_assign_stmt::VarAssignStmt;
pub use scope_stmt::ScopeStmt;
pub use continue_stmt::ContinueStmt;
pub use return_stmt::ReturnStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use fn_stmt::FunctionStmt;

pub trait StmtTrait {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    );
}

#[derive(Debug)]
pub struct Stmts {
    stmts: Vec<Stmt>,
}

impl Dissasemble for Stmts {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("");
        for stmt in &self.stmts {
            string_builder += stmt.dissasemble().as_str();
        }
        string_builder
    }
}

impl Stmts {
    pub fn new() -> Self {
        Self {
            stmts: Vec::new(),
        }
    }

    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Stmt> {
        self.stmts.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Stmt> {
        self.stmts.iter_mut()
    }

    pub fn validate_stmts(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        for stmt in self.iter_mut() {
            stmt.validate_stmt(ast_symbol_table, error_handler);
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(ExprStmt),
    VarDefStmt(VarDefStmt),
    VarAssignStmt(VarAssignStmt),
    ScopeStmt(ScopeStmt),
    FunctionStmt(FunctionStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ReturnStmt(ReturnStmt),
    IfStmt(IfStmt),
    LoopStmt(LoopStmt),
}

impl Dissasemble for Stmt {
    fn dissasemble(&self) -> String {
        match self {
            Self::ExprStmt(expr_stmt) => format!("{}\n", expr_stmt.dissasemble()),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.dissasemble(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.dissasemble(),
            Self::ScopeStmt(scope_stmt) => scope_stmt.dissasemble(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.dissasemble(),
            Self::BreakStmt(break_stmt) => break_stmt.dissasemble(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.dissasemble(),
            Self::ReturnStmt(return_stmt) => return_stmt.dissasemble(),
            Self::IfStmt(if_stmt) => if_stmt.dissasemble(),
            Self::LoopStmt(loop_stmt) => loop_stmt.dissasemble(),
        }
    }
}

impl StmtTrait for Stmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::VarDefStmt(var_def_stmt) =>
                var_def_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::VarAssignStmt(var_assign_stmt) =>
                var_assign_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::ScopeStmt(scope_stmt) => {
                ast_symbol_table.increment_scope();
                scope_stmt.validate_stmt(ast_symbol_table, error_handler);
                ast_symbol_table.decrement_scope();
            }
            Self::FunctionStmt(fn_stmt) => fn_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::BreakStmt(break_stmt) =>
                break_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::ContinueStmt(continue_stmt) =>
                continue_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::ReturnStmt(return_stmt) =>
                return_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::IfStmt(if_stmt) => if_stmt.validate_stmt(ast_symbol_table, error_handler),
            Self::LoopStmt(loop_stmt) => loop_stmt.validate_stmt(ast_symbol_table, error_handler),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: Rc<str>,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub metadata: TokenMetadata,
}
