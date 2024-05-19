use std::rc::Rc;

use crate::{
    error_handler::ErrorHandler,
    parser::{ self, token::{ Token, TokenMetadata } },
    value::ValueType,
};

use super::{ ast_symbol_table::{ AstSymbol, AstSymbolTable }, expr::{ AstIdentifier, Expr } };

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VarDefStmt(VarDefStmt),
    VarAssignStmt(VarAssignStmt),
    ScopeStmt(ScopeStmt),
    FunctionStmt(FunctionStmt),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ReturnStmt(ReturnStmt),
    IfStmt(IfStmt),
    LoopStmt(LoopStmt),
    TypeDefStmt(TypeDefStmt),
}

// impl Stmt {
//     pub fn type_check(
//         &mut self,
//         ast_environment: &mut parser::ast_generator::AstEnvironment,
//         token_vec: &mut Vec<TokenMetadata>
//     ) -> Result<(), String> {
//         match self {
//             Stmt::ContinueStmt(_) => { Ok(()) }
//             Stmt::BreakStmt(_) => { Ok(()) }
//             Stmt::LoopStmt(_) => panic!("type_check in stmt.rs"),
//             Stmt::IfStmt(_) => panic!("type_check in stmt.rs"),
//             Stmt::TypeDefStmt(_) => panic!("type_check in stmt.rs"),
//             Stmt::FunctionStmt(_) => panic!("type_check in stmt.rs"),
//             Stmt::ScopeStmt(_) => { Ok(()) }
//             Stmt::ExprStmt(expr) => {
//                 expr.type_check(ast_environment, token_vec)?;
//                 Ok(())
//             }
//             Stmt::VarDefStmt(variable_definition) => {
//                 let resulted_value_type = match &variable_definition.value {
//                     Some(value) => {
//                         match value.type_check(ast_environment, token_vec) {
//                             Ok(v) => v,
//                             Err(e) => {
//                                 ast_environment.insert(
//                                     variable_definition.name.to_string(),
//                                     ValueType::Empty,
//                                     variable_definition.is_mutable,
//                                     true
//                                 );

//                                 return Err(e);
//                             }
//                         }
//                     }
//                     None => ValueType::Empty,
//                 };

//                 let value_type = match &variable_definition.value_type {
//                     Some(value_type) => {
//                         if
//                             !&resulted_value_type.is(value_type) &&
//                             !&resulted_value_type.is(&ValueType::Empty)
//                         {
//                             if let Some(value) = &variable_definition.value {
//                                 value.push_to_token_vec(token_vec);
//                             }
//                             token_vec.push(variable_definition.token_metadata);

//                             ast_environment.insert(
//                                 variable_definition.name.to_string(),
//                                 value_type.clone(),
//                                 variable_definition.is_mutable,
//                                 true
//                             );

//                             return Err(
//                                 format!(
//                                     "Variable '{}' is of type {} but the provided value is of type {}",
//                                     variable_definition.name,
//                                     value_type.to_type_string(),
//                                     resulted_value_type.to_type_string()
//                                 )
//                             );
//                         } else {
//                             value_type
//                         }
//                     }
//                     None => &resulted_value_type,
//                 };

//                 ast_environment.insert(
//                     variable_definition.name.to_string(),
//                     value_type.clone(),
//                     variable_definition.is_mutable,
//                     !resulted_value_type.is(&ValueType::Empty) // Compare type to if is empty
//                 );

//                 variable_definition.value_type = Some(value_type.clone());

//                 Ok(())
//             }
//             Stmt::VarAssignStmt(variable_assignment) => {
//                 let resulted_value_type = match
//                     variable_assignment.value.type_check(ast_environment, token_vec)
//                 {
//                     Ok(v) => v,
//                     Err(e) => {
//                         return Err(e);
//                     }
//                 };

//                 let variable = ast_environment.get(&variable_assignment.temp_get_lexeme());

//                 if let Some((value_type, is_mutable, is_initialized)) = variable {
//                     if !is_initialized {
//                         println!(
//                             "Found non-initialized variable: '{}'.\nTODO: Implement checks when conditional blocks are implemented.",
//                             variable_assignment.temp_get_lexeme()
//                         );
//                     }
//                     if !is_mutable && is_initialized {
//                         variable_assignment.value.push_to_token_vec(token_vec);

