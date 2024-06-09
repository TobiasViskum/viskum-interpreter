use crate::{ ast::ast_symbol_table::AstSymbolTable, error_handler::ErrorHandler, Dissasemble };

use super::{ ExprStmt, StmtTrait };

#[derive(Debug)]
pub struct VarAssignStmt {
    pub target_expr: ExprStmt,
    pub value: ExprStmt,
}

impl VarAssignStmt {
    pub fn new(target_expr: ExprStmt, value: ExprStmt) -> Self {
        Self {
            target_expr,
            value,
        }
    }
}

impl Dissasemble for VarAssignStmt {
    fn dissasemble(&self) -> String {
        format!("{} = {}\n", self.target_expr.dissasemble(), self.value.dissasemble())
    }
}

impl StmtTrait for VarAssignStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        ast_symbol_table.assign_variable(self, error_handler)
    }
}
