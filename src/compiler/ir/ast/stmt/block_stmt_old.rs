use ahash::AHashMap;

use crate::compiler::{
    ds::symbol_table::{ SSAKey, SymbolTableRef },
    error_handler::ErrorHandler,
    ir::icfg::{
        cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGProcessNode, CFG },
        dag::DAG,
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ BasicBlockStmt, GotoNodeIds, Stmt, Stmts };

#[derive(Debug)]
pub struct BlockStmt<'ast> {
    stmts: Stmts<'ast>,
    symbol_table_ref: SymbolTableRef,
}

impl<'ast> BlockStmt<'ast> {
    pub fn from_basic_block(basic_block: BasicBlockStmt<'ast>) -> Self {
        todo!()
        // let (stmts, symbol_table_ref) = basic_block.take();

        // Self {
        //     stmts,
        //     symbol_table_ref,
        // }
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
}

impl<'ast> Dissasemble for BlockStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("{\n");
        string_builder += self.stmts.dissasemble().as_str();
        string_builder += "}\n";
        string_builder
    }
}

impl<'ast> StmtTrait for BlockStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg: &mut ICFG,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        self.stmts.compile_into_icfg(icfg_builder, goto_node_ids);
        icfg_builder.push_linear_block_if_exists();

        let latest_node_id = icfg_builder.get_current_cfg_node_id();
    }

    fn is_linear_control_flow(&self) -> bool {
        self.stmts.is_linear_control_flow()
    }

    fn validate_stmt(&mut self, _: &mut SymbolTableRef, error_handler: &mut ErrorHandler) {
        self.stmts.validate_stmt(&mut self.get_symbol_table_ref(), error_handler);
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        if self.is_linear_control_flow() { Some(self) } else { None }
    }
}

impl<'ast> LinearControlFlow for BlockStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut ahash::AHashMap<SSAKey, usize>
    ) -> usize {
        self.stmts.compile_into_dag(dag, ident_node_id_map)
    }
}
