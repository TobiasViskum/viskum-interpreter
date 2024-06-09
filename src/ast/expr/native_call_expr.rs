use crate::{
    ast::ast_symbol_table::AstSymbolTable,
    compiler::cfg::dag::DAG,
    error_handler::{ CompileError, InternalError, InternalErrorCode, SrcCharsRange },
    parser::token::TokenMetadata,
    value::ValueType,
    vm::instructions::NativeCall,
    Dissasemble,
};

use super::{ Expr, ExprEvaluateErr, ExprEvaluateResult, ExprResult, ExprTrait };

#[derive(Debug)]
pub struct NativeCallExpr {
    metadata: TokenMetadata,
    args: Vec<Expr>,
    native_call: NativeCall,
}

impl Dissasemble for NativeCallExpr {
    fn dissasemble(&self) -> String {
        todo!()
    }
}

impl ExprTrait for NativeCallExpr {
    fn evaluate(&mut self, _: &AstSymbolTable) -> ExprEvaluateResult {
        return Err(
            ExprEvaluateErr::InternalError(
                InternalError::new(InternalErrorCode::Compile(4), "Cannot evaluate a native call")
            )
        );
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        todo!()
    }

    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
        todo!()
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        todo!()
    }
}
