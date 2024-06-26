use crate::compiler::{
    ds::{
        symbol_table::{ SymbolTableActions, SymbolTableRef },
        value::{ ops::UnaryOp, ValueType },
    },
    error_handler::{ CompileError, ReportedError, SrcCharsRange },
    ir::icfg::dag::DAG,
    Dissasemble,
};

use super::{ Expr, ExprTrait };

#[derive(Debug)]
pub struct UnaryExpr<'ast> {
    op: UnaryOp,
    rhs: &'ast mut Expr<'ast>,
}

impl<'ast> UnaryExpr<'ast> {
    pub fn new(op: UnaryOp, rhs: &'ast mut Expr<'ast>) -> Self {
        Self {
            op,
            rhs,
        }
    }

    pub fn get_op(&self) -> UnaryOp {
        self.op
    }

    pub fn get_rhs(&mut self) -> &'ast mut Expr {
        self.rhs
    }

    pub fn check_if_mutable_ref_to_immutable_var(
        &self,
        symbol_table_ref: &SymbolTableRef
    ) -> Result<(), CompileError> {
        match (unsafe { &*self.rhs }) {
            Expr::IdentifierExpr(ref ident_expr) => {
                if ident_expr.is_var_mutable(symbol_table_ref)? {
                    Ok(())
                } else {
                    return Err(
                        CompileError::new_multiple(
                            vec![
                                ReportedError::new(
                                    format!(
                                        "Cannot get a reference to immutable variable '{}'",
                                        ident_expr.get_lexeme()
                                    ),
                                    ident_expr.collect_metadata()
                                ),
                                ReportedError::new(
                                    format!(
                                        "Consider changing to `mut {}`",
                                        ident_expr.get_lexeme()
                                    ),
                                    symbol_table_ref
                                        .get()
                                        .lookup_as_var(&ident_expr.get_lexeme())
                                        .or_else(|msg|
                                            Err(
                                                CompileError::new(
                                                    ReportedError::new(
                                                        msg,
                                                        ident_expr.collect_metadata()
                                                    )
                                                )
                                            )
                                        )?
                                        .get_metadata()
                                        .into()
                                )
                            ]
                        )
                    );
                }
            }
            _ => Ok(()),
        }
    }
}

impl<'ast> Dissasemble for UnaryExpr<'ast> {
    fn dissasemble(&self) -> String {
        unsafe { format!("{}{}", self.op.dissasemble(), (*self.rhs).dissasemble()) }
    }
}

impl<'ast> ExprTrait for UnaryExpr<'ast> {
    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
    //     let (value, mut src_chars_range) = unsafe { (*self.rhs).evaluate(ast_symbol_table)? };
    //     src_chars_range.dec_char_by(self.op.get_op_len());

    //     let new_value = match self.op {
    //         UnaryOp::Not =>
    //             match value.not() {
    //                 Ok(value) => { value }
    //                 Err(msg) => {
    //                     return Err(
    //                         ExprEvaluateErr::CompileError(
    //                             CompileError::new(ReportedError::new(msg, src_chars_range))
    //                         )
    //                     );
    //                 }
    //             }
    //         UnaryOp::Neg => {
    //             match value.neg() {
    //                 Ok(value) => { value }
    //                 Err(msg) => {
    //                     return Err(
    //                         ExprEvaluateErr::CompileError(
    //                             CompileError::new(ReportedError::new(msg, src_chars_range))
    //                         )
    //                     );
    //                 }
    //             }
    //         }
    //         UnaryOp::Deref => {
    //             return Err(
    //                 ExprEvaluateErr::InternalError(
    //                     InternalError::new(
    //                         InternalErrorCode::Compile(2),
    //                         "Cannot evaluate a deref expression"
    //                     )
    //                 )
    //             );
    //         }
    //         UnaryOp::Ref => {
    //             return Err(
    //                 ExprEvaluateErr::InternalError(
    //                     InternalError::new(
    //                         InternalErrorCode::Compile(0),
    //                         "Cannot evaluate an immutable reference"
    //                     )
    //                 )
    //             );
    //         }
    //         UnaryOp::MutRef => {
    //             return Err(
    //                 ExprEvaluateErr::InternalError(
    //                     InternalError::new(
    //                         InternalErrorCode::Compile(1),
    //                         "Cannot evaluate a mutable reference"
    //                     )
    //                 )
    //             );
    //         }
    //     };

