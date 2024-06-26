use crate::{
    compiler::{
        ds::{ symbol_table::SymbolTableRef, value::{ ops::BinaryOp, ValueType } },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::icfg::dag::DAG,
        Dissasemble,
    },
    macros::merge_chars_range,
};

use super::{ Expr, ExprTrait };

#[derive(Debug)]
pub struct BinaryExpr<'ast> {
    lhs: &'ast mut Expr<'ast>,
    op: BinaryOp,
    rhs: &'ast mut Expr<'ast>,
}

impl<'ast> BinaryExpr<'ast> {
    pub fn new(lhs: &'ast mut Expr<'ast>, op: BinaryOp, rhs: &'ast mut Expr<'ast>) -> Self {
        Self {
            lhs,
            op,
            rhs,
        }
    }

    pub fn get_lhs(&mut self) -> &'ast mut Expr {
        self.lhs
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }

    pub fn get_rhs(&mut self) -> &'ast mut Expr {
        self.rhs
    }
}

impl<'ast> Dissasemble for BinaryExpr<'ast> {
    fn dissasemble(&self) -> String {
        unsafe {
            format!(
                "{} {} {}",
                self.lhs.dissasemble(),
                self.op.dissasemble(),
                self.rhs.dissasemble()
            )
        }
    }
}

impl<'ast> ExprTrait for BinaryExpr<'ast> {
    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
    //     let (lhs_value, lhs_src_chars) = unsafe { (*self.lhs).evaluate(ast_symbol_table)? };
    //     let (rhs_value, rhs_src_chars_range) = unsafe { (*self.rhs).evaluate(ast_symbol_table)? };

    //     let mut src_chars_range = lhs_src_chars;
    //     src_chars_range.merge(&rhs_src_chars_range);

    //     let new_value = (match self.op {
    //         BinaryOp::Add => {
    //             match lhs_value.add(&rhs_value) {
    //                 Ok(v) => Ok(v),
    //                 Err(msg) =>
    //                     Err(
    //                         ExprEvaluateErr::CompileError(
    //                             CompileError::new(ReportedError::new(msg, src_chars_range))
    //                         )
    //                     ),
    //             }
    //         }
    //         BinaryOp::Sub => {
    //             match lhs_value.sub(&rhs_value) {
    //                 Ok(v) => Ok(v),
    //                 Err(msg) =>
    //                     Err(
    //                         ExprEvaluateErr::CompileError(
    //                             CompileError::new(ReportedError::new(msg, src_chars_range))
    //                         )
    //                     ),
    //             }
    //         }
    //         BinaryOp::Mul => {
    //             match lhs_value.mul(&rhs_value) {
    //                 Ok(v) => Ok(v),
    //                 Err(msg) =>
    //                     Err(
    //                         ExprEvaluateErr::CompileError(
    //                             CompileError::new(ReportedError::new(msg, src_chars_range))
    //                         )
    //                     ),
    //             }
    //         }
    //         BinaryOp::Div => {
    //             match lhs_value.mul(&rhs_value) {
    //                 Ok(v) => Ok(v),
    //                 Err(msg) =>
    //                     Err(
    //                         ExprEvaluateErr::CompileError(
    //                             CompileError::new(ReportedError::new(msg, src_chars_range))
    //                         )
    //                     ),
    //             }
    //         }
    //         BinaryOp::ComparisonOp(comparison_op) => {
    //             match comparison_op {
    //                 ComparisonOp::Eq => {
    //                     match lhs_value.cmp_eq(&rhs_value) {
    //                         Ok(bool) => Ok(bool),
    //                         Err(msg) =>
    //                             Err(
    //                                 ExprEvaluateErr::CompileError(
    //                                     CompileError::new(ReportedError::new(msg, src_chars_range))
    //                                 )
    //                             ),
    //                     }
    //                 }
    //                 ComparisonOp::Ne => {
    //                     match lhs_value.cmp_ne(&rhs_value) {
    //                         Ok(bool) => Ok(bool),
    //                         Err(msg) =>
    //                             Err(
    //                                 ExprEvaluateErr::CompileError(
    //                                     CompileError::new(ReportedError::new(msg, src_chars_range))
    //                                 )
    //                             ),
    //                     }
    //                 }
    //                 ComparisonOp::Gt => {
    //                     match lhs_value.cmp_gt(&rhs_value) {
    //                         Ok(bool) => Ok(bool),
    //                         Err(msg) =>
    //                             Err(
    //                                 ExprEvaluateErr::CompileError(
    //                                     CompileError::new(ReportedError::new(msg, src_chars_range))
    //                                 )
    //                             ),
    //                     }
    //                 }
    //                 ComparisonOp::Ge => {
    //                     match lhs_value.cmp_ge(&rhs_value) {
    //                         Ok(bool) => Ok(bool),
    //                         Err(msg) =>
    //                             Err(
    //                                 ExprEvaluateErr::CompileError(
    //                                     CompileError::new(ReportedError::new(msg, src_chars_range))
    //                                 )
    //                             ),
    //                     }
    //                 }
    //                 ComparisonOp::Lt => {
    //                     match lhs_value.cmp_lt(&rhs_value) {
    //                         Ok(bool) => Ok(bool),
    //                         Err(msg) =>
    //                             Err(
    //                                 ExprEvaluateErr::CompileError(
    //                                     CompileError::new(ReportedError::new(msg, src_chars_range))
    //                                 )
    //                             ),
    //                     }
    //                 }
    //                 ComparisonOp::Le => {
    //                     match lhs_value.cmp_le(&rhs_value) {
    //                         Ok(bool) => Ok(bool),
    //                         Err(msg) =>
    //                             Err(
    //                                 ExprEvaluateErr::CompileError(
    //                                     CompileError::new(ReportedError::new(msg, src_chars_range))
    //                                 )
    //                             ),
    //                     }
    //                 }
    //             }
    //         }
    //     })?;

