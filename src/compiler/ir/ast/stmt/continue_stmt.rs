use crate::compiler::{ ds::symbol_table::SymbolTableRef, error_handler::ErrorHandler, Dissasemble };

use super::{ LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct ContinueStmt;

impl ContinueStmt {
    pub fn new() -> Self {
        Self
    }
}

impl StmtTrait for ContinueStmt {
    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        todo!()
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl Dissasemble for ContinueStmt {
    fn dissasemble(&self) -> String {
        "continue".to_string()
    }
}
