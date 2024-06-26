use crate::{
    compiler::cfg::dag::DAG,
    error_handler::{ CompileError, SrcCharsRange },
    parser::token::TokenMetadata,
    symbol_table::SymbolTableRef,
    value::ValueType,
    vm::instructions::NativeCall,
    Dissasemble,
};

use super::{ Expr, ExprTrait };

#[derive(Debug)]
pub struct NativeCallExpr<'ast> {
    metadata: TokenMetadata,
    args: Vec<Expr<'ast>>,
    native_call: NativeCall,
}

impl<'ast> Dissasemble for NativeCallExpr<'ast> {
    fn dissasemble(&self) -> String {
        todo!()
    }
}

impl<'ast> ExprTrait for NativeCallExpr<'ast> {
    // fn evaluate(&mut self, _: &AstSymbolTable) -> ExprEvaluateResult {
    //     return Err(
    //         ExprEvaluateErr::InternalError(
    //             InternalError::new(InternalErrorCode::Compile(4), "Cannot evaluate a native call")
    //         )
    //     );
    // }

    fn compile_to_dag_node(&self, _: &mut DAG) -> usize {
        todo!()
    }

    fn type_check(&mut self, _: &SymbolTableRef) -> Result<ValueType, CompileError> {
        todo!()
    }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     todo!()
    // }

    fn collect_metadata(&self) -> SrcCharsRange {
        todo!()
    }
}
