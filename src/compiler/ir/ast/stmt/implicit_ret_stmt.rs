use ahash::AHashMap;

use crate::compiler::{
    ds::symbol_table::{ SSAKey, SymbolTableRef },
    error_handler::ErrorHandler,
    ir::icfg::{
        dag::{ DAGIdentNode, DAGNode, DAG },
        icfg_builder::{ CFGBuilder, ICFGBuilder },
        ICFG,
    },
    traits::{ LinearControlFlow, StmtTrait },
    Dissasemble,
};

use super::{ ExprStmt, GotoNodeIds };

#[derive(Debug)]
pub struct ImplicitRetStmt<'ast> {
    expr: ExprStmt<'ast>,
    // strings
    // other heap allocated objects
}

impl<'ast> ImplicitRetStmt<'ast> {
    pub fn new(expr: ExprStmt<'ast>) -> Self {
        Self {
            expr,
        }
    }

    pub fn get_expr(&self) -> &ExprStmt<'ast> {
        &self.expr
    }
}

impl<'ast> StmtTrait for ImplicitRetStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        todo!()
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {}

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}

impl<'ast> Dissasemble for ImplicitRetStmt<'ast> {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
