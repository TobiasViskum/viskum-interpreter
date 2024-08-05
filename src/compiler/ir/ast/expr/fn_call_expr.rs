use std::rc::Rc;

use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::ValueType },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::{ ast::stmt::ExprStmt, icfg::dag::DAG },
        parser::token::TokenMetadata,
        print_todo,
        traits::{ Dissasemble, ExprTrait, SymbolTableActions },
    },
    macros::merge_chars_range,
};

#[derive(Debug)]
pub struct FnCallExpr<'ast> {
    lexeme: Rc<str>,
    metadata: TokenMetadata,
    args: Vec<ExprStmt<'ast>>,
    real_function_args: Vec<Rc<str>>,
    return_type: ValueType,
}

impl<'ast> FnCallExpr<'ast> {
    pub fn new(lexeme: Rc<str>, metadata: TokenMetadata, args: Vec<ExprStmt<'ast>>) -> Self {
        Self {
            lexeme,
            metadata,
            args,
            real_function_args: vec![],
            return_type: ValueType::Void,
        }
    }
}

impl<'ast> Dissasemble for FnCallExpr<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = format!("{}(", self.lexeme);
        for i in 0..self.args.len() {
            string_builder += &self.args[i].dissasemble();
            if i != 0 {
                string_builder += ", ";
            }
        }
        string_builder += ")";
        string_builder
    }
}

impl<'ast> ExprTrait for FnCallExpr<'ast> {
    // fn evaluate(&mut self, _: &AstSymbolTable) -> ExprEvaluateResult {
    //     Err(
    //         ExprEvaluateErr::InternalError(
    //             InternalError::new(InternalErrorCode::Compile(5), "Cannot evaluate function call")
    //         )
    //     )
    // }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        todo!()
    }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        let symbol_fn = symbol_table_ref
            .get()
            .lookup_as_fn(&self.lexeme)
            .or_else(|msg|
                Err(CompileError::new(ReportedError::new(msg, self.collect_metadata())))
            )?;

        print_todo("Compare amount of args and each arg value type");

        Ok(symbol_fn.get_return_type().clone())
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        let metadata = self.metadata.into();

        merge_chars_range!(
            metadata,
            self.args
                .iter()
                .map(|arg| arg.collect_metadata())
                .collect()
        )
    }
}