    //     Ok((new_value, src_chars_range))
    // }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     let (lhs_expr_result, rhs_expr_result) = unsafe {
    //         (
    //             (*self.lhs).type_check_and_constant_fold(ast_symbol_table)?,
    //             (*self.rhs).type_check_and_constant_fold(ast_symbol_table)?,
    //         )
    //     };

    //     if lhs_expr_result.get_can_constant_fold() {
    //         unsafe {
    //             let (new_value, src_chars_range) = (*self.lhs).evaluate(ast_symbol_table)?;
    //             *self.lhs = Expr::LiteralExpr(LiteralExpr::new(new_value, src_chars_range.into()));
    //         }
    //     }
    //     if rhs_expr_result.get_can_constant_fold() {
    //         unsafe {
    //             let (new_value, src_chars_range) = (*self.rhs).evaluate(ast_symbol_table)?;
    //             *self.rhs = Expr::LiteralExpr(LiteralExpr::new(new_value, src_chars_range.into()));
    //         }
    //     }

    //     let can_constant_fold =
    //         lhs_expr_result.get_can_constant_fold() &&
    //         rhs_expr_result.get_can_constant_fold() &&
    //         self.op.get_can_constant_fold();

    //     let lhs_type = lhs_expr_result.take_value_type();
    //     let rhs_type = rhs_expr_result.take_value_type();

    //     match lhs_type.try_binary(&rhs_type, self.op) {
    //         Ok(v) => Ok(ExprResultOk::new(v, can_constant_fold)),
    //         Err(msg) => {
    //             let metadata = self.collect_metadata();
    //             let additional: &str;
    //             Err(ExprResultErr::new(vec![CompileError::new(ReportedError::new(msg, metadata))]))
    //         }
    //     }
    // }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        let (lhs_type, rhs_type) = unsafe {
            (self.lhs.type_check(symbol_table_ref)?, (*self.rhs).type_check(symbol_table_ref)?)
        };

        match lhs_type.try_binary(&rhs_type, self.op) {
            Ok(v) => Ok(v),
            Err(msg) => {
                let metadata = self.collect_metadata();
                Err(CompileError::new(ReportedError::new(msg, metadata)))
            }
        }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        todo!()
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        let (left_metadata, right_metadata) = unsafe {
            ((*self.lhs).collect_metadata(), (*self.rhs).collect_metadata())
        };

        let metadata = merge_chars_range!(left_metadata, right_metadata);

        metadata
    }
}
