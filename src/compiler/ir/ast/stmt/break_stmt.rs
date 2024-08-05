use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::ErrorHandler,
    ir::icfg::{ cfg::{ CFGNodeId, CFG }, icfg_builder::ICFGBuilder, ICFG },
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

#[derive(Debug)]
pub struct BreakStmt;

impl BreakStmt {
    pub fn new() -> Self {
        Self
    }
}

impl StmtTrait for BreakStmt {
    fn compile_into_icfg(&self, icfg_builder: &mut ICFGBuilder) {
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
        todo!()
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl Dissasemble for BreakStmt {
    fn dissasemble(&self) -> String {
        "break".to_string()
    }
}
