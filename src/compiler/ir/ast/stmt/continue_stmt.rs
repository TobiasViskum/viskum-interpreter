use crate::compiler::{
    error_handler::ErrorHandler,
    ir::{
        ast::AST_DISSASEMBLE_INDENTATION,
        icfg::{
            cfg::{ CFGGotoNode, CFGNode, CFGNodeId, CFGNodeType, CFG },
            icfg_builder::{ CFGBuilder, ICFGBuilder },
            ICFG,
        },
    },
    print_todo,
    traits::{ AstDissasemble, Dissasemble, LinearControlFlow, StmtTrait },
    ProgramSymbolTablePhase1,
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
        icfg_builder: &mut ICFGBuilder,
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
        program_symbol_table: &mut ProgramSymbolTablePhase1,
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
        "continue\n".to_string()
    }
}
impl AstDissasemble for ContinueStmt {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        format!(
            "[{}]: {}continue\n",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth)
        )
    }
}
