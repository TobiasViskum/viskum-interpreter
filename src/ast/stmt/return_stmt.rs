use crate::{
    ast::{ ast_symbol_table::AstSymbolTable, expr::{ Expr, ExprTrait } },
    error_handler::{ CompileError, ErrorHandler },
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::{ ExprStmt, StmtTrait };

#[derive(Debug)]
pub struct ReturnStmt {
    return_expr: Option<ExprStmt>,
    metadata: TokenMetadata,
}

impl ReturnStmt {
    pub fn new(return_expr: Option<ExprStmt>, metadata: TokenMetadata) -> Self {
        Self { return_expr, metadata }
    }
}

impl Dissasemble for ReturnStmt {
    fn dissasemble(&self) -> String {
        match &self.return_expr {
            Some(return_expr) => format!("return {}", return_expr.dissasemble()),
            None => "return".to_string(),
        }
    }
}

impl StmtTrait for ReturnStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        if !ast_symbol_table.is_in_func() {
            error_handler.report_compile_error(
                CompileError::new(
                    "Return statements cannot be used outside of functions".to_string(),
                    self.metadata.into()
                )
            )
        } else {
            if let Some(return_value) = &mut self.return_expr {
                match return_value.type_check_and_constant_fold(ast_symbol_table) {
                    Ok(expr_result_ok) => {
                        let resolved_return_type = expr_result_ok.take_value_type();

                        let expected_type = ast_symbol_table
                            .get_current_fn_return_type()
                            .unwrap()
                            .clone();
                        if resolved_return_type != expected_type {
                            error_handler.report_compile_error(
                                CompileError::new(
                                    format!(
                                        "Wrong return type. Expected type {:?} but got type {:?}",
                                        expected_type,
                                        resolved_return_type
                                    ),
                                    return_value.collect_metadata()
                                )
                            )
                        }
                    }
                    Err(expr_result_err) => {
                        error_handler.report_many_compile_errors(expr_result_err.take_errors())
                    }
                }
            }
        }
    }
}
