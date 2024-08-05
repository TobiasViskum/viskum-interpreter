use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGDecisionNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ ExprStmt, ScopeStmt };

#[derive(Debug)]
pub struct IfStmt<'ast> {
    condition: Option<ExprStmt<'ast>>,
    true_block: ScopeStmt<'ast>,
    false_block: Option<&'ast mut IfStmt<'ast>>,
}

impl<'ast> IfStmt<'ast> {
    pub fn new(
        condition: Option<ExprStmt<'ast>>,
        true_block: ScopeStmt<'ast>,
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

impl<'ast> StmtTrait for IfStmt<'ast> {
    fn compile_into_icfg(&self, icfg_builder: &mut ICFGBuilder) {
        // let condition = self.condition.as_ref().map(|expr| expr.compile_to_dag());
        // let decision_node_id = current_cfg.push_node(
        //     CFGNode::new(CFGNodeType::DecisionNode(CFGDecisionNode::new(condition)))
        // );
        // let true_node_id = self.true_block.compile_into_icfg(icfg, current_cfg);

        // let false_node_id = self.false_block
        //     .as_ref()
        //     .map(|scope| scope.compile_into_icfg(icfg, current_cfg));

        todo!()
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
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
