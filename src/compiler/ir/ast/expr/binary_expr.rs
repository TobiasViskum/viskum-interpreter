use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{ ssa_ident::SSAIdent, value::{ ops::BinaryOp, ValueType } },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::icfg::{ dag::{ DAGBinaryNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
        traits::{ Dissasemble, ExprTrait },
        ProgramSymbolTablePhase1,
    },
    macros::merge_chars_range,
};

use super::Expr;

#[derive(Debug)]
pub struct BinaryExpr<'ast> {
    lhs: &'ast mut Expr<'ast>,
    op: BinaryOp,
    rhs: &'ast mut Expr<'ast>,
    result_type: Option<ValueType>,
}

impl<'ast> BinaryExpr<'ast> {
    pub fn new(lhs: &'ast mut Expr<'ast>, op: BinaryOp, rhs: &'ast mut Expr<'ast>) -> Self {
        Self {
            lhs,
            op,
            rhs,
            result_type: None,
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
        format!("{} {} {}", self.lhs.dissasemble(), self.op.dissasemble(), self.rhs.dissasemble())
    }
}

impl<'ast> ExprTrait for BinaryExpr<'ast> {
    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError> {
        let (lhs_type, rhs_type) = (
            self.lhs.type_check(program_symbol_table)?,
            self.rhs.type_check(program_symbol_table)?,
        );

        match lhs_type.try_binary(&rhs_type, self.op) {
            Ok(v) => {
                self.result_type = Some(v.clone());
                Ok(v)
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
        let lhs_node_id = self.lhs.compile_into_dag(dag, ident_node_id_map, icfg_builder);
        let rhs_node_id = self.rhs.compile_into_dag(dag, ident_node_id_map, icfg_builder);
        let binary_node_id = dag.push_node(
            DAGNode::BinaryNode(
                DAGBinaryNode::new(
                    self.op,
                    self.result_type.as_ref().expect("Expected result type in BinaryExpr").clone()
                )
            )
        );
        dag.add_edge(binary_node_id, lhs_node_id);
        dag.add_edge(binary_node_id, rhs_node_id);
        binary_node_id
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        let (left_metadata, right_metadata) = (
            (*self.lhs).collect_metadata(),
            (*self.rhs).collect_metadata(),
        );

        let metadata = merge_chars_range!(left_metadata, right_metadata);

        metadata
    }
}
