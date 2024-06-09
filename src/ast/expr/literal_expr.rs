use crate::{
    ast::ast_symbol_table::AstSymbolTable,
    compiler::cfg::dag::DAG,
    error_handler::{ CompileError, InternalError, SrcCharsRange },
    parser::token::TokenMetadata,
    value::{ Value, ValueType },
    Dissasemble,
};

use super::{ ExprEvaluateResult, ExprResult, ExprResultOk, ExprTrait };

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
    fn evaluate(&mut self, _: &AstSymbolTable) -> ExprEvaluateResult {
        Ok((self.get_value().clone(), self.collect_metadata()))
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        todo!()
    }

    fn type_check_and_constant_fold(&mut self, _: &AstSymbolTable) -> ExprResult {
        Ok(ExprResultOk::new(self.value.to_value_type(), true))
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
