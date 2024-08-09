use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGDecisionNode, CFGGotoNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ BasicBlockStmt, BlockStmt, ExprStmt, GotoNodeIds, NodeIdsRange };

#[derive(Debug)]
pub struct IfStmt<'ast> {
    condition: Option<ExprStmt<'ast>>,
    true_block: BlockStmt<'ast>,
    false_block: Option<&'ast mut IfStmt<'ast>>,
}

impl<'ast> IfStmt<'ast> {
    pub fn new(
        condition: Option<ExprStmt<'ast>>,
        true_block: BlockStmt<'ast>,
        false_block: Option<&'ast mut IfStmt<'ast>>
    ) -> Self {
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
            string_builder += format!("if {} ", condition.dissasemble()).as_str();

            string_builder += self.true_block.dissasemble().as_str();

            if let Some(false_block) = &self.false_block {
                string_builder += false_block.dissasemble().as_str();
            }
        } else {
            string_builder += "else ";

            string_builder += self.true_block.dissasemble().as_str();
        }

        string_builder
    }
}

impl<'ast> IfStmt<'ast> {
    fn add_edges_in_if_branch_end(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds,
        decision_node_id: usize,
        block_node_ids: Option<NodeIdsRange>
    ) -> Option<usize> {
        let linear_block_id = icfg_builder.push_linear_block_if_exists();

        let latest_node_id = match (linear_block_id, block_node_ids) {
            (Some(linear_block_id), Some(block_node_ids)) => {
                icfg_builder.push_cfg_edge(
                    decision_node_id,
                    block_node_ids.get_first_added_node_id()
                );
                icfg_builder.push_cfg_edge(
                    block_node_ids.get_last_added_node_id(),
                    linear_block_id
                );
                Some(linear_block_id)
            }
            (None, Some(block_node_ids)) => {
                icfg_builder.push_cfg_edge(
                    decision_node_id,
                    block_node_ids.get_first_added_node_id()
                );
                Some(block_node_ids.get_last_added_node_id())
            }
            (Some(linear_block_id), None) => {
                icfg_builder.push_cfg_edge(decision_node_id, linear_block_id);
                Some(linear_block_id)
            }
            (None, None) => None,
        };

        latest_node_id
    }
}

impl<'ast> StmtTrait for IfStmt<'ast> {
    type ReturnTypeCompileIntoICFG = NodeIdsRange;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        let condition = self.condition.as_ref().map(|expr| expr.compile_to_dag());

        let decision_node_id = icfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::DecisionNode(CFGDecisionNode::new(condition)))
        );
        let true_node_ids = self.true_block.compile_into_icfg(icfg_builder, goto_node_ids);
        let latest_true_node_id = self.add_edges_in_if_branch_end(
            icfg_builder,
            goto_node_ids,
            decision_node_id,
            true_node_ids
        );

        let true_goto_node_id = icfg_builder.push_cfg_node(
            CFGNode::new(CFGNodeType::GotoNode(CFGGotoNode))
        );
        match latest_true_node_id {
            Some(latest_node_id) => {
                icfg_builder.push_cfg_edge(latest_node_id, true_goto_node_id);
            }
            None => {
                icfg_builder.push_cfg_edge(decision_node_id, true_goto_node_id);
            }
        }
        goto_node_ids.push_if_branch_end_node_id(true_goto_node_id);

        print_todo(
            "Add optimization, so if nothing inside block, don't generate cfg nodes (decision_node_id, true_node_ids)"
        );

        let false_goto_node_id = self.false_block.as_ref().map(|scope| {
            let false_node_ids = scope.compile_into_icfg(icfg_builder, goto_node_ids);
            let false_goto_node_id = self.add_edges_in_if_branch_end(
                icfg_builder,
                goto_node_ids,
                decision_node_id,
                Some(false_node_ids)
            );

            println!("{:?} {:?}", false_node_ids, false_goto_node_id);
            match false_goto_node_id {
                Some(false_goto_node_id) => false_goto_node_id,
                None => false_node_ids.get_last_added_node_id(),
            }
        });

        println!("{:?}", false_goto_node_id);

        NodeIdsRange::new(decision_node_id, false_goto_node_id.unwrap_or(true_goto_node_id))
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
