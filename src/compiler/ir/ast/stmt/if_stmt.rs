use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGDecisionNode, CFGGotoNode, CFGLabelNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::{ CFGBuilder, ICFGBuilder },
        ICFG,
    },
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ BlockStmt, ExprStmt, GotoNodeIds };

#[derive(Debug)]
pub struct IfStmt<'ast> {
    condition: Option<ExprStmt<'ast>>,
    true_block: BlockStmt<'ast>,
    false_block: Option<&'ast mut IfStmt<'ast>>,
}

impl<'ast> IfStmt<'ast> {
    pub fn new(
        condition: Option<ExprStmt<'ast>>,
        mut true_block: BlockStmt<'ast>,
        false_block: Option<&'ast mut IfStmt<'ast>>
    ) -> Self {
        true_block.set_is_basic_block(false);
        Self {
            condition,
            true_block,
            false_block,
        }
    }
}

impl<'ast> Dissasemble for IfStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::new();
        if let Some(condition) = &self.condition {
            string_builder += format!("if {} {{\n", condition.dissasemble()).as_str();

            string_builder += self.true_block.dissasemble().as_str();

            if let Some(false_block) = &self.false_block {
                string_builder += false_block.dissasemble().as_str();
            }
        } else {
            string_builder += "} else {\n";

            string_builder += self.true_block.dissasemble().as_str();

            string_builder += "}\n";
        }

        string_builder
    }
}

impl<'ast> StmtTrait for IfStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        let condition = self.condition.as_ref().map(|expr| expr.compile_to_dag());

        if let Some(condition) = condition {
            let decision_node_id = cfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::DecisionNode(CFGDecisionNode::new(condition)))
            );

            /* Compile true block and label */
            let true_label_node_id = cfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::LabelNode(CFGLabelNode))
            );
            cfg_builder.push_cfg_edge(decision_node_id, true_label_node_id);
            cfg_builder.push_cfg_edge(true_label_node_id, cfg_builder.get_next_cfg_node_id());
            self.true_block.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids);
            let true_goto_node_id = cfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
            );
            cfg_builder.push_cfg_edge(decision_node_id, cfg_builder.get_next_cfg_node_id());

            /* Compile false block and label */
            if let Some(false_block) = self.false_block.as_ref() {
                let false_label_node_id = cfg_builder.push_cfg_node(
                    CFGNode::new(CFGNodeType::LabelNode(CFGLabelNode))
                );
                cfg_builder.push_cfg_edge(false_label_node_id, cfg_builder.get_next_cfg_node_id());
                false_block.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids);
            }

            cfg_builder.push_cfg_edge(true_goto_node_id, cfg_builder.get_next_cfg_node_id());
        } else {
            /* else { .. } block */
            self.true_block.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids);
            cfg_builder.push_linear_block_if_exists();
            let false_goto_node_id = cfg_builder.push_cfg_node(
                CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
            );
            cfg_builder.push_cfg_edge(false_goto_node_id, cfg_builder.get_next_cfg_node_id())
        }
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

        self.true_block.validate_stmt(symbol_table_ref, error_handler);

        if let Some(false_block) = &mut self.false_block {
            false_block.validate_stmt(symbol_table_ref, error_handler);
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}
