use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::{ ops::BinaryOp, ValueType } },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::icfg::dag::{ DAGBinaryNode, DAGNode, DAG },
        traits::{ Dissasemble, ExprTrait },
    },
    macros::merge_chars_range,
};

use super::Expr;

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
        format!("{} {} {}", self.lhs.dissasemble(), self.op.dissasemble(), self.rhs.dissasemble())
    }
}

impl<'ast> ExprTrait for BinaryExpr<'ast> {
    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        let (lhs_type, rhs_type) = (
            self.lhs.type_check(symbol_table_ref)?,
            (*self.rhs).type_check(symbol_table_ref)?,
        );

        match lhs_type.try_binary(&rhs_type, self.op) {
            Ok(v) => Ok(v),
            Err(msg) => {
                let metadata = self.collect_metadata();
                Err(CompileError::new(ReportedError::new(msg, metadata)))
            }
        }
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let lhs_node_id = self.lhs.compile_into_dag(dag, ident_node_id_map);
        let rhs_node_id = self.rhs.compile_into_dag(dag, ident_node_id_map);
        let binary_node_id = dag.push_node(DAGNode::BinaryNode(DAGBinaryNode::new(self.op)));
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
