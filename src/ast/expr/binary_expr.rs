use crate::{
    ast::ast_symbol_table::AstSymbolTable,
    compiler::cfg::dag::{ DAGNode, DAGOp, DAG },
    error_handler::{ CompileError, SrcCharsRange },
    merge_chars_range,
    operations::{ BinaryOp, ComparisonOp },
    parser::token::TokenMetadata,
    value::{ Value, ValueType },
    Dissasemble,
};

use super::{
    Expr,
    ExprEvaluateErr,
    ExprEvaluateResult,
    ExprResult,
    ExprResultErr,
    ExprResultOk,
    ExprTrait,
    LiteralExpr,
};

#[derive(Debug)]
pub struct BinaryExpr {
    lhs: Box<Expr>,
    op: BinaryOp,
    rhs: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(lhs: Expr, op: BinaryOp, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }

    pub fn get_lhs(&self) -> &Expr {
        &self.lhs
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }

    pub fn get_rhs(&self) -> &Expr {
        &self.rhs
    }
}

impl Dissasemble for BinaryExpr {
    fn dissasemble(&self) -> String {
        format!("{} {} {}", self.lhs.dissasemble(), self.op.dissasemble(), self.rhs.dissasemble())
    }
}

impl ExprTrait for BinaryExpr {
    fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
        let (lhs_value, lhs_src_chars) = self.lhs.evaluate(ast_symbol_table)?;
        let (rhs_value, rhs_src_chars_range) = self.rhs.evaluate(ast_symbol_table)?;
        let mut src_chars_range = lhs_src_chars;
        src_chars_range.merge(&rhs_src_chars_range);

        let new_value = (match self.op {
            BinaryOp::Add => {
                match lhs_value.add(&rhs_value) {
                    Ok(v) => Ok(v),
                    Err(msg) =>
                        Err(ExprEvaluateErr::CompileError(CompileError::new(msg, src_chars_range))),
                }
            }
            BinaryOp::Sub => {
                match lhs_value.sub(&rhs_value) {
                    Ok(v) => Ok(v),
                    Err(msg) =>
                        Err(ExprEvaluateErr::CompileError(CompileError::new(msg, src_chars_range))),
                }
            }
            BinaryOp::Mul => {
                match lhs_value.mul(&rhs_value) {
                    Ok(v) => Ok(v),
                    Err(msg) =>
                        Err(ExprEvaluateErr::CompileError(CompileError::new(msg, src_chars_range))),
                }
            }
            BinaryOp::Div => {
                match lhs_value.mul(&rhs_value) {
                    Ok(v) => Ok(v),
                    Err(msg) =>
                        Err(ExprEvaluateErr::CompileError(CompileError::new(msg, src_chars_range))),
                }
            }
            BinaryOp::ComparisonOp(comparison_op) => {
                match comparison_op {
                    ComparisonOp::Equal => {
                        match lhs_value.cmp_eq(&rhs_value) {
                            Ok(bool) => Ok(bool),
                            Err(msg) =>
                                Err(
                                    ExprEvaluateErr::CompileError(
                                        CompileError::new(msg, src_chars_range)
                                    )
                                ),
                        }
                    }
                    ComparisonOp::NotEqual => {
                        match lhs_value.cmp_ne(&rhs_value) {
                            Ok(bool) => Ok(bool),
                            Err(msg) =>
                                Err(
                                    ExprEvaluateErr::CompileError(
                                        CompileError::new(msg, src_chars_range)
                                    )
                                ),
                        }
                    }
                    ComparisonOp::Greater => {
                        match lhs_value.cmp_gt(&rhs_value) {
                            Ok(bool) => Ok(bool),
                            Err(msg) =>
                                Err(
                                    ExprEvaluateErr::CompileError(
                                        CompileError::new(msg, src_chars_range)
                                    )
                                ),
                        }
                    }
                    ComparisonOp::GreaterEqual => {
                        match lhs_value.cmp_ge(&rhs_value) {
                            Ok(bool) => Ok(bool),
                            Err(msg) =>
                                Err(
                                    ExprEvaluateErr::CompileError(
                                        CompileError::new(msg, src_chars_range)
                                    )
                                ),
                        }
                    }
                    ComparisonOp::Less => {
                        match lhs_value.cmp_lt(&rhs_value) {
                            Ok(bool) => Ok(bool),
                            Err(msg) =>
                                Err(
                                    ExprEvaluateErr::CompileError(
                                        CompileError::new(msg, src_chars_range)
                                    )
                                ),
                        }
                    }
                    ComparisonOp::LessEqual => {
                        match lhs_value.cmp_le(&rhs_value) {
                            Ok(bool) => Ok(bool),
                            Err(msg) =>
                                Err(
                                    ExprEvaluateErr::CompileError(
                                        CompileError::new(msg, src_chars_range)
                                    )
                                ),
                        }
                    }
                }
            }
        })?;

        Ok((new_value, src_chars_range))
    }

    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
        let lhs_expr_result = self.lhs.type_check_and_constant_fold(ast_symbol_table)?;
        let rhs_expr_result = self.rhs.type_check_and_constant_fold(ast_symbol_table)?;

        if lhs_expr_result.get_can_constant_fold() {
            let (new_value, src_chars_range) = self.lhs.evaluate(ast_symbol_table)?;
            *self.lhs = Expr::LiteralExpr(LiteralExpr::new(new_value, src_chars_range.into()));
        }
        if rhs_expr_result.get_can_constant_fold() {
            let (new_value, src_chars_range) = self.rhs.evaluate(ast_symbol_table)?;
            *self.rhs = Expr::LiteralExpr(LiteralExpr::new(new_value, src_chars_range.into()));
        }

        let can_constant_fold =
            lhs_expr_result.get_can_constant_fold() && rhs_expr_result.get_can_constant_fold();

        match
            lhs_expr_result
                .take_value_type()
                .type_check_binary(&rhs_expr_result.take_value_type(), self.op)
        {
            Ok(v) => Ok(ExprResultOk::new(v, can_constant_fold)),
            Err(msg) => {
                let metadata = self.collect_metadata();
                Err(ExprResultErr::new(vec![CompileError::new(msg, metadata)]))
            }
        }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let left_dag = self.lhs.compile_to_dag_node(dag);
        let right_dag = self.rhs.compile_to_dag_node(dag);

        let op = DAGOp::BinaryOp(self.op);

        let dag_node = DAGNode::new(op, Some(vec![left_dag, right_dag]));

        dag.add_node(dag_node)
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        let left_metadata = self.lhs.collect_metadata();
        let right_metadata = self.rhs.collect_metadata();

        let metadata = merge_chars_range!(left_metadata, right_metadata);

        metadata
    }
}
