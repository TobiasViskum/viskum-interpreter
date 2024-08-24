use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::ErrorHandler,
    ir::{
        ast::{ expr::{ Expr, IdentifierExpr }, AST_DISSASEMBLE_INDENTATION },
        icfg::{
            dag::{ DAGAssignNode, DAGNode, DAG },
            icfg_builder::{ CFGBuilder, ICFGBuilder },
            ICFG,
        },
    },
    traits::{ AstDissasemble, Dissasemble, ExprTrait },
    ProgramSymbolTablePhase1,
};

use super::{ ExprStmt, GotoNodeIds, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct VarAssignStmt<'ast> {
    target_expr: IdentifierExpr, // ExprStmt<'ast>,
    value: ExprStmt<'ast>,
    result_type: Option<ValueType>,
}

impl<'ast> VarAssignStmt<'ast> {
    pub fn new(target_expr: IdentifierExpr, value: ExprStmt<'ast>) -> Self {
        Self {
            target_expr,
            value,
            result_type: None,
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

    pub fn get_value_expr(&self) -> &ExprStmt<'ast> {
        &self.value
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
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        match program_symbol_table.assign_var(self) {
            Ok(v) => {
                self.result_type = Some(v.clone());
                self.target_expr.set_result_type(v);
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
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        let ident_node_id = self.target_expr.compile_into_dag(dag, ident_node_id_map, icfg_builder);
        let value_node_id = self.value.compile_into_dag(dag, ident_node_id_map, icfg_builder);

        let assign_node_id = dag.push_node(
            DAGNode::AssignNode(
                DAGAssignNode::new(
                    self.result_type
                        .as_ref()
                        .expect("Expected result type in VarAssignStmt")
                        .clone()
                )
            )
        );

        dag.add_edge(assign_node_id, ident_node_id);
        dag.add_edge(assign_node_id, value_node_id);

        dag.set_entry_node_id(assign_node_id);

        assign_node_id
    }
}

impl<'ast> Dissasemble for VarAssignStmt<'ast> {
    fn dissasemble(&self) -> String {
        format!("{} = {}\n", self.target_expr.dissasemble(), self.value.dissasemble())
    }
}

impl<'ast> AstDissasemble for VarAssignStmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        format!(
            "[{}]: {}{} = {}\n",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            self.target_expr.dissasemble(),
            self.value.dissasemble()
        )
    }
}
