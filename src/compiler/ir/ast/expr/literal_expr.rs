use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::{ Value, ValueType } },
    error_handler::{ CompileError, SrcCharsRange },
    ir::icfg::dag::DAG,
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::ExprTrait;

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
    // fn evaluate(&mut self, _: &AstSymbolTable) -> ExprEvaluateResult {
    //     Ok((self.get_value().clone(), self.collect_metadata()))
    // }

    fn compile_to_dag_node(&self, _: &mut DAG) -> usize {
        todo!()
    }

    // fn type_check_and_constant_fold(&mut self, _: &AstSymbolTable) -> ExprResult {
    //     Ok(ExprResultOk::new(self.value.to_value_type(), true))
    // }

    fn type_check(&mut self, _: &SymbolTableRef) -> Result<ValueType, CompileError> {
        Ok(self.value.to_value_type())
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
