use crate::{ parser::token::Token, value::{ Value, ValueType } };

#[derive(Debug, Clone)]
pub struct AstValue {
    value: Value,
    token: Token,
}

impl AstValue {
    pub fn new(value: Value, token: Token) -> Self {
        Self {
            value,
            token,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value
    }

    pub fn get_token(&self) -> Token {
        self.token.clone()
    }
}

#[derive(Debug, Clone)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Truthy,
}

impl ExprOp {
    pub fn to_op_string(&self) -> String {
        match self {
            ExprOp::Add => "addition".to_string(),
            ExprOp::Sub => "subtraction".to_string(),
            ExprOp::Mul => "multiplication".to_string(),
            ExprOp::Div => "division".to_string(),
            ExprOp::Neg => "negation".to_string(),
            ExprOp::Truthy => "truthy".to_string(),
        }
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
    VarAssignment(VarAssignment),
}
#[derive(Debug)]
pub struct VarAssignment {
    name: String,
    value_type: ValueType,
    mutable: bool,
    value: Expr,
}
#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    Literal(AstValue),
}

impl Expr {
    pub fn type_check_and_get_eval(&self) -> Result<Option<Expr>, (String, Token)> {
        match self {
            Expr::BinaryExpr(expr) => expr.type_check_and_get_eval(),
            Expr::UnaryExpr(expr) => expr.type_check_and_get_eval(),
            Expr::Literal(ast_value) => Ok(Some(Expr::Literal(ast_value.clone()))),
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: ExprOp,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn type_check_and_get_eval(&self) -> Result<Option<Expr>, (String, Token)> {
        match (&*self.left, &*self.right) {
            (Expr::Literal(left), Expr::Literal(right)) => {
                match &self.operator {
                    ExprOp::Add => {
                        match left.get_value().add(&right.get_value()) {
                            Ok(v) => Ok(Some(Expr::Literal(AstValue::new(v, left.get_token())))),
                            Err(e) => Err((e, left.get_token())),
                        }
                    }
                    ExprOp::Sub => {
                        match left.get_value().sub(&right.get_value()) {
                            Ok(v) => Ok(Some(Expr::Literal(AstValue::new(v, left.get_token())))),
                            Err(e) => Err((e, left.get_token())),
                        }
                    }
                    ExprOp::Mul => {
                        match left.get_value().mul(&right.get_value()) {
                            Ok(v) => Ok(Some(Expr::Literal(AstValue::new(v, left.get_token())))),
                            Err(e) => Err((e, left.get_token())),
                        }
                    }
                    ExprOp::Div => {
                        match left.get_value().div(&right.get_value()) {
                            Ok(v) => Ok(Some(Expr::Literal(AstValue::new(v, left.get_token())))),
                            Err(e) => Err((e, left.get_token())),
                        }
                    }
                    op =>
                        Err((
                            format!("Invalid binary operator: {}", op.to_op_string()),
                            left.get_token(),
                        )),
                }
            }
            _ => Ok(None),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: ExprOp,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn type_check_and_get_eval(&self) -> Result<Option<Expr>, (String, Token)> {
        match &*self.right {
            Expr::Literal(value) => {
                match &self.operator {
                    ExprOp::Neg => {
                        match value.get_value().neg() {
                            Ok(v) => Ok(Some(Expr::Literal(AstValue::new(v, value.get_token())))),
                            Err(e) => Err((e, value.get_token())),
                        }
                    }
                    op =>
                        Err((
                            format!("Invalid unary operator: {}", op.to_op_string()),
                            value.get_token(),
                        )),
                }
            }
            _ => Ok(None),
        }
    }
}
