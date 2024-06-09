use std::rc::Rc;

use ahash::AHashMap;

use crate::{
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    merge_chars_range,
    parser::token::TokenMetadata,
    value::ValueType,
};

use super::{
    expr::{ Expr, ExprTrait },
    stmt::{ FunctionArgument, FunctionStmt, VarAssignStmt, VarDefStmt },
};

#[derive(Debug)]
pub struct AstSymbolTable {
    symbols: Vec<AHashMap<Rc<str>, AstSymbol>>,
    current_function_return_type: Vec<ValueType>,
}

impl AstSymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: vec![AHashMap::new()],
            current_function_return_type: Vec::new(),
        }
    }

    pub fn is_in_func(&self) -> bool {
        self.current_function_return_type.len() != 0
    }

    pub fn get_current_fn_return_type(&self) -> Option<&ValueType> {
        self.current_function_return_type.last()
    }

    pub fn push_function_return_type(&mut self, return_type: ValueType) {
        self.current_function_return_type.push(return_type);
    }

    pub fn pop_function_return_type(&mut self) {
        self.current_function_return_type.pop();
    }

    pub fn increment_scope(&mut self) {
        self.symbols.push(AHashMap::new());
    }

    pub fn decrement_scope(&mut self) {
        self.symbols.pop();
    }

    pub fn get(&self, lexeme: &Rc<str>) -> Option<&AstSymbol> {
        for table in self.symbols.iter().rev() {
            if let Some(symbol) = table.get(lexeme) {
                return Some(symbol);
            }
        }

        None
    }

    pub fn declare_function(
        &mut self,
        function_stmt: &FunctionStmt,
        error_handler: &mut ErrorHandler
    ) {
        self.symbols
            .last_mut()
            .unwrap()
            .insert(
                function_stmt.get_name(),
                AstSymbol::Function(
                    AstFunction::new(
                        function_stmt.get_args().clone(),
                        function_stmt.get_return_type().clone(),
                        function_stmt.get_metadata()
                    )
                )
            );
    }

    pub fn assign_variable(
        &mut self,
        var_assign_stmt: &mut VarAssignStmt,
        error_handler: &mut ErrorHandler
    ) {
        let ident_expr = match &var_assign_stmt.target_expr.get_expr() {
            Expr::IdentifierExpr(ident) => ident,
            _ => panic!("target_expr can only be an identifier for now"),
        };

        let symbol = match self.get(&ident_expr.get_lexeme()) {
            Some(symbol) => symbol,
            None => {
                error_handler.report_compile_error(
                    CompileError::new(
                        format!(
                            "Undefined variable: {}. Functions does not capture variables from the surrounding scope",
                            ident_expr.get_lexeme()
                        ),
                        ident_expr.collect_metadata()
                    )
                );
                return;
            }
        };

        match &symbol {
            AstSymbol::Variable(var) => {
                let value_type = match var_assign_stmt.value.type_check_and_constant_fold(self) {
                    Ok(expr_result_ok) => expr_result_ok.take_value_type(),
                    Err(expr_result_err) => {
                        error_handler.report_many_compile_errors(expr_result_err.take_errors());
                        return;
                    }
                };

                if !var.mutable {
                    error_handler.report_compile_error(
                        CompileError::new(
                            format!(
                                "Cannot mutate immutable variable: {}",
                                ident_expr.get_lexeme()
                            ),
                            ident_expr.collect_metadata()
                        )
                    );
                    return;
                }

                if !var.value_type.is(&value_type) {
                    error_handler.report_compile_error(
                        CompileError::new(
                            format!(
                                "Variable '{}' is of type {:?} but was assigned to type {:?}",
                                ident_expr.get_lexeme(),
                                var.value_type,
                                value_type
                            ),
                            var_assign_stmt.value.collect_metadata()
                        )
                    )
                }
            }
            AstSymbol::Function(_) => {
                error_handler.report_compile_error(
                    CompileError::new(
                        "Cannot assign a function".to_string(),
                        ident_expr.collect_metadata()
                    )
                );
            }
        }
    }

    pub fn insert_variable(
        &mut self,
        name: Rc<str>,
        value_type: ValueType,
        mutable: bool,
        metadata: TokenMetadata
    ) {
        self.symbols
            .last_mut()
            .unwrap()
            .insert(name, AstSymbol::Variable(AstVariable::new(value_type, mutable, metadata)));
    }

    pub fn declare_variable(
        &mut self,
        var_def_stmt: &mut VarDefStmt,
        error_handler: &mut ErrorHandler
    ) {
        match &mut var_def_stmt.value {
            Some(value_expr) => {
                let value_expr_type = match value_expr.type_check_and_constant_fold(self) {
                    Ok(expr_result_ok) => expr_result_ok.take_value_type(),
                    Err(expr_result_err) => {
                        error_handler.report_many_compile_errors(expr_result_err.take_errors());
                        return;
                    }
                };

                if let Some(provided_type) = &var_def_stmt.value_type {
                    if value_expr_type != *provided_type {
                        error_handler.report_compile_error(
                            CompileError::new(
                                format!(
                                    "Variable '{}' is of type {:?} but found {:?}",
                                    var_def_stmt.name,
                                    provided_type,
                                    value_expr_type
                                ),

                                merge_chars_range!(
                                    var_def_stmt.token_metadata.into(),
                                    value_expr.collect_metadata()
                                )
                            )
                        );
                    } else {
                        self.insert_variable(
                            Rc::clone(&var_def_stmt.name),
                            provided_type.clone(),
                            var_def_stmt.is_mutable,
                            var_def_stmt.token_metadata
                        );
                    }
                } else {
                    self.insert_variable(
                        Rc::clone(&var_def_stmt.name),
                        value_expr_type,
                        var_def_stmt.is_mutable,
                        var_def_stmt.token_metadata
                    );
                }
            }
            None => {
                panic!("Variables without initial value are not supported yet");
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstSymbol {
    Variable(AstVariable),
    Function(AstFunction),
}

#[derive(Debug, Clone)]
pub struct AstVariable {
    pub metadata: TokenMetadata,
    pub value_type: ValueType,
    pub mutable: bool,
}

impl AstVariable {
    pub fn new(value_type: ValueType, mutable: bool, metadata: TokenMetadata) -> Self {
        Self {
            value_type,
            mutable,
            metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstFunction {
    pub args: Vec<FunctionArgument>,
    pub return_type: ValueType,
    pub metadata: TokenMetadata,
}

impl AstFunction {
    pub fn new(
        args: Vec<FunctionArgument>,
        return_type: ValueType,
        metadata: TokenMetadata
    ) -> Self {
        Self {
            args,
            return_type,
            metadata,
        }
    }
}