    //     Ok((new_value, src_chars_range))
    // }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     let rhs_type = unsafe { (*self.rhs).type_check_and_constant_fold(ast_symbol_table)? };

    //     if rhs_type.get_can_constant_fold() {
    //         unsafe {
    //             let (new_value, src_chars_range) = (*self.rhs).evaluate(ast_symbol_table)?;
    //             *self.rhs = Expr::LiteralExpr(LiteralExpr::new(new_value, src_chars_range.into()));
    //         }
    //     }

    //     let can_constant_fold = rhs_type.get_can_constant_fold() && self.op.get_can_constant_fold();

    //     match rhs_type.value_type.try_unary(self.op) {
    //         Ok(v) => {
    //             match self.op {
    //                 UnaryOp::MutRef => {
    //                     let is_ident_expr = unsafe { (*self.rhs).is_ident_expr() };
    //                     match is_ident_expr {
    //                         true => {
    //                             let (lexeme, metadata) = unsafe { (*self.rhs).unwrap_ident_expr() };

    //                             let (value_type, origin_metadata, is_mutable) = match
    //                                 ast_symbol_table.get(&lexeme, &metadata)
    //                             {
    //                                 Ok(symbol) => symbol.unwrap().unwrap_variable(),
    //                                 Err(err) => {
    //                                     return ExprResult::Err(ExprResultErr::new(vec![err]));
    //                                 }
    //                             };

    //                             if *is_mutable {
    //                                 let mut src_chars_range: SrcCharsRange = metadata.into();
    //                                 src_chars_range.dec_char_by(self.op.get_op_len());
    //                                 Err(
    //                                     ExprResultErr::new(
    //                                         vec![
    //                                             CompileError::new_multiple(
    //                                                 vec![
    //                                                     ReportedError::new(
    //                                                         format!("Cannot get a reference to immutable variable '{}'", lexeme),
    //                                                         src_chars_range
    //                                                     ),
    //                                                     ReportedError::new(
    //                                                         format!("Consider changing to `mut {}`", lexeme),
    //                                                         (*origin_metadata).into()
    //                                                     )
    //                                                 ]
    //                                             )
    //                                         ]
    //                                     )
    //                                 )
    //                             } else {
    //                                 Ok(
    //                                     ExprResultOk::new(
    //                                         ValueType::MutableRef(Box::new(value_type.clone())),
    //                                         can_constant_fold
    //                                     )
    //                                 )
    //                             }
    //                         }
    //                         false =>
    //                             Ok(ExprResultOk {
    //                                 value_type: ValueType::MutableRef(Box::new(v)),
    //                                 can_constant_fold,
    //                             }),
    //                     }
    //                 }
    //                 _ => Ok(ExprResultOk { value_type: v, can_constant_fold }),
    //             }
    //         }
    //         Err(msg) => {
    //             let metadata = self.collect_metadata();
    //             Err(ExprResultErr::new(vec![CompileError::new(ReportedError::new(msg, metadata))]))
    //         }
    //     }
    // }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        let rhs_type = unsafe { (*self.rhs).type_check(symbol_table_ref)? };

        match rhs_type.try_unary(self.op) {
            Ok(v) => {
                match self.op {
                    UnaryOp::MutRef => {
                        self.check_if_mutable_ref_to_immutable_var(symbol_table_ref).and(Ok(v))
                    }
                    _ => Ok(v),
                }
            }
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
        unsafe { (*self.rhs).collect_metadata() }
    }
}
