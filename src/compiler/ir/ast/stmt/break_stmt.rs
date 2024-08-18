use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::{ CompileError, ErrorHandler, ReportedError },
    ir::icfg::{
        cfg::{ CFGGotoNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::{ CFGBuilder, ICFGBuilder },
        ICFG,
    },
    parser::token::TokenMetadata,
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::GotoNodeIds;

#[derive(Debug)]
pub struct BreakStmt {
    metadata: TokenMetadata,
}

impl BreakStmt {
    pub fn new(metadata: TokenMetadata) -> Self {
        Self { metadata }
    }
}

impl StmtTrait for BreakStmt {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        let break_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
        );
        goto_node_ids.push_break_node_id(break_node_id);
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        print_todo("Check if break stmt is used outside of loop")
        // let local_symbol_table = symbol_table_ref.get();
        // if !local_symbol_table.is_is_in_loop() {
        //     error_handler.report_compile_error(
        //         CompileError::new(ReportedError::new(message, chars_range))
        //     )
        // }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl Dissasemble for BreakStmt {
    fn dissasemble(&self) -> String {
        "break\n".to_string()
    }
}
