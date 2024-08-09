use ahash::AHashMap;

use crate::compiler::{
    ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::ValueType },
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    ir::{
        ast::expr::Expr,
        icfg::{
            cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGProcessNode, CFG },
            dag::DAG,
            icfg_builder::ICFGBuilder,
            ICFG,
        },
    },
    traits::{ Dissasemble, ExprTrait, LinearControlFlow, StmtTrait },
};

use super::{ GotoNodeIds, NodeIdsRange };

#[derive(Debug)]
pub struct ExprStmt<'ast> {
    expr: Expr<'ast>,
}

impl<'ast> ExprStmt<'ast> {
    pub fn new(expr: Expr<'ast>) -> Self {
        Self { expr }
    }

    pub fn get_expr<'b>(&'b self) -> &'b Expr<'ast> {
        &self.expr
    }

    pub fn collect_metadata(&self) -> SrcCharsRange {
        self.expr.collect_metadata()
    }

    pub fn type_check(
        &mut self,
        symbol_table_ref: &SymbolTableRef
    ) -> Result<ValueType, CompileError> {
        self.expr.type_check(symbol_table_ref)
    }

    pub fn compile_to_dag(&self) -> DAG {
        let mut dag = DAG::new();
        let mut ident_node_id_map = AHashMap::new();
        self.compile_into_dag(&mut dag, &mut ident_node_id_map);
        dag
    }
}

impl<'ast> StmtTrait for ExprStmt<'ast> {
    type ReturnTypeCompileIntoICFG = NodeIdsRange;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        let mut dag = DAG::new();
        let mut ident_node_id_map = AHashMap::new();
        let entry_node_id = self.compile_into_dag(&mut dag, &mut ident_node_id_map);
        dag.set_entry_node_id(entry_node_id);
        let cfg_process_node = CFGNode::new(CFGNodeType::ProcessNode(CFGProcessNode::new(dag)));
        let cfg_node = icfg_builder.push_cfg_node(cfg_process_node);
        NodeIdsRange::new(cfg_node, cfg_node)
    }

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match self.expr.type_check(symbol_table_ref) {
            Ok(_) => {}
            Err(err) => error_handler.report_compile_error(err),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl<'ast> LinearControlFlow for ExprStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let node_id = self.expr.compile_into_dag(dag, ident_node_id_map);
        dag.set_entry_node_id(node_id);
        node_id
    }
}

impl<'ast> Dissasemble for ExprStmt<'ast> {
    fn dissasemble(&self) -> String {
        self.expr.dissasemble()
    }
}
