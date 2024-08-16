use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGGotoNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::{ CFGBuilder },
        ICFG,
    },
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ GotoNodeIds };

#[derive(Debug)]
pub struct ContinueStmt;

impl ContinueStmt {
    pub fn new() -> Self {
        Self
    }
}

impl StmtTrait for ContinueStmt {
    fn compile_into_icfg(
        &self,
        icfg: &mut ICFG,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        let continue_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
        );
        goto_node_ids.push_continue_node_id(continue_node_id);
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        print_todo("Check if continue stmt is used outside of loop")
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
