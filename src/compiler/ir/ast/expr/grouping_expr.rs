use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::ValueType },
    error_handler::{ CompileError, SrcCharsRange },
    ir::icfg::dag::DAG,
    Dissasemble,
};

use super::{ Expr, ExprTrait };

#[derive(Debug)]
pub struct GroupingExpr<'ast> {
    expr: &'ast mut Expr<'ast>,
}

impl<'ast> GroupingExpr<'ast> {
    pub fn new(expr: &'ast mut Expr<'ast>) -> Self {
        Self {
            expr,
        }
    }

    pub fn get_expr(&mut self) -> &'ast mut Expr {
        self.expr
    }
}

impl<'ast> Dissasemble for GroupingExpr<'ast> {
    fn dissasemble(&self) -> String {
        unsafe { format!("({})", (*self.expr).dissasemble()) }
    }
}

impl<'ast> ExprTrait for GroupingExpr<'ast> {
    fn collect_metadata(&self) -> SrcCharsRange {
        unsafe {
            let mut metadata = (*self.expr).collect_metadata();
            metadata.dec_char_by(1);
            metadata.inc_char_by(1);
            metadata
        }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        todo!()
    }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        unsafe { (*self.expr).type_check(symbol_table_ref) }
    }

    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
    //     unsafe { (*self.expr).evaluate(ast_symbol_table) }
    // }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     unsafe { (*self.expr).type_check_and_constant_fold(ast_symbol_table) }
    // }
}
