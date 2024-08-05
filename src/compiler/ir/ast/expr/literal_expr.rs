use ahash::AHashMap;

use crate::compiler::{
    ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::{ Value, ValueType } },
    error_handler::{ CompileError, SrcCharsRange },
    ir::icfg::dag::{ DAGConstNode, DAGNode, DAG },
    parser::token::TokenMetadata,
    traits::{ Dissasemble, ExprTrait },
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
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        dag.push_node(DAGNode::ConstNode(DAGConstNode::new(self.value.clone())))
    }

    fn type_check(&mut self, _: &SymbolTableRef) -> Result<ValueType, CompileError> {
        Ok(self.value.to_value_type())
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
