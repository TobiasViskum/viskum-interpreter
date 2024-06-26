use std::rc::Rc;

use crate::{
    compiler::{
        ds::{ symbol_table::{ SymbolTableActions, SymbolTableRef }, value::ValueType },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::{ ast::stmt::ExprStmt, icfg::dag::DAG },
        parser::token::TokenMetadata,
        print_todo,
        Dissasemble,
    },
    macros::merge_chars_range,
};

use super::ExprTrait;

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

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        todo!()
    }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     match ast_symbol_table.get(&self.lexeme, &self.metadata) {
    //         Ok(symbol) => {
    //             match symbol {
    //                 Some(symbol) => {
    //                     match symbol {
    //                         AstSymbol::Function { .. } => {}
    //                         AstSymbol::Variable { .. } => {
    //                             return Err(
    //                                 ExprResultErr::new(
    //                                     vec![
    //                                         CompileError::new(
    //                                             ReportedError::new(
    //                                                 format!(
    //                                                     "Undefined function '{}'. A variable with the same name exists",
    //                                                     self.lexeme
    //                                                 ),
    //                                                 self.metadata.into()
    //                                             )
    //                                         )
    //                                     ]
    //                                 )
    //                             );
    //                         }
    //                     }
    //                 }
    //                 None => {
    //                     return Err(
    //                         ExprResultErr::new(
    //                             vec![
    //                                 CompileError::new(
    //                                     ReportedError::new(
    //                                         format!("Undefined function '{}'", self.lexeme),
    //                                         self.metadata.into()
    //                                     )
    //                                 )
    //                             ]
    //                         )
    //                     );
    //                 }
    //             }
    //         }
    //         Err(err) => {
    //             return ExprResult::Err(ExprResultErr::new(vec![err]));
    //         }
    //     }

    //     let mut compile_errors = vec![];

    //     for i in 0..self.args.len() {
    //         match self.real_function_args.get(i) {
    //             Some(real_arg_type) => {
    //                 let arg_type = self.args[i].type_check_and_constant_fold(ast_symbol_table)?;
    //                 print_todo("Check if args type matches is not implemented yet");
    //                 // if arg_type != *real_arg_type {
    //                 //     compile_errors.push(
    //                 //         CompileError::new(
    //                 //             format!(
    //                 //                 "Argument '{}' expected to be of type {:?} but received type {:?}",
    //                 //                 arg_name,
    //                 //                 real_arg_type,
    //                 //                 arg_type
    //                 //             ),
    //                 //             self.args[i].collect_metadata()
    //                 //         )
    //                 //     );
    //                 // }
    //             }
    //             None => {
    //                 if self.real_function_args.len() > self.args.len() {
    //                     return Err(
    //                         ExprResultErr::new(
    //                             vec![
    //                                 CompileError::new(
    //                                     ReportedError::new(
    //                                         format!(
    //                                             "Expected {} args but got {}",
    //                                             self.real_function_args.len(),
    //                                             self.args.len()
    //                                         ),
    //                                         merge_chars_range!(
    //                                             self.args
    //                                                 .iter()
    //                                                 .map(|arg| arg.collect_metadata())
    //                                                 .collect()
    //                                         )
    //                                     )
    //                                 )
    //                             ]
    //                         )
    //                     );
    //                 }
    //             }
    //         }
    //     }

    //     if compile_errors.len() > 0 {
    //         return Err(ExprResultErr::new(compile_errors));
    //     }

    //     Ok(ExprResultOk::new(self.return_type.clone(), false))
    // }

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
