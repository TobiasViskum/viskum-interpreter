use crate::{
    ast::{
        ast_symbol_table::AstSymbolTable,
        expr::{ Expr, ExprEvaluateResult, ExprResult, ExprTrait, LiteralExpr },
    },
    compiler::cfg::dag::DAG,
    error_handler::{ ErrorHandler, SrcCharsRange },
    value::Value,
    Dissasemble,
};

use super::StmtTrait;

#[derive(Debug)]
pub struct ExprStmt {
    expr: Expr,
}

impl ExprStmt {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn get_expr(&self) -> &Expr {
        &self.expr
    }
}

impl ExprTrait for ExprStmt {
    fn collect_metadata(&self) -> SrcCharsRange {
        self.expr.collect_metadata()
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        self.expr.compile_to_dag_node(dag)
    }

    fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
        let (new_value, src_chars_range) = self.expr.evaluate(ast_symbol_table)?;

        self.expr = Expr::LiteralExpr(LiteralExpr::new(new_value, src_chars_range.into()));

        Ok((Value::Void, src_chars_range))
    }

    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
        let expr_result_ok = self.expr.type_check_and_constant_fold(ast_symbol_table)?;
        if expr_result_ok.get_can_constant_fold() {
            self.evaluate(ast_symbol_table)?;
        }
        Ok(expr_result_ok)
    }
}

impl StmtTrait for ExprStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        match self.expr.type_check_and_constant_fold(ast_symbol_table) {
            Ok(_) => {}
            Err(expr_result_err) =>
                error_handler.report_many_compile_errors(expr_result_err.take_errors()),
        }
    }
}

impl Dissasemble for ExprStmt {
    fn dissasemble(&self) -> String {
        self.expr.dissasemble()
    }
}
