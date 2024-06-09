use crate::{ ast::ast_symbol_table::AstSymbolTable, error_handler::ErrorHandler, Dissasemble };

use super::StmtTrait;

#[derive(Debug)]
pub struct ContinueStmt;

impl ContinueStmt {
    pub fn new() -> Self {
        Self
    }
}

impl StmtTrait for ContinueStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        todo!()
    }
}

impl Dissasemble for ContinueStmt {
    fn dissasemble(&self) -> String {
        "continue".to_string()
    }
}
