use crate::compiler::{ ds::symbol_table::SymbolTableRef, error_handler::ErrorHandler, Dissasemble };

use super::{ ExprStmt, LinearControlFlow, ScopeStmt, StmtTrait };

#[derive(Debug)]
pub struct LoopStmt<'ast> {
    condition: Option<ExprStmt<'ast>>,
    body: ScopeStmt<'ast>,
}

impl<'ast> LoopStmt<'ast> {
    pub fn new(condition: Option<ExprStmt<'ast>>, body: ScopeStmt<'ast>) -> Self {
        Self { condition, body }
    }
}

impl<'ast> Dissasemble for LoopStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("");
        match &self.condition {
            Some(condition) => {
                string_builder += format!("while {} {{\n", condition.dissasemble()).as_str();
                string_builder += self.body.dissasemble().as_str();
                string_builder += "}\n";
            }
            None => {
                string_builder += "loop {\n";
                string_builder += self.body.dissasemble().as_str();
                string_builder += "}\n";
            }
        }
        string_builder
    }
}

impl<'ast> StmtTrait for LoopStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        if let Some(condition) = &mut self.condition {
            condition.validate_stmt(symbol_table_ref, error_handler);
        }
        self.body.validate_stmt(symbol_table_ref, error_handler)
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}
