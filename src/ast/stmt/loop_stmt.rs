use crate::Dissasemble;

use super::{ ExprStmt, ScopeStmt, StmtTrait };

#[derive(Debug)]
pub struct LoopStmt {
    condition: Option<ExprStmt>,
    body: ScopeStmt,
}

impl LoopStmt {
    pub fn new(condition: Option<ExprStmt>, body: ScopeStmt) -> Self {
        Self { condition, body }
    }
}

impl Dissasemble for LoopStmt {
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

impl StmtTrait for LoopStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut crate::ast::ast_symbol_table::AstSymbolTable,
        error_handler: &mut crate::error_handler::ErrorHandler
    ) {
        if let Some(condition) = &mut self.condition {
            condition.validate_stmt(ast_symbol_table, error_handler);
        }
        self.body.validate_stmt(ast_symbol_table, error_handler)
    }
}
