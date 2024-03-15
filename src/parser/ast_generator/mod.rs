use std::collections::HashMap;

use crate::{
    ast::{
        Ast,
        AstIdentifier,
        AstValue,
        BinaryExpr,
        Expr,
        Stmt,
        UnaryExpr,
        VariableAssignment,
        VariableDefinition,
    },
    operations::{ BinaryOp, UnaryOp },
    value::ValueType,
};
use super::token::TokenMetadata;

pub struct AstGenerator {
    ast: Option<Ast>,
    statements: Vec<Stmt>,
    expressions: Vec<Expr>,
    panic_mode: bool,
    variables: HashMap<String, (ValueType, bool)>,
}

impl AstGenerator {
    pub fn new() -> Self {
        Self {
            ast: Some(Ast::new()),
            statements: Vec::new(),
            expressions: Vec::new(),
            panic_mode: false,
            variables: HashMap::new(),
        }
    }

    pub fn free(&mut self) {
        self.ast = None;
    }

    pub fn enter_panic_mode(&mut self) {
        self.panic_mode = true;
    }

    pub fn exit_panic_mode(&mut self) {
        self.panic_mode = false;
    }

    pub fn emit_constant_literal(&mut self, value: AstValue) {
        self.expressions.push(Expr::Literal(value));
    }

    pub fn emit_identifier_lookup(&mut self, variable: AstIdentifier) {
        self.expressions.push(Expr::IdentifierLookup(variable));
    }

    pub fn emit_variable_assignment(
        &mut self,
        token_metadata: TokenMetadata
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        let value = self.expressions.pop().unwrap();
        let identifier = match self.expressions.pop().unwrap() {
            Expr::IdentifierLookup(v) => v,
            _ => {
                return Err(("Expected identifier in assignment".to_string(), vec![token_metadata]));
            }
        };
        // let target_expr = if self.expressions.len() > 0 {
        //     // This can only be expr::function_call(s)
        //     Some(self.expressions.pop().unwrap())
        // } else {
        //     None
        // };

        let mut token_vec: Vec<TokenMetadata> = Vec::new();
        let type_checked_value_type = match value.type_check(&self.variables, &mut token_vec) {
            Ok(vt) => vt,
            Err(e) => {
                return Err((e, token_vec));
            }
        };

        if type_checked_value_type != self.variables.get(&identifier.get_lexeme()).unwrap().0 {
            value.push_to_token_vec(&mut token_vec);
            token_vec.push(token_metadata);
            return Err((
                format!(
                    "Variable '{}' is of type {}, but was assigned to type {}",
                    &identifier.get_lexeme(),
                    self.variables.get(&identifier.get_lexeme()).unwrap().0.to_type_string(),
                    type_checked_value_type.to_type_string()
                ),
                token_vec,
            ));
        }

        let variable_assignment = Stmt::VariableAssignment(
            VariableAssignment::new(None, identifier, value)
        );

        println!("variable_assignment: {:#?}", variable_assignment);

        self.statements.push(variable_assignment);

        Ok(())
    }

    pub fn emit_variable_definition(
        &mut self,
        lexeme: String,
        value_type: Option<ValueType>,
        is_mutable: bool
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        let value = self.expressions.pop().unwrap();

        let mut token_vec: Vec<TokenMetadata> = Vec::new();
        let type_checked_value_type = match value.type_check(&self.variables, &mut token_vec) {
            Ok(vt) => vt,
            Err(e) => {
                return Err((e, token_vec));
            }
        };

        if value_type.is_some() && value_type.clone().unwrap() != type_checked_value_type {
            let value_type = value_type.unwrap();
            value.push_to_token_vec(&mut token_vec);
            return Err((
                format!(
                    "Type mismatch: expected {}, found {:?}",
                    value_type.to_type_string(),
                    type_checked_value_type
                ),
                token_vec,
            ));
        }

        self.variables.insert(lexeme.clone(), (type_checked_value_type.clone(), is_mutable));

        let variable_definition = Stmt::VariableDefinition(
            VariableDefinition::new(lexeme, type_checked_value_type, is_mutable, value)
        );

        self.statements.push(variable_definition);

        Ok(())
    }

    pub fn emit_binary_op(
        &mut self,
        expr_op: BinaryOp
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        let popped_left = self.expressions.pop();
        let popped_right = self.expressions.pop();

        let (left, right) = match (popped_left, popped_right) {
            (Some(left), Some(right)) => (left, right),
            (Some(left), None) => {
                let mut metadata = match left {
                    Expr::Literal(v) => v.get_token_metadata(),
                    _ => panic!("This is weird..."),
                };
                metadata.increment_length();
                metadata.increment_length();

                return Err((
                    "Expected right-hand side of binary operation".to_string(),
                    vec![metadata],
                ));
            }
            (x, y) => {
                println!("{:?} {:?}", x, y);
                panic!("This is also weird...")
            }
        };

        let binary_expr = BinaryExpr {
            left: Box::new(left),
            operator: expr_op.clone(),
            right: Box::new(right),
        };

        // let expr = match binary_expr.type_check_and_get_eval() {
        //     Ok(Some(expr)) => expr,
        //     Ok(None) => Expr::BinaryExpr(binary_expr),
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        let expr = Expr::BinaryExpr(binary_expr);

        self.expressions.push(expr);

        Ok(())
    }

