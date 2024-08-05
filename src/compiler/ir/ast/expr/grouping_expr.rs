use ahash::AHashMap;

use crate::compiler::{
    ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::ValueType },
    error_handler::{ CompileError, SrcCharsRange },
    ir::icfg::dag::{ DAGGroupNode, DAGNode, DAG },
    traits::{ Dissasemble, ExprTrait },
};

use super::Expr;

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
        format!("({})", (*self.expr).dissasemble())
    }
}

impl<'ast> ExprTrait for GroupingExpr<'ast> {
    fn collect_metadata(&self) -> SrcCharsRange {
        let mut metadata = (*self.expr).collect_metadata();
        metadata.dec_char_by(1);
        metadata.inc_char_by(1);
        metadata
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let group_content_id = self.expr.compile_into_dag(dag, ident_node_id_map);
        let group_node_id = dag.push_node(DAGNode::GroupNode(DAGGroupNode));
        dag.add_edge(group_node_id, group_content_id);
        group_node_id
    }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        (*self.expr).type_check(symbol_table_ref)
    }

    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
    //     unsafe { (*self.expr).evaluate(ast_symbol_table) }
    // }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     unsafe { (*self.expr).type_check_and_constant_fold(ast_symbol_table) }
    // }
}
