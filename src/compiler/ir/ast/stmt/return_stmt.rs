use crate::compiler::{
    ds::symbol_table::SymbolTableRef,
    error_handler::{ CompileError, ErrorHandler, ReportedError },
    ir::ast::expr::ExprTrait,
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::{ ExprStmt, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct ReturnStmt<'ast> {
    return_expr: Option<ExprStmt<'ast>>,
    metadata: TokenMetadata,
}

impl<'ast> ReturnStmt<'ast> {
    pub fn new(return_expr: Option<ExprStmt<'ast>>, metadata: TokenMetadata) -> Self {
        Self { return_expr, metadata }
    }
}

impl<'ast> Dissasemble for ReturnStmt<'ast> {
    fn dissasemble(&self) -> String {
        match &self.return_expr {
            Some(return_expr) => format!("return {}", return_expr.dissasemble()),
            None => "return".to_string(),
        }
    }
}

impl<'ast> StmtTrait for ReturnStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        let provided_return_type = match symbol_table_ref.get().get_fn_return_type() {
            None => {
                error_handler.report_compile_error(
                    CompileError::new(
                        ReportedError::new(
                            "Return statements cannot be used outside of functions".to_string(),
                            self.metadata.into()
                        )
                    )
                );
                return;
            }
            Some(v) => { Some(v) }
        };

        let found_return_type = match &mut self.return_expr {
            Some(v) => {
                match v.type_check(symbol_table_ref) {
                    Ok(v) => Some(v),
                    Err(err) => {
                        error_handler.report_compile_error(err);
                        return;
                    }
                }
            }
            None => None,
        };

        match (found_return_type, provided_return_type) {
            (None, None) => {}
            (Some(found_return_type), None) => {
                error_handler.report_compile_error(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Expected return type: '()' but got '{}'",
                                found_return_type.dissasemble()
                            ),
                            self.metadata.into()
                        )
                    )
                )
            }
            (None, Some(provided_return_type)) => {
                error_handler.report_compile_error(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Expected return type: '{}' but got '()'",
                                provided_return_type.dissasemble()
                            ),
                            self.metadata.into()
                        )
                    )
                )
            }
            (Some(found_return_type), Some(provided_return_type)) => {
                if !found_return_type.is(provided_return_type) {
                    error_handler.report_compile_error(
                        CompileError::new(
                            ReportedError::new(
                                format!(
                                    "Expected return type: '{}' but got '{}'",
                                    provided_return_type.dissasemble(),
                                    found_return_type.dissasemble()
                                ),
                                self.metadata.into()
                            )
                        )
                    )
                }
            }
        };
    }
}
