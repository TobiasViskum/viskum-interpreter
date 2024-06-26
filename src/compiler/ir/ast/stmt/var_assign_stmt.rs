use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::dag::DAG,
    Dissasemble,
};

use super::{ ExprStmt, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct VarAssignStmt<'ast> {
    target_expr: ExprStmt<'ast>,
    value: ExprStmt<'ast>,
}

impl<'ast> VarAssignStmt<'ast> {
    pub fn new(target_expr: ExprStmt<'ast>, value: ExprStmt<'ast>) -> Self {
        Self {
            target_expr,
            value,
        }
    }

    pub fn get_target_expr(&self) -> &ExprStmt<'ast> {
        &self.target_expr
    }

    pub fn get_mut_value_expr(&mut self) -> &mut ExprStmt<'ast> {
        &mut self.value
    }

    pub fn get_value_expr(&mut self) -> &ExprStmt<'ast> {
        &self.value
    }
}

impl<'ast> Dissasemble for VarAssignStmt<'ast> {
    fn dissasemble(&self) -> String {
        format!("{} = {}\n", self.target_expr.dissasemble(), self.value.dissasemble())
    }
}

impl<'ast> StmtTrait for VarAssignStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match symbol_table_ref.get_mut().assing_var(self) {
            Ok(_) => {}
            Err(err) => error_handler.report_compile_error(err),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl<'ast> LinearControlFlow for VarAssignStmt<'ast> {
    fn compile_into_dag(&self, dag: &mut DAG) {
        todo!()
    }
}
