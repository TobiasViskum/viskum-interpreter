use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    ir::{
        ast::expr::Expr,
        icfg::{
            cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGProcessNode, CFG },
            dag::DAG,
            icfg_builder::{ CFGBuilder, ICFGBuilder },
            ICFG,
        },
    },
    traits::{ AstDissasemble, Dissasemble, ExprTrait, LinearControlFlow, StmtTrait },
    ProgramSymbolTablePhase1,
};

use super::{ GotoNodeIds };

#[derive(Debug)]
pub struct ExprStmt<'ast> {
    expr: Expr<'ast>,
}

impl<'ast> ExprStmt<'ast> {
    pub fn new(expr: Expr<'ast>) -> Self {
        Self { expr }
    }

    pub fn get_expr<'b>(&'b self) -> &'b Expr<'ast> {
        &self.expr
    }

    pub fn collect_metadata(&self) -> SrcCharsRange {
        self.expr.collect_metadata()
    }

    pub fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError> {
        self.expr.type_check(program_symbol_table)
    }

    pub fn compile_to_dag(&self, icfg_builder: &mut ICFGBuilder) -> DAG {
        let mut dag = DAG::new();
        let mut ident_node_id_map = AHashMap::new();
        self.compile_into_dag(&mut dag, &mut ident_node_id_map, icfg_builder);
        dag
    }
}

impl<'ast> StmtTrait for ExprStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        unimplemented!("Should hopefully be compiled into DAG instead")
        // let mut dag = DAG::new();
        // let mut ident_node_id_map = AHashMap::new();
        // let entry_node_id = self.compile_into_dag(&mut dag, &mut ident_node_id_map);
        // dag.set_entry_node_id(entry_node_id);
        // let cfg_process_node = CFGNode::new(CFGNodeType::ProcessNode(CFGProcessNode::new(dag)));
        // let cfg_node = icfg_builder.push_cfg_node(cfg_process_node);
    }

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        match self.expr.type_check(program_symbol_table) {
            Ok(_) => {}
            Err(err) => error_handler.report_compile_error(err),
        }
    }

    // fn validate_stmt(
    //     &mut self,
    //     symbol_table_ref: &mut SymbolTableRef,
    //     error_handler: &mut ErrorHandler
    // ) {
    //     match self.expr.type_check(symbol_table_ref) {
    //         Ok(_) => {}
    //         Err(err) => error_handler.report_compile_error(err),
    //     }
    // }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl<'ast> LinearControlFlow for ExprStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        let node_id = self.expr.compile_into_dag(dag, ident_node_id_map, icfg_builder);
        dag.set_entry_node_id(node_id);
        node_id
    }
}

impl<'ast> Dissasemble for ExprStmt<'ast> {
    fn dissasemble(&self) -> String {
        self.expr.dissasemble()
    }
}

impl<'ast> AstDissasemble for ExprStmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        format!("{}", self.expr.dissasemble())
    }
}
