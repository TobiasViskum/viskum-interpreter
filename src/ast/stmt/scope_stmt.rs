use crate::{ ast::ast_symbol_table::AstSymbolTable, error_handler::ErrorHandler, Dissasemble };

use super::{ Stmt, StmtTrait, Stmts };

#[derive(Debug)]
pub struct ScopeStmt {
    stmts: Stmts,
    forwards_declarations: Stmts, // TypeDefStmt, FnStmt, (ClassStmt)
}

impl ScopeStmt {
    pub fn new() -> Self {
        Self {
            stmts: Stmts::new(),
            forwards_declarations: Stmts::new(),
        }
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn push_forward_stmt(&mut self, stmt: Stmt) {
        self.forwards_declarations.push(stmt)
    }
}

impl Dissasemble for ScopeStmt {
    fn dissasemble(&self) -> String {
        let mut string_builder = self.forwards_declarations.dissasemble();
        string_builder += self.stmts.dissasemble().as_str();
        string_builder
    }
}

impl StmtTrait for ScopeStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        self.forwards_declarations.validate_stmts(ast_symbol_table, error_handler);
        self.stmts.validate_stmts(ast_symbol_table, error_handler);
    }
}
