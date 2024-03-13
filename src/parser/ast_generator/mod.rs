use crate::ast::{ Ast, AstValue, BinaryExpr, Expr, ExprOp, Stmt, UnaryExpr };
use super::token::Token;

pub struct AstGenerator {
    ast: Option<Ast>,
    constants: Vec<Expr>,
    panic_mode: bool,
}

impl AstGenerator {
    pub fn new() -> Self {
        Self {
            ast: Some(Ast::new()),
            constants: Vec::new(),
            panic_mode: false,
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

    pub fn emit_constant(&mut self, value: AstValue) {
        if self.panic_mode {
            return;
        }

        self.constants.push(Expr::Literal(value));
    }

    pub fn emit_binary_op(&mut self, expr_op: ExprOp) -> Result<(), (String, Token)> {
        if self.panic_mode {
            return Ok(());
        }

        let left = self.constants.pop().unwrap();
        let right = self.constants.pop().unwrap();

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

        self.constants.push(expr);

        Ok(())
    }

    pub fn emit_unary_op(&mut self, expr_op: ExprOp) -> Result<(), (String, Token)> {
        if self.panic_mode {
            return Ok(());
        }

        let right = self.constants.pop().unwrap();

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

        self.constants.push(expr);

        Ok(())
    }

    pub fn push_expr_stmt(&mut self) {
        match self.constants.pop() {
            Some(expr) => self.ast.as_mut().unwrap().push_stmt(Stmt::ExprStmt(expr)),
            None => {}
        }
        self.exit_panic_mode();
    }

    pub fn get_ast(&mut self) -> Ast {
        self.ast.take().unwrap()
    }
}

pub fn evaluate_constant_expression(expr: Expr) -> Result<Expr, (String, Token)> {
    match expr {
        Expr::BinaryExpr(binary_expr) => {
            let evaluated_left = evaluate_constant_expression(*binary_expr.left);
            let evaluated_right = evaluate_constant_expression(*binary_expr.right);

            match (evaluated_left, evaluated_right) {
                (Ok(left), Ok(right)) => {
                    match (left, right) {
                        (Expr::Literal(lhs), Expr::Literal(rhs)) => {
                            match binary_expr.operator {
                                ExprOp::Add => {
                                    match lhs.get_value().add(&rhs.get_value()) {
                                        Ok(v) =>
                                            Ok(Expr::Literal(AstValue::new(v, lhs.get_token()))),
                                        Err(e) => Err((e, lhs.get_token())),
                                    }
                                }
                                ExprOp::Sub => {
                                    match lhs.get_value().sub(&rhs.get_value()) {
                                        Ok(v) =>
                                            Ok(Expr::Literal(AstValue::new(v, lhs.get_token()))),
                                        Err(e) => Err((e, lhs.get_token())),
                                    }
                                }
                                ExprOp::Div => {
                                    match lhs.get_value().div(&rhs.get_value()) {
                                        Ok(v) =>
                                            Ok(Expr::Literal(AstValue::new(v, lhs.get_token()))),
                                        Err(e) => Err((e, lhs.get_token())),
                                    }
                                }
                                ExprOp::Mul => {
                                    match lhs.get_value().mul(&rhs.get_value()) {
                                        Ok(v) =>
                                            Ok(Expr::Literal(AstValue::new(v, lhs.get_token()))),
                                        Err(e) => Err((e, lhs.get_token())),
                                    }
                                }
                                _ => {
                                    todo!();
                                }
                            }
                        }
                        _ => panic!("Expected literals"),
                    }
                }
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        }
        Expr::UnaryExpr(unary_expr) => {
            let evaluated_right = evaluate_constant_expression(*unary_expr.right);

            match evaluated_right {
                Ok(right) => {
                    let unary_expr = UnaryExpr {
                        operator: unary_expr.operator.clone(),
                        right: Box::new(right),
                    };

                    let expr = match unary_expr.type_check_and_get_eval() {
                        Ok(Some(expr)) => expr,
                        Ok(None) => Expr::UnaryExpr(unary_expr),
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    Ok(expr)
                }
                Err(e) => Err(e),
            }
        }
        Expr::Literal(value) => Ok(Expr::Literal(value.clone())),
    }
}
