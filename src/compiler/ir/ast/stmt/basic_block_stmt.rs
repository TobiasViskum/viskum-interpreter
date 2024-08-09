use crate::compiler::{
    ds::symbol_table::{ SSAKey, SymbolTableRef },
    error_handler::ErrorHandler,
    ir::icfg::{ cfg::{ CFGNodeId, CFG }, dag::DAG, icfg_builder::ICFGBuilder, ICFG },
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ BlockStmt, GotoNodeIds, NodeIdsRange, Stmt, Stmts };

#[derive(Debug)]
pub struct BasicBlockStmt<'ast> {
    stmts: Stmts<'ast>,
    symbol_table_ref: SymbolTableRef,
    // forwards_declarations: Stmts, // TypeDefStmt, FnStmt, (ClassStmt)
}

impl<'ast> BasicBlockStmt<'ast> {
    pub fn new(symbol_table_ref: SymbolTableRef) -> Self {
        Self {
            stmts: Stmts::new(),
            symbol_table_ref,
            // symbol_table_ref,
        }
    }

    pub fn get_symbol_table_ref(&self) -> SymbolTableRef {
        self.symbol_table_ref
    }

    pub fn push_stmt(&mut self, stmt: Stmt<'ast>) {
        print_todo("Remove result from below function (wait to declare func until validate_stmt)");
        self.stmts.push(stmt, &mut self.symbol_table_ref);
    }

    pub fn get_stmts(&self) -> &Stmts<'ast> {
        &self.stmts
    }

    pub fn take(self) -> (Stmts<'ast>, SymbolTableRef) {
        (self.stmts, self.symbol_table_ref)
    }
}

impl<'ast> Dissasemble for BasicBlockStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("{\n");
        string_builder += self.stmts.dissasemble().as_str();
        string_builder += "}\n";
        string_builder
    }
}

impl<'ast> StmtTrait for BasicBlockStmt<'ast> {
    type ReturnTypeCompileIntoICFG = Option<NodeIdsRange>;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        self.stmts.compile_into_icfg(icfg_builder, goto_node_ids)
    }

    fn is_linear_control_flow(&self) -> bool {
        self.stmts.is_linear_control_flow()
    }

    fn validate_stmt(&mut self, d: &mut SymbolTableRef, error_handler: &mut ErrorHandler) {
        println!("2");
        self.stmts.validate_stmt(&mut self.get_symbol_table_ref(), error_handler);
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        if self.is_linear_control_flow() { Some(self) } else { None }
    }
}

impl<'ast> LinearControlFlow for BasicBlockStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut ahash::AHashMap<SSAKey, usize>
    ) -> usize {
        self.stmts.compile_into_dag(dag, ident_node_id_map)
    }
}
