use std::collections::HashMap;

use crate::{
    operations::{ BinaryOp, UnaryOp },
    parser::token::TokenMetadata,
    value::{ Value, ValueType },
};

#[derive(Debug, Clone)]
pub struct AstValue {
    value: Value,
    token_metadata: TokenMetadata,
}

impl AstValue {
    pub fn new(value: Value, token_metadata: TokenMetadata) -> Self {
        Self {
            value,
            token_metadata,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value
    }

    pub fn get_token_metadata(&self) -> TokenMetadata {
        self.token_metadata.clone()
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata.clone());
    }
}

#[derive(Debug, Clone)]
pub struct AstIdentifier {
    lexeme: String,
    token_metadata: TokenMetadata,
}

impl AstIdentifier {
    pub fn new(lexeme: String, token_metadata: TokenMetadata) -> Self {
        Self {
            lexeme,
            token_metadata,
        }
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn get_token_metadata(&self) -> TokenMetadata {
        self.token_metadata.clone()
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata.clone());
    }
}

#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}

impl Ast {
    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            stmts: Vec::new(),
        }
    }
}
#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
    VariableDefinition(VariableDefinition),
    VariableAssignment(VariableAssignment),
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub target_expr: Option<Expr>,
    pub field: AstIdentifier,
    pub value: Expr,
}

impl VariableAssignment {
    pub fn new(target_expr: Option<Expr>, field: AstIdentifier, value: Expr) -> Self {
        Self {
            target_expr,
            field,
            value,
        }
    }
}

#[derive(Debug)]
pub struct VariableDefinition {
    pub name: String,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub value: Expr,
}

impl VariableDefinition {
    pub fn new(name: String, value_type: ValueType, is_mutable: bool, value: Expr) -> Self {
        Self {
            name,
            value_type,
            is_mutable,
            value,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    Literal(AstValue),
    IdentifierLookup(AstIdentifier),
}

impl Expr {
    pub fn type_check(
        &self,
        variables: &HashMap<String, (ValueType, bool)>,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        match self {
            Expr::BinaryExpr(expr) => expr.type_check(variables, token_vec),
            Expr::UnaryExpr(expr) => expr.type_check(variables, token_vec),
            Expr::Literal(ast_value) => Ok(ast_value.value.to_value_type()),
            Expr::IdentifierLookup(ast_identifier) => {
                if let Some((value_type, is_mutable)) = variables.get(&ast_identifier.lexeme) {
                    if !is_mutable {
                        token_vec.push(ast_identifier.token_metadata.clone());
                        return Err(
                            format!(
                                "Cannot assign to immutable variable: '{}'",
                                ast_identifier.lexeme
                            )
                        );
                    }
                    Ok(value_type.clone())
                } else {
                    token_vec.push(ast_identifier.token_metadata.clone());
                    return Err(format!("Undefined variable: '{}'", ast_identifier.lexeme));
                }
            }
        }
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        match self {
            Expr::BinaryExpr(expr) => expr.push_to_token_vec(token_vec),
            Expr::UnaryExpr(expr) => expr.push_to_token_vec(token_vec),
            Expr::Literal(ast_value) => ast_value.push_to_token_vec(token_vec),
            Expr::IdentifierLookup(ast_identifier) => ast_identifier.push_to_token_vec(token_vec),
        }
    }
}

// impl Expr {
//     pub fn type_check_and_get_eval(&self) -> Result<Option<Expr>, (String, TokenMetadata)> {
//         match self {
//             Expr::BinaryExpr(expr) => expr.type_check_and_get_eval(),
//             Expr::UnaryExpr(expr) => expr.type_check_and_get_eval(),
//             Expr::Literal(ast_value) => Ok(Some(Expr::Literal(ast_value.clone()))),
//         }
//     }
// }

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOp,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        self.left.push_to_token_vec(token_vec);
        self.right.push_to_token_vec(token_vec);
    }

    pub fn type_check(
        &self,
        variables: &HashMap<String, (ValueType, bool)>,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        let left_type = self.left.type_check(variables, token_vec)?;
        let right_type = self.right.type_check(variables, token_vec)?;

        match left_type.type_check_binary(&right_type, self.operator.clone()) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.left.push_to_token_vec(token_vec);
                self.right.push_to_token_vec(token_vec);
                Err(e)
            }
        }
    }
}

// impl BinaryExpr {
//     pub fn type_check_and_get_eval(&self) -> Result<Option<Expr>, (String, TokenMetadata)> {
//         match (&*self.left, &*self.right) {
//             (Expr::Literal(left), Expr::Literal(right)) => {
//                 match &self.operator {
//                     ExprOp::Add => {
//                         match left.get_value().add(&right.get_value()) {
//                             Ok(v) =>
//                                 Ok(
//                                     Some(Expr::Literal(AstValue::new(v, left.get_token_metadata())))
//                                 ),
//                             Err(e) => Err((e, left.get_token_metadata())),
//                         }
//                     }
//                     ExprOp::Sub => {
//                         match left.get_value().sub(&right.get_value()) {
//                             Ok(v) =>
//                                 Ok(
//                                     Some(Expr::Literal(AstValue::new(v, left.get_token_metadata())))
//                                 ),
//                             Err(e) => Err((e, left.get_token_metadata())),
//                         }
//                     }
//                     ExprOp::Mul => {
//                         match left.get_value().mul(&right.get_value()) {
//                             Ok(v) =>
//                                 Ok(
//                                     Some(Expr::Literal(AstValue::new(v, left.get_token_metadata())))
//                                 ),
//                             Err(e) => Err((e, left.get_token_metadata())),
//                         }
//                     }
//                     ExprOp::Div => {
//                         match left.get_value().div(&right.get_value()) {
//                             Ok(v) =>
//                                 Ok(
//                                     Some(Expr::Literal(AstValue::new(v, left.get_token_metadata())))
//                                 ),
//                             Err(e) => Err((e, left.get_token_metadata())),
//                         }
//                     }
//                     op =>
//                         Err((
//                             format!("Invalid binary operator: {}", op.to_op_string()),
//                             left.get_token_metadata(),
//                         )),
//                 }
//             }
//             _ => Ok(None),
//         }
//     }
// }

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        self.right.push_to_token_vec(token_vec);
    }

    pub fn type_check(
        &self,
        variables: &HashMap<String, (ValueType, bool)>,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        let right_type = self.right.type_check(variables, token_vec)?;

        match right_type.type_check_unary(self.operator.clone()) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.right.push_to_token_vec(token_vec);
                if token_vec.len() > 0 {
                    let mutable_right = token_vec.get_mut(0).unwrap();
                    for _ in 0..self.operator.get_op_len() {
                        mutable_right.decrement_start();
                        mutable_right.increment_length();
                    }
                }
                Err(e)
            }
        }
    }
}

// impl UnaryExpr {
//     pub fn type_check_and_get_eval(&self) -> Result<Option<Expr>, (String, TokenMetadata)> {
//         match &*self.right {
//             Expr::Literal(value) => {
//                 match &self.operator {
//                     ExprOp::Neg => {
//                         match value.get_value().neg() {
//                             Ok(v) =>
//                                 Ok(
//                                     Some(
//                                         Expr::Literal(AstValue::new(v, value.get_token_metadata()))
//                                     )
//                                 ),
//                             Err(e) => Err((e, value.get_token_metadata())),
//                         }
//                     }
//                     op =>
//                         Err((
//                             format!("Invalid unary operator: {}", op.to_op_string()),
//                             value.get_token_metadata(),
//                         )),
//                 }
//             }
//             _ => Ok(None),
//         }
//     }
// }
