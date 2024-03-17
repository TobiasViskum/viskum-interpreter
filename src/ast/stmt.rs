use crate::{ parser::{ ast_generator::AstEnvironment, token::TokenMetadata }, value::ValueType };

use super::{ expr::Expr, AstIdentifier };

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VariableDefinition(VariableDefinitionStmt),
    VariableAssignment(VariableAssignmentStmt),
    ScopeStmt(ScopeStmt),
}

impl Stmt {
    pub fn type_check(
        &mut self,
        ast_environment: &mut AstEnvironment,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<(), String> {
        match self {
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
                            &resulted_value_type != value_type &&
                            &resulted_value_type != &ValueType::Empty
                        {
                            if let Some(value) = &variable_definition.value {
                                value.push_to_token_vec(token_vec);
                            }
                            token_vec.push(variable_definition.token_metadata);

                            ast_environment.insert(
                                variable_definition.name.to_string(),
                                *value_type,
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
                    *value_type,
                    variable_definition.is_mutable,
                    resulted_value_type != ValueType::Empty // Compare type to if is empty
                );

                variable_definition.value_type = Some(*value_type);

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
                        if &resulted_value_type != &value_type {
                            variable_assignment.value.push_to_token_vec(token_vec);
                            variable_assignment.field.push_to_token_vec(token_vec);

                            let error_message = if &value_type == &ValueType::Unkown {
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
pub struct ScopeStmt {
    pub stmts: Vec<Stmt>,
}

impl ScopeStmt {
    pub fn new() -> Self {
        Self {
            stmts: Vec::new(),
        }
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
}
