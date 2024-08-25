use crate::compiler::{
    error_handler::ErrorHandler,
    ir::{
        ast::AST_DISSASEMBLE_INDENTATION,
        icfg::{
            cfg::{ CFGGotoNode, CFGNode, CFGNodeType },
            icfg_builder::{ CFGBuilder, ICFGBuilder },
        },
    },
    parser::token::TokenMetadata,
    print_todo,
    traits::{ AstDissasemble, Dissasemble, LinearControlFlow, StmtTrait },
    ProgramSymbolTablePhase1,
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
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        print_todo("Check if break stmt is used outside of loop")
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

impl AstDissasemble for BreakStmt {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        format!(
            "[{}]: {}break\n",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth)
        )
    }
}
