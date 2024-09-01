use ahash::AHashMap;

use crate::compiler::{
    ds::{
        ssa_ident::SSAIdent,
        symbol_table::{ ProgramSymbolTablePhase1, UserSymbolVar },
        value::ValueType,
    },
    error_handler::ErrorHandler,
    ir::{
        ast::{ expr::IdentifierExpr, AST_DISSASEMBLE_INDENTATION },
        icfg::{ dag::{ DAGDeclareNode, DAGNode, DAG }, icfg_builder::{ CFGBuilder, ICFGBuilder } },
    },
    parser::token::TokenMetadata,
    traits::{ AstDissasemble, LinearControlFlow, StmtTrait },
    Dissasemble,
};

use super::GotoNodeIds;

#[derive(Debug)]
pub struct VarDeclarationStmt {
    ident_expr: IdentifierExpr,
    value_type: ValueType,
    mut_keyword_metadata: Option<TokenMetadata>,
}

impl VarDeclarationStmt {
    pub fn new(
        ident_expr: IdentifierExpr,
        value_type: ValueType,
        mut_keyword_metadata: Option<TokenMetadata>
    ) -> Self {
        Self { ident_expr, value_type, mut_keyword_metadata }
    }

    pub fn get_ident_expr(&self) -> &IdentifierExpr {
        &self.ident_expr
    }

    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }

    pub fn get_is_mutable(&self) -> bool {
        self.mut_keyword_metadata.is_some()
    }
}

impl StmtTrait for VarDeclarationStmt {
    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        _error_handler: &mut ErrorHandler
    ) {
        program_symbol_table.insert_var(
            self.ident_expr.get_ssa_ident().clone(),
            UserSymbolVar::new(
                self.value_type.clone(),
                self.ident_expr.get_metadata(),
                self.mut_keyword_metadata
            )
        )
    }

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        panic!("Should not go here")
    }

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl LinearControlFlow for VarDeclarationStmt {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        _: &mut AHashMap<SSAIdent, usize>,
        _: &mut ICFGBuilder
    ) -> usize {
        let decl_node_id = dag.push_node(
            DAGNode::DeclareNode(
                DAGDeclareNode::new(
                    self.ident_expr.get_ssa_ident().clone(),
                    self.value_type.clone()
                )
            )
        );
        dag.set_entry_node_id(decl_node_id);
        decl_node_id
    }
}

impl Dissasemble for VarDeclarationStmt {
    fn dissasemble(&self) -> String {
        let mutable_string = match self.mut_keyword_metadata.is_some() {
            true => "mut ".to_string(),
            false => "".to_string(),
        };

        format!(
            "{}{} {}\n",
            mutable_string,
            self.ident_expr.dissasemble(),
            self.value_type.dissasemble()
        )
    }
}

impl AstDissasemble for VarDeclarationStmt {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        format!(
            "[{}]: {}{}",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            self.dissasemble()
        )
    }
}