//                         return Err(
//                             format!(
//                                 "Cannot mutate immutable variable '{}'",
//                                 variable_assignment.temp_get_lexeme()
//                             )
//                         );
//                     } else {
//                         // if !&resulted_value_type.is(&value_type) {
//                         //     variable_assignment.value.push_to_token_vec(token_vec);

//                         //     let error_message = if value_type.is(&ValueType::Unkown) {
//                         //         format!(
//                         //             "The tye of variable '{}' is unknown and cannot be assigned to",
//                         //             variable_assignment.temp_get_lexeme()
//                         //         )
//                         //     } else {
//                         //         format!(
//                         //             "Variable '{}' is of type {} but the assignment value is of type {}",
//                         //             variable_assignment.temp_get_lexeme(),
//                         //             value_type.to_type_string(),
//                         //             resulted_value_type.to_type_string()
//                         //         )
//                         //     };

//                         //     return Err(error_message);
//                         // }
//                     }
//                 } else {
//                     variable_assignment.value.push_to_token_vec(token_vec);

//                     return Err(
//                         format!(
//                             "Cannot assign to undefined variable: '{}'",
//                             variable_assignment.temp_get_lexeme()
//                         )
//                     );
//                 }

//                 Ok(())
//             }
//         }
//     }
// }

#[derive(Debug)]
pub struct VarAssignStmt {
    pub target_expr: Expr,
    pub value: Expr,
}

impl VarAssignStmt {
    pub fn new(target_expr: Expr, value: Expr) -> Self {
        Self {
            target_expr,
            value,
        }
    }

    pub fn temp_get_lexeme(&self) -> Rc<str> {
        match &self.target_expr {
            Expr::IdentifierLookup(ident) => ident.get_lexeme(),
            _ => panic!("temp_get_lexeme"),
        }
    }
}

#[derive(Debug)]
pub struct VarDefStmt {
    pub name: Rc<str>,
    pub value_type: Option<ValueType>,
    pub is_mutable: bool,
    pub value: Option<Expr>,
    pub token_metadata: TokenMetadata,
}

impl VarDefStmt {
    pub fn new(
        name: Rc<str>,
        value_type: Option<ValueType>,
        is_mutable: bool,
        value: Option<Expr>,
        token_metadata: TokenMetadata
    ) -> Self {
        Self {
            name,
            value_type,
            is_mutable,
            value,
            token_metadata,
        }
    }
}

#[derive(Debug)]
pub struct BreakStmt;

impl BreakStmt {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub return_expr: Option<Expr>,
    pub metadata: TokenMetadata,
}

impl ReturnStmt {
    pub fn new(return_expr: Option<Expr>, metadata: TokenMetadata) -> Self {
        Self { return_expr, metadata }
    }
}

#[derive(Debug)]
pub struct ContinueStmt;

impl ContinueStmt {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct LoopStmt {
    pub condition: Option<Expr>,
    pub body: ScopeStmt,
}

impl LoopStmt {
    pub fn new(condition: Option<Expr>, body: ScopeStmt) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Option<Expr>,
    pub true_block: ScopeStmt,
    pub false_block: Option<Box<IfStmt>>,
}

impl IfStmt {
    pub fn new(
        condition: Option<Expr>,
        true_block: ScopeStmt,
        false_block: Option<Box<IfStmt>>
    ) -> Self {
        Self {
            condition,
            true_block,
            false_block,
        }
    }

    pub fn type_check(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        if let Some(condition) = &mut self.condition {
            let token_vec = &mut vec![];
            let _ = condition.type_check(ast_symbol_table, token_vec);
        }

        self.true_block.type_check(ast_symbol_table, error_handler);

        if let Some(false_block) = &mut self.false_block {
            let _ = false_block.type_check(ast_symbol_table, error_handler);
        }
    }
}

#[derive(Debug)]
pub struct ScopeStmt {
    pub cf_stmts: Vec<Stmt>,
    pub forwards_declarations: Vec<Stmt>, // TypeDefStmt, FnStmt, (ClassStmt)
}

impl ScopeStmt {
    pub fn new() -> Self {
        Self {
            cf_stmts: Vec::new(),
            forwards_declarations: Vec::new(),
        }
    }

