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
    // VariableAssignment(VariableAssignment),
}

#[derive(Debug)]
pub struct VariableAssignment {
    pub target_expr: Option<Expr>,
    pub field: AstIdentifier,
    pub value: Expr,
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
    VariableLookup(AstIdentifier),
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
