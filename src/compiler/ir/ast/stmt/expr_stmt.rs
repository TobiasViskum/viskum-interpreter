use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::ValueType },
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    ir::{ ast::expr::{ Expr, ExprTrait }, icfg::dag::DAG },
    Dissasemble,
};

use super::{ LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct ExprStmt<'ast> {
    expr: Expr<'ast>,
}

impl<'ast> ExprStmt<'ast> {
    pub fn new(expr: Expr<'ast>) -> Self {
        Self { expr }
    }

    pub fn get_expr<'b>(&'b self) -> &'b Expr<'ast> {
        &self.expr
    }

    // pub fn get_value_type(&mut self, error_handler: &mut ErrorHandler) -> Option<ValueType> {
    //     match self.type_check_and_constant_fold(ast_symbol_table) {}
    // }
}

impl<'ast> ExprTrait for ExprStmt<'ast> {
    fn collect_metadata(&self) -> SrcCharsRange {
        unsafe { self.expr.collect_metadata() }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        unsafe { self.expr.compile_to_dag_node(dag) }
    }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        unsafe { self.expr.type_check(symbol_table_ref) }
    }

    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
    //     let (new_value, src_chars_range) = unsafe { (*self.expr).evaluate(ast_symbol_table)? };

    //     unsafe {
    //         *self.expr = Expr::LiteralExpr(
    //             LiteralExpr::new(new_value.clone(), src_chars_range.into())
    //         );
    //     }

    //     Ok((new_value, src_chars_range))
    // }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     let expr_result_ok = unsafe {
    //         (*self.expr).type_check_and_constant_fold(ast_symbol_table)?
    //     };

    //     if expr_result_ok.get_can_constant_fold() {
    //         self.evaluate(ast_symbol_table)?;
    //     }

    //     Ok(expr_result_ok)
    // }
}

impl<'ast> StmtTrait for ExprStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        unsafe {
            match self.expr.type_check(&symbol_table_ref) {
                Ok(_) => {}
                Err(err) => error_handler.report_compile_error(err),
            }
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl<'ast> Dissasemble for ExprStmt<'ast> {
    fn dissasemble(&self) -> String {
        unsafe { self.expr.dissasemble() }
    }
}
