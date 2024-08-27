use std::rc::Rc;

use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::{ CompileError, ReportedError, SrcCharsRange },
    ir::icfg::{ dag::{ DAGIdentNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
    parser::token::TokenMetadata,
    traits::Dissasemble,
    ProgramSymbolTablePhase1,
};

use super::ExprTrait;

#[derive(Debug)]
pub struct IdentifierExpr {
    ssa_ident: SSAIdent,
    metadata: TokenMetadata,
    result_type: Option<ValueType>,
}

impl IdentifierExpr {
    pub fn new(ssa_ident: SSAIdent, metadata: TokenMetadata) -> Self {
        Self { ssa_ident, metadata, result_type: None }
    }

    pub fn get_lexeme(&self) -> Rc<str> {
        self.ssa_ident.get_ident()
    }

    pub fn set_result_type(&mut self, value_type: ValueType) {
        self.result_type = Some(value_type);
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }

    // pub fn is_var_mutable(
    //     &self,
    //     program_symbol_table: &ProgramSymbolTablePhase1
    // ) -> Result<bool, CompileError> {
    //     match program_symbol_table.lookup_var(&self.ssa_ident) {
    //         Ok(symbol_var) => Ok(symbol_var.get_is_mutable()),
    //         Err(msg) => Err(CompileError::new(ReportedError::new(msg, self.collect_metadata()))),
    //     }
    // }

    pub fn get_ssa_ident(&self) -> &SSAIdent {
        &self.ssa_ident
    }
}

impl Dissasemble for IdentifierExpr {
    fn dissasemble(&self) -> String {
        format!("{}", self.ssa_ident.dissasemble())
    }
}

impl ExprTrait for IdentifierExpr {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        _: &mut ICFGBuilder
    ) -> usize {
        if let Some(&node_id) = ident_node_id_map.get(&self.ssa_ident) {
            node_id
        } else {
            let ident_node_id = dag.push_node(
                DAGNode::IdentNode(
                    DAGIdentNode::new(
                        self.ssa_ident.clone(),
                        self.result_type
                            .as_ref()
                            .expect("Expected result type in IdentifierExpr")
                            .clone()
                    )
                )
            );
            ident_node_id_map.insert(self.ssa_ident.clone(), ident_node_id);
            ident_node_id
        }
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError> {
        match program_symbol_table.lookup_var(&self.ssa_ident) {
            Ok(symbol_var) => {
                let value_type = symbol_var.get_value_type();
                self.result_type = Some(value_type.clone());
                Ok(value_type)
            }
            Err(msg) => Err(CompileError::new(ReportedError::new(msg, self.collect_metadata()))),
        }
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
