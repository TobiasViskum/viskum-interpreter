use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::{ Value, ValueType } },
    error_handler::{ CompileError, SrcCharsRange },
    ir::icfg::{ dag::{ DAGConstNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
    parser::token::TokenMetadata,
    traits::{ Dissasemble, ExprTrait },
    ProgramSymbolTablePhase1,
};

#[derive(Debug)]
pub struct LiteralExpr {
    value: Value,
    metadata: TokenMetadata,
}

impl LiteralExpr {
    pub fn new(value: Value, metadata: TokenMetadata) -> Self {
        Self {
            value,
            metadata,
        }
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }

    pub fn take_value(self) -> Value {
        self.value
    }
}

impl Dissasemble for LiteralExpr {
    fn dissasemble(&self) -> String {
        format!("{}", self.value.dissasemble())
    }
}

impl ExprTrait for LiteralExpr {
    fn get_result_type(&self) -> ValueType {
        self.value.to_value_type()
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        _: &mut AHashMap<SSAIdent, usize>,
        _: &mut ICFGBuilder,
        declaring_ssa_ident: Option<&SSAIdent>
    ) -> usize {
        dag.push_node(DAGNode::ConstNode(DAGConstNode::new(self.value.clone())))
    }

    fn type_check(
        &mut self,
        _: &ProgramSymbolTablePhase1
    ) -> Result<(ValueType, Option<SSAIdent>), CompileError> {
        Ok((self.value.to_value_type(), None))
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
