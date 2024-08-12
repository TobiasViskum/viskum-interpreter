use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGDecisionNode, CFGGotoNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ BlockStmt, ExprStmt, GotoNodeIds };

#[derive(Debug)]
pub struct LoopStmt<'ast> {
    condition: Option<ExprStmt<'ast>>,
    body: BlockStmt<'ast>,
}

impl<'ast> LoopStmt<'ast> {
    pub fn new(condition: Option<ExprStmt<'ast>>, mut body: BlockStmt<'ast>) -> Self {
        body.set_is_basic_block(false);
        Self { condition, body }
    }
}

impl<'ast> Dissasemble for LoopStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::new();
        match &self.condition {
            Some(condition) => {
                string_builder += format!("while {} {{\n", condition.dissasemble()).as_str();
                string_builder += self.body.dissasemble().as_str();
                string_builder += "}\n";
            }
            None => {
                string_builder += "loop {\n";
                string_builder += self.body.dissasemble().as_str();
                string_builder += "}\n";
            }
        }
        string_builder
    }
}

impl<'ast> StmtTrait for LoopStmt<'ast> {
    fn compile_into_icfg(&self, icfg_builder: &mut ICFGBuilder, goto_node_ids: &mut GotoNodeIds) {
        let condition = self.condition.as_ref().map(|expr| expr.compile_to_dag());

        let decision_node_id = if let Some(condition) = condition {
            let decision_node_id = icfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::DecisionNode(CFGDecisionNode::new(condition)))
            );
            icfg_builder.push_cfg_edge(decision_node_id, icfg_builder.get_next_cfg_node_id());
            Some(decision_node_id)
        } else {
            None
        };

        let first_loop_node_id = if decision_node_id.is_some() {
            icfg_builder.get_current_cfg_node_id()
        } else {
            icfg_builder.get_next_cfg_node_id()
        };

        let break_node_ids_in_prev_block = goto_node_ids.take_break_node_ids();
        let continue_node_ids_in_prev_block = goto_node_ids.take_continue_node_ids();

        self.body.compile_into_icfg(icfg_builder, goto_node_ids);
        icfg_builder.push_linear_block_if_exists();

        icfg_builder.push_cfg_node(CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode)));
        let goto_node_id = icfg_builder.get_current_cfg_node_id();
        icfg_builder.push_cfg_edge(goto_node_id, first_loop_node_id);

        let after_loop_node_id = icfg_builder.get_next_cfg_node_id();

        if let Some(decisicion_node_id) = decision_node_id {
            icfg_builder.push_cfg_edge(decisicion_node_id, after_loop_node_id);
        }

        for break_node_id in goto_node_ids.take_break_node_ids().iter() {
            icfg_builder.push_cfg_edge(*break_node_id, after_loop_node_id);
        }
        for continue_node_id in goto_node_ids.take_continue_node_ids().iter() {
            icfg_builder.push_cfg_edge(*continue_node_id, first_loop_node_id);
        }

        goto_node_ids.replace_break_node_ids(break_node_ids_in_prev_block);
        goto_node_ids.replace_continue_node_ids(continue_node_ids_in_prev_block);
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        if let Some(condition) = &mut self.condition {
            condition.validate_stmt(symbol_table_ref, error_handler);
        }
        self.body.validate_stmt(symbol_table_ref, error_handler)
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}
