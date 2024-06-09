use crate::{ ast::ast_symbol_table::AstSymbolTable, error_handler::ErrorHandler, Dissasemble };

use super::StmtTrait;

#[derive(Debug)]
pub struct BreakStmt;

impl BreakStmt {
    pub fn new() -> Self {
        Self
    }
}

impl StmtTrait for BreakStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        todo!()
    }
}

impl Dissasemble for BreakStmt {
    fn dissasemble(&self) -> String {
        "break".to_string()
    }
}
