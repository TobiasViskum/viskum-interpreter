use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGDecisionNode, CFGGotoNode, CFGLabelNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::{ CFGBuilder, ICFGBuilder },
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
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        let break_node_ids_in_prev_block = goto_node_ids.take_break_node_ids();
        let continue_node_ids_in_prev_block = goto_node_ids.take_continue_node_ids();

        let condition = self.condition.as_ref().map(|expr| expr.compile_to_dag());

        let start_loop_goto_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
        );
        let start_loop_label_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::LabelNode(CFGLabelNode))
        );
        cfg_builder.push_cfg_edge(start_loop_goto_node_id, start_loop_label_node_id);
        cfg_builder.push_cfg_edge(start_loop_label_node_id, cfg_builder.get_next_cfg_node_id());

        let decision_node_id = if let Some(condition) = condition {
            let decision_node_id = cfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::DecisionNode(CFGDecisionNode::new(condition)))
            );

            /* Compile body */
            let loop_body_label_node_id = cfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::LabelNode(CFGLabelNode))
            );
            cfg_builder.push_cfg_edge(decision_node_id, loop_body_label_node_id);
            cfg_builder.push_cfg_edge(loop_body_label_node_id, cfg_builder.get_next_cfg_node_id());
            self.body.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids);
            Some(decision_node_id)
        } else {
            self.body.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids);
            None
        };

        // Compile end of loop
        let end_loop_goto_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
        );
        cfg_builder.push_cfg_edge(end_loop_goto_node_id, start_loop_label_node_id);

        let end_loop_label_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::LabelNode(CFGLabelNode))
        );
        cfg_builder.push_cfg_edge(end_loop_label_node_id, cfg_builder.get_next_cfg_node_id());
        if let Some(decision_node_id) = decision_node_id {
            cfg_builder.push_cfg_edge(decision_node_id, end_loop_label_node_id);
        }

        /* Add edges between breaks and continues */
        for break_node_id in goto_node_ids.take_break_node_ids().iter() {
            cfg_builder.push_cfg_edge(*break_node_id, end_loop_label_node_id);
        }
        for continue_node_id in goto_node_ids.take_continue_node_ids().iter() {
            cfg_builder.push_cfg_edge(*continue_node_id, start_loop_label_node_id);
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
