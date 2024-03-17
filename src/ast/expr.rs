use crate::{
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value::ValueType,
};

use super::{ AstIdentifier, AstValue };

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
        ast_environemtn: &AstEnvironment,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        match self {
            Expr::BinaryExpr(expr) => expr.type_check(ast_environemtn, token_vec),
            Expr::UnaryExpr(expr) => expr.type_check(ast_environemtn, token_vec),
            Expr::Literal(ast_value) => Ok(ast_value.value.to_value_type()),
            Expr::IdentifierLookup(ast_identifier) => {
                if let Some((value_type, _, _)) = ast_environemtn.get(&ast_identifier.lexeme) {
                    Ok(value_type)
                } else {
                    token_vec.push(ast_identifier.token_metadata);
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
        ast_environment: &AstEnvironment,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        let left_type = self.left.type_check(ast_environment, token_vec)?;
        let right_type = self.right.type_check(ast_environment, token_vec)?;

        match left_type.type_check_binary(&right_type, self.operator) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.left.push_to_token_vec(token_vec);
                self.right.push_to_token_vec(token_vec);
                Err(e)
            }
        }
    }
}

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
        ast_environment: &AstEnvironment,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        let right_type = self.right.type_check(ast_environment, token_vec)?;

        match right_type.type_check_unary(self.operator) {
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
