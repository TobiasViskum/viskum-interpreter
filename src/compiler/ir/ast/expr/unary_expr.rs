use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::{ ops::UnaryOp, ValueType } },
    error_handler::{ CompileError, ReportedError, SrcCharsRange },
    ir::icfg::{ dag::{ DAGNode, DAGUnaryNode, DAG }, icfg_builder::ICFGBuilder },
    Dissasemble,
    ProgramSymbolTablePhase1,
};

use super::{ Expr, ExprTrait };

#[derive(Debug)]
pub struct UnaryExpr<'ast> {
    op: UnaryOp,
    rhs: &'ast mut Expr<'ast>,
    result_type: Option<ValueType>,
}

impl<'ast> UnaryExpr<'ast> {
    pub fn new(op: UnaryOp, rhs: &'ast mut Expr<'ast>) -> Self {
        Self {
            op,
            rhs,
            result_type: None,
        }
    }

    pub fn get_op(&self) -> UnaryOp {
        self.op
    }

    pub fn get_rhs(&mut self) -> &'ast mut Expr {
        self.rhs
    }

    // pub fn check_if_mutable_ref_to_immutable_var(
    //     &self,
    //     symbol_table_ref: &SymbolTableRef
    // ) -> Result<(), CompileError> {
    //     match &*self.rhs {
    //         Expr::IdentifierExpr(ref ident_expr) => {
    //             if ident_expr.is_var_mutable(symbol_table_ref)? {
    //                 Ok(())
    //             } else {
    //                 return Err(
    //                     CompileError::new_multiple(
    //                         vec![
    //                             ReportedError::new(
    //                                 format!(
    //                                     "Cannot get a reference to immutable variable '{}'",
    //                                     ident_expr.get_lexeme()
    //                                 ),
    //                                 ident_expr.collect_metadata()
    //                             ),
    //                             ReportedError::new(
    //                                 format!(
    //                                     "Consider changing to `mut {}`",
    //                                     ident_expr.get_lexeme()
    //                                 ),
    //                                 symbol_table_ref
    //                                     .get()
    //                                     .lookup_as_var(&ident_expr.get_lexeme())
    //                                     .or_else(|msg|
    //                                         Err(
    //                                             CompileError::new(
    //                                                 ReportedError::new(
    //                                                     msg,
    //                                                     ident_expr.collect_metadata()
    //                                                 )
    //                                             )
    //                                         )
    //                                     )?
    //                                     .get_metadata()
    //                                     .into()
    //                             )
    //                         ]
    //                     )
    //                 );
    //             }
    //         }
    //         _ => Ok(()),
    //     }
    // }
}

impl<'ast> Dissasemble for UnaryExpr<'ast> {
    fn dissasemble(&self) -> String {
        format!("{}{}", self.op.dissasemble(), (*self.rhs).dissasemble())
    }
}

impl<'ast> ExprTrait for UnaryExpr<'ast> {
    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError> {
        let rhs_type = self.rhs.type_check(program_symbol_table)?;

        match rhs_type.try_unary(self.op) {
            Ok(v) => {
                self.result_type = Some(v.clone());

                match self.op {
                    // UnaryOp::MutRef => {
                    //     self.check_if_mutable_ref_to_immutable_var(symbol_table_ref).and(Ok(v))
                    // }
                    _ => Ok(v),
                }
            }
            Err(msg) => {
                let metadata = self.collect_metadata();
                Err(CompileError::new(ReportedError::new(msg, metadata)))
            }
        }
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        let rhs_node_id = self.rhs.compile_into_dag(dag, ident_node_id_map, icfg_builder);
        let unary_node_id = dag.push_node(
            DAGNode::UnaryNode(
                DAGUnaryNode::new(
                    self.op,
                    self.result_type.as_ref().expect("Expected result type in UnaryExpr").clone()
                )
            )
        );
        dag.add_edge(unary_node_id, rhs_node_id);
        unary_node_id
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        (*self.rhs).collect_metadata()
    }
}
