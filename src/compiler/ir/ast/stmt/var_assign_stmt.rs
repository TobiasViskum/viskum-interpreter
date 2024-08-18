use ahash::AHashMap;

use crate::compiler::{
    ds::symbol_table::{ SSAKey, SymbolTableRef },
    error_handler::ErrorHandler,
    ir::{
        ast::expr::{ Expr, IdentifierExpr },
        icfg::{
            dag::{ DAGAssignNode, DAGNode, DAG },
            icfg_builder::{ CFGBuilder, ICFGBuilder },
            ICFG,
        },
    },
    traits::{ Dissasemble, ExprTrait },
};

use super::{ ExprStmt, GotoNodeIds, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct VarAssignStmt<'ast> {
    target_expr: IdentifierExpr, // ExprStmt<'ast>,
    value: ExprStmt<'ast>,
}

impl<'ast> VarAssignStmt<'ast> {
    pub fn new(target_expr: IdentifierExpr, value: ExprStmt<'ast>) -> Self {
        Self {
            target_expr,
            value,
        }
    }

    pub fn get_target_expr(&self) -> &IdentifierExpr /*&ExprStmt<'ast>*/ {
        &self.target_expr
    }

    pub fn get_mut_target_expr(&mut self) -> &mut IdentifierExpr /*&mut ExprStmt<'ast>*/ {
        &mut self.target_expr
    }

    pub fn get_mut_value_expr(&mut self) -> &mut ExprStmt<'ast> {
        &mut self.value
    }

    pub fn get_value_expr(&mut self) -> &ExprStmt<'ast> {
        &self.value
    }

    pub fn set_ssa_subscript(&mut self, subscript: usize) {
        self.target_expr.set_ssa_subscript(subscript)
    }
}

impl<'ast> Dissasemble for VarAssignStmt<'ast> {
    fn dissasemble(&self) -> String {
        format!("{} = {}\n", self.target_expr.dissasemble(), self.value.dissasemble())
    }
}

impl<'ast> StmtTrait for VarAssignStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        todo!()
    }

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match symbol_table_ref.get_mut().assing_var(self) {
            Ok(ssa_key) => {
                self.set_ssa_subscript(ssa_key.get_subscript());
            }
            Err(err) => error_handler.report_compile_error(err),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl<'ast> LinearControlFlow for VarAssignStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let ident_node_id = self.target_expr.compile_into_dag(dag, ident_node_id_map);
        let value_node_id = self.value.compile_into_dag(dag, ident_node_id_map);

        let assign_node_id = dag.push_node(DAGNode::AssignNode(DAGAssignNode));

        dag.add_edge(assign_node_id, ident_node_id);
        dag.add_edge(assign_node_id, value_node_id);

        dag.set_entry_node_id(assign_node_id);

        assign_node_id
    }
}