    pub fn type_check(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        for stmt in &mut self.cf_stmts {
            match stmt {
                Stmt::VarAssignStmt(var_assign_stmt) => {
                    ast_symbol_table.assign_variable(var_assign_stmt, error_handler);
                }
                Stmt::VarDefStmt(var_def_stmt) => {
                    ast_symbol_table.declare_variable(var_def_stmt, error_handler);
                }
                Stmt::ScopeStmt(ref mut scope_stmt) => {
                    ast_symbol_table.increment_scope();

                    scope_stmt.type_check(ast_symbol_table, error_handler);

                    ast_symbol_table.decrement_scope();
                }
                Stmt::FunctionStmt(function_stmt) => {
                    ast_symbol_table.declare_function(function_stmt, error_handler);

                    let mut ast_symbol_table = AstSymbolTable::new();
                    ast_symbol_table.set_in_func(true);
                    ast_symbol_table.declare_function(function_stmt, error_handler);

                    for arg in &function_stmt.args {
                        ast_symbol_table.insert_variable(
                            arg.name.clone(),
                            arg.value_type,
                            arg.is_mutable,
                            arg.metadata
                        );
                    }
                    function_stmt.body.type_check(&mut ast_symbol_table, error_handler);
                }
                Stmt::ReturnStmt(return_stmt) => {
                    if !ast_symbol_table.is_in_func() {
                        error_handler.report_compile_error(
                            "Return statements cannot be used outside of functions".to_string(),
                            vec![return_stmt.metadata]
                        );
                    }

                    if let Some(return_value) = &mut return_stmt.return_expr {
                        let token_vec = &mut vec![];
                        let _ = return_value.type_check(ast_symbol_table, token_vec);
                    }
                }
                Stmt::ExprStmt(expr) => {
                    let mut token_vec = vec![];
                    match expr.type_check(ast_symbol_table, &mut token_vec) {
                        Ok(_) => {}
                        Err(e) => {
                            error_handler.report_compile_error(e, token_vec);
                        }
                    }
                }
                Stmt::IfStmt(if_stmt) => {
                    if_stmt.type_check(ast_symbol_table, error_handler);
                }
                c => {
                    eprint!("type_check doesn't support given stmt: {:?}", c);
                }
            }
        }
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.cf_stmts.push(stmt);
    }

    pub fn push_forward_stmt(&mut self, stmt: Stmt) {
        self.forwards_declarations.push(stmt)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: Rc<str>,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub metadata: TokenMetadata,
}

#[derive(Debug)]
pub struct FunctionStmt {
    pub name: Rc<str>,
    pub args: Vec<FunctionArgument>,
    pub return_type: Option<ValueType>,
    pub body: ScopeStmt,
    pub metadata: TokenMetadata,
}

impl FunctionStmt {
    pub fn new(
        name: Rc<str>,
        args: Vec<FunctionArgument>,
        return_type: Option<ValueType>,
        body: ScopeStmt,
        metadata: TokenMetadata
    ) -> Self {
        Self {
            name,
            args,
            return_type,
            body,
            metadata,
        }
    }
}

#[derive(Debug)]
pub struct TypeDefStmt {
    pub type_name: Rc<str>,
    pub typing: Typing,
}

impl TypeDefStmt {
    pub fn new(type_name: Rc<str>, typing: Typing) -> Self {
        Self {
            type_name,
            typing,
        }
    }
}

#[derive(Debug)]
pub enum TypingValue {
    ValueType(ValueType),
    Custom(Rc<str>),
}

#[derive(Debug)]
pub struct Typing {
    pub typing_value: TypingValue,
    pub token_metadata: TokenMetadata,
    pub type_args: Option<Vec<Typing>>,
}

impl Typing {
    pub fn new(
        typing_value: TypingValue,
        token_metadata: TokenMetadata,
        type_args: Option<Vec<Typing>>
    ) -> Self {
        Self {
            typing_value,
            token_metadata,
            type_args,
        }
    }

    pub fn new_int32(token_metadata: TokenMetadata) -> Self {
        Self {
            typing_value: TypingValue::ValueType(ValueType::Int32),
            token_metadata,
            type_args: None,
        }
    }

    pub fn new_bool(token_metadata: TokenMetadata) -> Self {
        Self {
            typing_value: TypingValue::ValueType(ValueType::Bool),
            token_metadata,
            type_args: None,
        }
    }

    pub fn new_custom(custom_name: Rc<str>, token_metadata: TokenMetadata) -> Self {
        Self {
            typing_value: TypingValue::Custom(custom_name),
            token_metadata,
            type_args: None,
        }
    }
}
