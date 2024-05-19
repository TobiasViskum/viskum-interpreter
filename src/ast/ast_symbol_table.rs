use std::rc::Rc;

use ahash::AHashMap;

use crate::{ error_handler::ErrorHandler, parser::token::TokenMetadata, value::ValueType };

use super::{ expr::Expr, stmt::{ FunctionArgument, FunctionStmt, VarAssignStmt, VarDefStmt } };

#[derive(Debug)]
pub struct AstSymbolTable {
    symbols: Vec<AHashMap<Rc<str>, AstSymbol>>,
    current_scope: usize,
    in_function: bool,
}

impl AstSymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: vec![AHashMap::new()],
            current_scope: 0,
            in_function: false,
        }
    }

    pub fn is_in_func(&self) -> bool {
        self.in_function
    }

    pub fn set_in_func(&mut self, val: bool) {
        self.in_function = val;
    }

    pub fn increment_scope(&mut self) {
        self.current_scope += 1;
        self.symbols.push(AHashMap::new());
    }

    pub fn decrement_scope(&mut self) {
        self.current_scope -= 1;
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
        self.symbols[self.current_scope].insert(
            Rc::clone(&function_stmt.name),
            AstSymbol::Function(
                AstFunction::new(
                    function_stmt.args.clone(),
                    function_stmt.return_type.or(Some(ValueType::Void)).unwrap(),
                    function_stmt.metadata
                )
            )
        );
    }

    pub fn assign_variable(
        &mut self,
        var_assign_stmt: &mut VarAssignStmt,
        error_handler: &mut ErrorHandler
    ) {
        let ident = match &var_assign_stmt.target_expr {
            Expr::IdentifierLookup(ident) => ident,
            _ => panic!("target_expr can only be an identifier for now"),
        };

        let symbol = match self.get(&ident.lexeme) {
            Some(symbol) => symbol,
            None => {
                error_handler.report_compile_error(
                    format!(
                        "Undefined variable: {}. Functions does not capture variables from the surrounding scope",
                        ident.lexeme
                    ),
                    vec![ident.token_metadata]
                );
                return;
            }
        };

        match &symbol {
            AstSymbol::Variable(var) => {
                let mut token_vec: Vec<TokenMetadata> = vec![];
                let value_type = match var_assign_stmt.value.type_check(&self, &mut token_vec) {
                    Ok(v) => v,
                    Err(e) => {
                        error_handler.report_compile_error(e, token_vec);
                        return;
                    }
                };

                if !var.mutable {
                    error_handler.report_compile_error(
                        format!("Cannot mutate immutable variable: {}", ident.get_lexeme()),
                        vec![ident.get_token_metadata()]
                    );
                    return;
                }

                if !var.value_type.is(&value_type) {
                    error_handler.report_compile_error(
                        format!(
                            "Variable '{}' is of type {:?} but was assigned to type {:?}",
                            ident.get_lexeme(),
                            var.value_type,
                            value_type
                        ),
                        token_vec
                    )
                }
            }
            AstSymbol::Function(func) => {
                error_handler.report_compile_error(
                    "Cannot assign a function".to_string(),
                    vec![ident.get_token_metadata()]
                );
            }
            AstSymbol::Error => {}
        }
    }

    pub fn insert_variable(
        &mut self,
        name: Rc<str>,
        value_type: ValueType,
        mutable: bool,
        metadata: TokenMetadata
    ) {
        self.symbols[self.current_scope].insert(
            name,
            AstSymbol::Variable(AstVariable::new(value_type, mutable, metadata))
        );
    }

    pub fn declare_variable(
        &mut self,
        var_def_stmt: &mut VarDefStmt,
        error_handler: &mut ErrorHandler
    ) {
        let value_type = match &mut var_def_stmt.value {
            Some(value) => {
                let mut token_vec: Vec<TokenMetadata> = vec![];
                match value.type_check(&self, &mut token_vec) {
                    Ok(v) => {
                        if let Some(value_type) = var_def_stmt.value_type {
                            if !value_type.is(&v) {
                                error_handler.report_compile_error(
                                    format!(
                                        "Variable '{}' is of type {:?} but found {:?}",
                                        var_def_stmt.name,
                                        value_type,
                                        v
                                    ),
                                    vec![var_def_stmt.token_metadata]
                                );
                                Some(value_type)
                            } else {
                                Some(value_type)
                            }
                        } else {
                            Some(v)
                        }
                    }
                    Err(e) => {
                        error_handler.report_compile_error(e, token_vec);
                        None
                    }
                }
            }
            None => { panic!("Should be unreachable") }
        };

        let ast_symbol = if value_type.is_none() {
            AstSymbol::Error
        } else {
            AstSymbol::Variable(
                AstVariable::new(
                    value_type.unwrap(),
                    var_def_stmt.is_mutable,
                    var_def_stmt.token_metadata
                )
            )
        };

        self.symbols[self.current_scope].insert(var_def_stmt.name.clone(), ast_symbol);
    }
}

#[derive(Debug, Clone)]
pub enum AstSymbol {
    Variable(AstVariable),
    Function(AstFunction),
    Error,
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
