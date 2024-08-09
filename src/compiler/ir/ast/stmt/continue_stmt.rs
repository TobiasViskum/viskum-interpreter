use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGGotoNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ GotoNodeIds, NodeIdsRange };

#[derive(Debug)]
pub struct ContinueStmt;

impl ContinueStmt {
    pub fn new() -> Self {
        Self
    }
}

impl StmtTrait for ContinueStmt {
    type ReturnTypeCompileIntoICFG = NodeIdsRange;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        let continue_node_id = icfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
        );
        goto_node_ids.push_continue_node_id(continue_node_id);

        NodeIdsRange::new(continue_node_id, continue_node_id)
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        todo!()
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl Dissasemble for ContinueStmt {
    fn dissasemble(&self) -> String {
        "continue".to_string()
    }
}
