use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::{ CompileError, SrcCharsRange },
    ir::icfg::{ dag::{ DAGGroupNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
    traits::{ Dissasemble, ExprTrait },
    ProgramSymbolTablePhase1,
};

use super::Expr;

#[derive(Debug)]
pub struct GroupingExpr<'ast> {
    expr: &'ast mut Expr<'ast>,
    result_type: Option<ValueType>,
}

impl<'ast> GroupingExpr<'ast> {
    pub fn new(expr: &'ast mut Expr<'ast>) -> Self {
        Self {
            expr,
            result_type: None,
        }
    }

    pub fn get_expr(&mut self) -> &'ast mut Expr {
        self.expr
    }
}

impl<'ast> Dissasemble for GroupingExpr<'ast> {
    fn dissasemble(&self) -> String {
        format!("({})", (*self.expr).dissasemble())
    }
}

impl<'ast> ExprTrait for GroupingExpr<'ast> {
    fn get_result_type(&self) -> ValueType {
        self.result_type.as_ref().expect("TC").clone()
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        let mut metadata = (*self.expr).collect_metadata();
        metadata.dec_char_by(1);
        metadata.inc_char_by(1);
        metadata
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder,
        declaring_ssa_ident: Option<&SSAIdent>
    ) -> usize {
        let group_content_id = self.expr.compile_into_dag(
            dag,
            ident_node_id_map,
            icfg_builder,
            declaring_ssa_ident
        );
        let group_node_id = dag.push_node(
            DAGNode::GroupNode(
                DAGGroupNode::new(
                    self.result_type.as_ref().expect("Expected result type in GroupingExpr").clone()
                )
            )
        );
        dag.add_edge(group_node_id, group_content_id);
        group_node_id
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<(ValueType, Option<SSAIdent>), CompileError> {
        match self.expr.type_check(program_symbol_table) {
            Ok((v, ssa_ident)) => {
                self.result_type = Some(v.clone());
                Ok((v, ssa_ident))
            }
            Err(err) => Err(err),
        }
    }
}