    pub fn emit_unary_op(&mut self, expr_op: UnaryOp) -> Result<(), (String, Vec<TokenMetadata>)> {
        if self.panic_mode {
            return Ok(());
        }

        let right = self.expressions.pop().unwrap();

        let unary_expr = UnaryExpr {
            operator: expr_op,
            right: Box::new(right),
        };

        // let expr = match unary_expr.type_check_and_get_eval() {
        //     Ok(Some(expr)) => expr,
        //     Ok(None) => Expr::UnaryExpr(unary_expr),
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        let expr = Expr::UnaryExpr(unary_expr);

        self.expressions.push(expr);

        Ok(())
    }

    pub fn push_expr(&mut self) -> Result<(), (String, Vec<TokenMetadata>)> {
        if self.panic_mode {
            self.exit_panic_mode();
            Ok(())
        } else {
            match self.expressions.pop() {
                Some(expr) => {
                    let mut token_vec: Vec<TokenMetadata> = Vec::new();
                    match expr.type_check(&self.variables, &mut token_vec) {
                        Ok(_) => {
                            self.ast.as_mut().unwrap().push_stmt(Stmt::ExprStmt(expr));
                            Ok(())
                        }
                        Err(e) => { Err((e, token_vec)) }
                    }
                }
                None => Ok(()),
            }
        }
    }

    pub fn push_stmt(&mut self) {
        if self.panic_mode {
            self.exit_panic_mode();
        } else {
            match self.statements.pop() {
                Some(stmt) => self.ast.as_mut().unwrap().push_stmt(stmt),
                None => {}
            }
        }
    }

    pub fn pop_expr(&mut self) -> Option<Expr> {
        self.expressions.pop()
    }

    pub fn get_ast(&mut self) -> Ast {
        self.ast.take().unwrap()
    }
}

// pub fn evaluate_constant_expression(expr: Expr) -> Result<Expr, (String, TokenMetadata)> {
//     match expr {
//         Expr::BinaryExpr(binary_expr) => {
//             let evaluated_left = evaluate_constant_expression(*binary_expr.left);
//             let evaluated_right = evaluate_constant_expression(*binary_expr.right);

//             match (evaluated_left, evaluated_right) {
//                 (Ok(left), Ok(right)) => {
//                     match (left, right) {
//                         (Expr::Literal(lhs), Expr::Literal(rhs)) => {
//                             match binary_expr.operator {
//                                 BinaryOp::Add => {
//                                     match lhs.get_value().add(&rhs.get_value()) {
//                                         Ok(v) =>
//                                             Ok(
//                                                 Expr::Literal(
//                                                     AstValue::new(v, lhs.get_token_metadata())
//                                                 )
//                                             ),
//                                         Err(e) => Err((e, lhs.get_token_metadata())),
//                                     }
//                                 }
//                                 BinaryOp::Sub => {
//                                     match lhs.get_value().sub(&rhs.get_value()) {
//                                         Ok(v) =>
//                                             Ok(
//                                                 Expr::Literal(
//                                                     AstValue::new(v, lhs.get_token_metadata())
//                                                 )
//                                             ),
//                                         Err(e) => Err((e, lhs.get_token_metadata())),
//                                     }
//                                 }
//                                 BinaryOp::Div => {
//                                     match lhs.get_value().div(&rhs.get_value()) {
//                                         Ok(v) =>
//                                             Ok(
//                                                 Expr::Literal(
//                                                     AstValue::new(v, lhs.get_token_metadata())
//                                                 )
//                                             ),
//                                         Err(e) => Err((e, lhs.get_token_metadata())),
//                                     }
//                                 }
//                                 BinaryOp::Mul => {
//                                     match lhs.get_value().mul(&rhs.get_value()) {
//                                         Ok(v) =>
//                                             Ok(
//                                                 Expr::Literal(
//                                                     AstValue::new(v, lhs.get_token_metadata())
//                                                 )
//                                             ),
//                                         Err(e) => Err((e, lhs.get_token_metadata())),
//                                     }
//                                 }
//                             }
//                         }
//                         _ => panic!("Expected literals"),
//                     }
//                 }
//                 (Err(e), _) => Err(e),
//                 (_, Err(e)) => Err(e),
//             }
//         }
//         Expr::UnaryExpr(unary_expr) => {
//             let evaluated_right = evaluate_constant_expression(*unary_expr.right);

//             match evaluated_right {
//                 Ok(right) => {
//                     let unary_expr = UnaryExpr {
//                         operator: unary_expr.operator.clone(),
//                         right: Box::new(right),
//                     };

//                     let expr = match unary_expr.type_check_and_get_eval() {
//                         Ok(Some(expr)) => expr,
//                         Ok(None) => Expr::UnaryExpr(unary_expr),
//                         Err(e) => {
//                             return Err(e);
//                         }
//                     };

//                     Ok(expr)
//                 }
//                 Err(e) => Err(e),
//             }
//         }
//         Expr::Literal(value) => Ok(Expr::Literal(value.clone())),
//     }
// }
