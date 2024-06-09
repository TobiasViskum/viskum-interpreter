use crate::{
    ast::ast_symbol_table::AstSymbolTable,
    compiler::cfg::dag::{ DAGNode, DAGOp, DAG },
    error_handler::{ CompileError, InternalError, InternalErrorCode, SrcCharsRange },
    operations::UnaryOp,
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
pub struct UnaryExpr {
    op: UnaryOp,
    rhs: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: UnaryOp, rhs: Expr) -> Self {
        Self {
            op,
            rhs: Box::new(rhs),
        }
    }

    pub fn get_op(&self) -> UnaryOp {
        self.op
    }

    pub fn get_rhs(&self) -> &Expr {
        &self.rhs
    }
}

impl Dissasemble for UnaryExpr {
    fn dissasemble(&self) -> String {
        format!("{}{}", self.op.dissasemble(), self.rhs.dissasemble())
    }
}

impl ExprTrait for UnaryExpr {
    fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
        let (value, mut src_chars_range) = self.rhs.evaluate(ast_symbol_table)?;
        src_chars_range.dec_char_by(self.op.get_op_len());

        let new_value = match self.op {
            UnaryOp::Truthy =>
                match value.not() {
                    Ok(value) => { value }
                    Err(msg) => {
                        return Err(
                            ExprEvaluateErr::CompileError(CompileError::new(msg, src_chars_range))
                        );
                    }
                }
            UnaryOp::Neg => {
                match value.neg() {
                    Ok(value) => { value }
                    Err(msg) => {
                        return Err(
                            ExprEvaluateErr::CompileError(CompileError::new(msg, src_chars_range))
                        );
                    }
                }
            }
            UnaryOp::Deref => {
                return Err(
                    ExprEvaluateErr::InternalError(
                        InternalError::new(
                            InternalErrorCode::Compile(2),
                            "Cannot evaluate a deref expression"
                        )
                    )
                );
            }
            UnaryOp::Ref => {
                return Err(
                    ExprEvaluateErr::InternalError(
                        InternalError::new(
                            InternalErrorCode::Compile(0),
                            "Cannot evaluate an immutable reference"
                        )
                    )
                );
            }
            UnaryOp::MutRef => {
                return Err(
                    ExprEvaluateErr::InternalError(
                        InternalError::new(
                            InternalErrorCode::Compile(1),
                            "Cannot evaluate a mutable reference"
                        )
                    )
                );
            }
        };

        Ok((new_value, src_chars_range))
    }

    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
        let rhs_type = self.rhs.type_check_and_constant_fold(ast_symbol_table)?;

        let can_constant_fold = rhs_type.get_can_constant_fold();

        match rhs_type.value_type.type_check_unary(self.op) {
            Ok(v) => Ok(ExprResultOk { value_type: v, can_constant_fold }),
            Err(msg) => {
                let metadata = self.collect_metadata();
                Err(ExprResultErr::new(vec![CompileError::new(msg, metadata)]))
            }
        }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let right_dag = self.rhs.compile_to_dag_node(dag);

        let op = DAGOp::UnaryOp(self.op);

        let dag_node = DAGNode::new(op, Some(vec![right_dag]));

        dag.add_node(dag_node)
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.rhs.collect_metadata()
    }
}
