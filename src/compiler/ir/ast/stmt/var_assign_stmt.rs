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
    target_expr: Expr<'ast>,
    value: Expr<'ast>,
    result_type: Option<ValueType>,
    ssa_ident: Option<SSAIdent>,
}

impl<'ast> VarAssignStmt<'ast> {
    pub fn new(target_expr: Expr<'ast>, value: Expr<'ast>) -> Self {
        Self {
            ssa_ident: None,
            target_expr,
            value,
            result_type: None,
        }
    }

    pub fn set_ssa_ident(&mut self, ssa_ident: SSAIdent) {
        self.ssa_ident = Some(ssa_ident);
    }

    pub fn get_target_expr(&self) -> &Expr<'ast> /*&ExprStmt<'ast>*/ {
        &self.target_expr
    }

    pub fn get_mut_target_expr(&mut self) -> &mut Expr<'ast> /*&mut ExprStmt<'ast>*/ {
        &mut self.target_expr
    }

    pub fn get_mut_value_expr(&mut self) -> &mut Expr<'ast> {
        &mut self.value
    }

    pub fn get_value_expr(&self) -> &Expr<'ast> {
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
        let ident_node_id = self.target_expr.compile_into_dag(
            dag,
            ident_node_id_map,
            icfg_builder,
            None
        );
        let value_node_id = self.value.compile_into_dag(dag, ident_node_id_map, icfg_builder, None);

        let has_field_expr = match self.target_expr {
            Expr::IdentifierExpr(_) => false,
            _ => true,
        };

        let assign_node_id = dag.push_node(
            DAGNode::AssignNode(
                DAGAssignNode::new(
                    self.ssa_ident.as_ref().expect("Expected during typechecking").clone(),
                    self.result_type
                        .as_ref()
                        .expect("Expected result type in VarAssignStmt")
                        .clone(),
                    has_field_expr
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
