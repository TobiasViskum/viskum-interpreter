use crate::{
    compiler::cfg::dag::{ DAGNode, DAGOp, DAG },
    error_handler::CompileError,
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value::{ Value, ValueType },
};

use super::stmt::{ Stmt, VarAssignStmt };

#[derive(Debug)]
pub struct ExprBuilder {
    exprs: Vec<Expr>,
}

impl ExprBuilder {
    pub fn new() -> Self {
        Self {
            exprs: Vec::new(),
        }
    }

    pub fn get_built_expr(&mut self) -> Expr {
        self.exprs.pop().unwrap()
    }

    pub fn emit_ident_lookup(&mut self, ast_identifier: AstIdentifier) {
        self.exprs.push(Expr::IdentifierLookup(ast_identifier))
    }

    pub fn emit_constant_literal(&mut self, ast_value: AstValue) {
        self.exprs.push(Expr::Literal(ast_value))
    }

    pub fn emit_unary_op(&mut self, unary_op: UnaryOp) -> Result<(), CompileError> {
        let rhs = self.exprs.pop().expect("Expected rhs of unary operation");

        let unary_expr = UnaryExpr {
            operator: unary_op,
            right: Box::new(rhs),
        };

        self.exprs.push(Expr::UnaryExpr(unary_expr));

        Ok(())
    }

    pub fn emit_binary_op(&mut self, binary_op: BinaryOp) -> Result<(), CompileError> {
    

        let popped_left = self.exprs.pop();
        let popped_right = self.exprs.pop();

        let (lhs, rhs) = match (popped_left, popped_right) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            (Some(lsh), None) => panic!("Missing right hand side of binary operation"),
            (lhs, rhs) =>
                panic!("Error in emit_binary_op: ExprBuilder: lhs: {:?}, rhs: {:?}", lhs, rhs),
        };

        let binary_expr = BinaryExpr {
            left: Box::new(lhs),
            operator: binary_op,
            right: Box::new(rhs),
        };

        self.exprs.push(Expr::BinaryExpr(binary_expr));

        Ok(())
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
    pub fn compile_to_dag(&self) -> DAG {
        let mut dag = DAG::new();

        let entry_node_id = self.compile_to_dag_node(&mut dag);

        dag.set_entry_node_id(entry_node_id);

        dag
    }

    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        match self {
            Expr::BinaryExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::UnaryExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::Literal(ast_value) => ast_value.compile_to_dag_node(dag),
            Expr::IdentifierLookup(ast_identifier) => ast_identifier.compile_to_dag_node(dag),
        }
    }

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
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let left_dag = self.left.compile_to_dag_node(dag);
        let right_dag = self.right.compile_to_dag_node(dag);

        let op = DAGOp::BinaryOp(self.operator);

        let mut dag_node = DAGNode::new(op, Some(vec![left_dag, right_dag]));

        dag.add_node(dag_node)
    }

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
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let right_dag = self.right.compile_to_dag_node(dag);

        let op = DAGOp::UnaryOp(self.operator);

        let mut dag_node = DAGNode::new(op, Some(vec![right_dag]));

        dag.add_node(dag_node)
    }

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

#[derive(Debug, Clone)]
pub struct AstValue {
    pub value: Value,
    pub token_metadata: TokenMetadata,
}

impl AstValue {
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let op = DAGOp::Const(self.value.clone());

        let mut dag_node = DAGNode::new(op, None);

        dag.add_node(dag_node)
    }

    pub fn new(value: Value, token_metadata: TokenMetadata) -> Self {
        Self {
            value,
            token_metadata,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value.clone()
    }

    pub fn get_token_metadata(&self) -> TokenMetadata {
        self.token_metadata
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata);
    }
}

#[derive(Debug, Clone)]
pub struct AstIdentifier {
    pub lexeme: String,
    pub token_metadata: TokenMetadata,
}

impl AstIdentifier {
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let op = DAGOp::Identifier(self.lexeme.clone());

        let mut dag_node = DAGNode::new(op, None);

        dag.add_node(dag_node)
    }

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
        self.token_metadata
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata);
    }
}
