use crate::{ parser::{ self, token::{ Token, TokenMetadata } }, value::ValueType };

use super::{ expr::{ AstIdentifier, Expr }, type_check::{ AstEnvironment, SymbolTypeDef } };

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VariableDefinition(VariableDefinitionStmt),
    VariableAssignment(VariableAssignmentStmt),
    ScopeStmt(ScopeStmt),
    FunctionStmt(FunctionStmt),
    IfStmt(IfStmt),
    TypeDefStmt(TypeDefStmt),
}

impl Stmt {
    pub fn type_check(
        &mut self,
        ast_environment: &mut parser::ast_generator::AstEnvironment,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<(), String> {
        match self {
            Stmt::IfStmt(_) => panic!("type_check in stmt.rs"),
            Stmt::TypeDefStmt(_) => { Ok(()) }
            Stmt::FunctionStmt(_) => {
                // let mut function_environment = AstEnvironment::new();
                // for parameter in parameters {
                //     function_environment.insert(
                //         parameter.name.to_string(),
                //         parameter.value_type,
                //         parameter.is_mutable,
                //         true
                //     );
                // }

                // body.type_check(&mut function_environment, token_vec)?;

                Ok(())
            }
            Stmt::ScopeStmt(_) => { Ok(()) }
            Stmt::ExprStmt(expr) => {
                expr.type_check(ast_environment, token_vec)?;
                Ok(())
            }
            Stmt::VariableDefinition(variable_definition) => {
                let resulted_value_type = match &variable_definition.value {
                    Some(value) => {
                        match value.type_check(ast_environment, token_vec) {
                            Ok(v) => v,
                            Err(e) => {
                                ast_environment.insert(
                                    variable_definition.name.to_string(),
                                    ValueType::Unkown,
                                    variable_definition.is_mutable,
                                    true
                                );

                                return Err(e);
                            }
                        }
                    }
                    None => ValueType::Empty,
                };

                let value_type = match &variable_definition.value_type {
                    Some(value_type) => {
                        if
                            !&resulted_value_type.is(value_type) &&
                            !&resulted_value_type.is(&ValueType::Empty)
                        {
                            if let Some(value) = &variable_definition.value {
                                value.push_to_token_vec(token_vec);
                            }
                            token_vec.push(variable_definition.token_metadata);

                            ast_environment.insert(
                                variable_definition.name.to_string(),
                                value_type.clone(),
                                variable_definition.is_mutable,
                                true
                            );

                            return Err(
                                format!(
                                    "Variable '{}' is of type {} but the provided value is of type {}",
                                    variable_definition.name,
                                    value_type.to_type_string(),
                                    resulted_value_type.to_type_string()
                                )
                            );
                        } else {
                            value_type
                        }
                    }
                    None => &resulted_value_type,
                };

                ast_environment.insert(
                    variable_definition.name.to_string(),
                    value_type.clone(),
                    variable_definition.is_mutable,
                    !resulted_value_type.is(&ValueType::Empty) // Compare type to if is empty
                );

                variable_definition.value_type = Some(value_type.clone());

                Ok(())
            }
            Stmt::VariableAssignment(variable_assignment) => {
                let resulted_value_type = match
                    variable_assignment.value.type_check(ast_environment, token_vec)
                {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let variable = ast_environment.get(&variable_assignment.field.get_lexeme());

                if let Some((value_type, is_mutable, is_initialized)) = variable {
                    if !is_initialized {
                        println!(
                            "Found non-initialized variable: '{}'.\nTODO: Implement checks when conditional blocks are implemented.",
                            variable_assignment.field.get_lexeme()
                        );
                    }
                    if !is_mutable && is_initialized {
                        variable_assignment.value.push_to_token_vec(token_vec);
                        variable_assignment.field.push_to_token_vec(token_vec);

                        return Err(
                            format!(
                                "Cannot mutate immutable variable '{}'",
                                variable_assignment.field.lexeme
                            )
                        );
                    } else {
                        if !&resulted_value_type.is(&value_type) {
                            variable_assignment.value.push_to_token_vec(token_vec);
                            variable_assignment.field.push_to_token_vec(token_vec);

                            let error_message = if value_type.is(&ValueType::Unkown) {
                                format!(
                                    "The tye of variable '{}' is unknown and cannot be assigned to",
                                    variable_assignment.field.lexeme
                                )
                            } else {
                                format!(
                                    "Variable '{}' is of type {} but the assignment value is of type {}",
                                    variable_assignment.field.lexeme,
                                    value_type.to_type_string(),
                                    resulted_value_type.to_type_string()
                                )
                            };

                            return Err(error_message);
                        }
                    }
                } else {
                    variable_assignment.value.push_to_token_vec(token_vec);
                    variable_assignment.field.push_to_token_vec(token_vec);

                    return Err(
                        format!(
                            "Cannot assign to undefined variable: '{}'",
                            variable_assignment.field.lexeme
                        )
                    );
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct VariableAssignmentStmt {
    pub target_expr: Option<Expr>,
    pub field: AstIdentifier,
    pub value: Expr,
}

impl VariableAssignmentStmt {
    pub fn new(target_expr: Option<Expr>, field: AstIdentifier, value: Expr) -> Self {
        Self {
            target_expr,
            field,
            value,
        }
    }
}

#[derive(Debug)]
pub struct VariableDefinitionStmt {
    pub name: String,
    pub value_type: Option<ValueType>,
    pub is_mutable: bool,
    pub value: Option<Expr>,
    pub token_metadata: TokenMetadata,
}

impl VariableDefinitionStmt {
    pub fn new(
        name: String,
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
pub struct IfStmt {
    condition: Expr,
    true_block: ScopeStmt,
    false_block: Option<Box<IfStmt>>,
}

impl IfStmt {
    pub fn new(condition: Expr, true_block: ScopeStmt, false_block: Option<Box<IfStmt>>) -> Self {
        Self {
            condition,
            true_block,
            false_block,
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

    // pub fn type_check(&self, ast_environment: &mut AstEnvironment) {
    //     self.forward_declare(ast_environment);

    //     for stmt in &self.stmts {
    //         match stmt {

    //         }
    //     }
    // }

    pub fn forward_declare(&self, ast_environment: &mut AstEnvironment) {
        for stmt in &self.cf_stmts {
            match stmt {
                Stmt::TypeDefStmt(type_def_stmt) => {
                    let type_name = type_def_stmt.type_name.clone();
                    let typing_value = match &type_def_stmt.typing.typing_value {
                        TypingValue::Custom(_) => { panic!("Custom types not supported yet") }
                        TypingValue::ValueType(value_type) => { value_type.clone() }
                    };

                    ast_environment.scopes[ast_environment.scope_depth].type_definitions.insert(
                        type_name,
                        SymbolTypeDef {
                            value_type: typing_value,
                        }
                    );
                }
                _ => {}
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
    pub name: String,
    pub value_type: ValueType,
    pub is_mutable: bool,
}

#[derive(Debug)]
pub struct FunctionStmt {
    pub name: String,
    pub args: Vec<FunctionArgument>,
    pub return_type: Option<ValueType>,
    pub body: ScopeStmt,
}

impl FunctionStmt {
    pub fn new(
        name: String,
        args: Vec<FunctionArgument>,
        return_type: Option<ValueType>,
        body: ScopeStmt
    ) -> Self {
        Self {
            name,
            args,
            return_type,
            body,
        }
    }
}

#[derive(Debug)]
pub struct TypeDefStmt {
    pub type_name: String,
    pub typing: Typing,
}

impl TypeDefStmt {
    pub fn new(type_name: String, typing: Typing) -> Self {
        Self {
            type_name,
            typing,
        }
    }
}

#[derive(Debug)]
pub enum TypingValue {
    ValueType(ValueType),
    Custom(String),
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

    pub fn new_custom(custom_name: String, token_metadata: TokenMetadata) -> Self {
        Self {
            typing_value: TypingValue::Custom(custom_name),
            token_metadata,
            type_args: None,
        }
    }
}
