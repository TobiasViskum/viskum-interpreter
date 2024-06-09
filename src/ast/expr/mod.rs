use crate::{
    compiler::cfg::dag::{ DAGNode, DAGOp, DAG },
    error_handler::{ CompileError, InternalError, SrcCharsRange },
    operations::{ BinaryOp, UnaryOp },
    parser::token::TokenMetadata,
    print_todo,
    value::{ Value, ValueType },
    vm::instructions::NativeCall,
    Dissasemble,
};

use super::ast_symbol_table::{ AstSymbol, AstSymbolTable };

mod binary_expr;
mod unary_expr;
mod literal_expr;
mod identifier_expr;
mod fn_call_expr;
mod native_call_expr;

pub use binary_expr::BinaryExpr;
pub use unary_expr::UnaryExpr;
pub use literal_expr::LiteralExpr;
pub use identifier_expr::IdentifierExpr;
pub use fn_call_expr::FnCallExpr;
pub use native_call_expr::NativeCallExpr;

#[derive(Debug)]
pub struct ExprResultOk {
    value_type: ValueType,
    can_constant_fold: bool,
}

impl ExprResultOk {
    pub fn new(value_type: ValueType, can_constant_fold: bool) -> Self {
        Self {
            value_type,
            can_constant_fold,
        }
    }

    pub fn take_value_type(self) -> ValueType {
        self.value_type
    }

    pub fn get_can_constant_fold(&self) -> bool {
        self.can_constant_fold
    }
}

#[derive(Debug)]
pub struct ExprResultErr {
    compile_errors: Vec<CompileError>,
    internal_errors: Vec<InternalError>,
}

impl ExprResultErr {
    pub fn new(compile_errors: Vec<CompileError>) -> Self {
        Self { compile_errors, internal_errors: vec![] }
    }

    pub fn push_internal_error(&mut self, internal_error: InternalError) {
        self.internal_errors.push(internal_error);
    }

    pub fn take_errors(self) -> Vec<CompileError> {
        self.compile_errors
    }
}

pub type ExprResult = Result<ExprResultOk, ExprResultErr>;

#[derive(Debug)]
pub enum ExprEvaluateErr {
    CompileError(CompileError),
    InternalError(InternalError),
}

impl From<ExprEvaluateErr> for ExprResultErr {
    fn from(value: ExprEvaluateErr) -> Self {
        match value {
            ExprEvaluateErr::CompileError(compile_error) => ExprResultErr::new(vec![compile_error]),
            ExprEvaluateErr::InternalError(internal_error) => {
                let mut expr_result_error = ExprResultErr::new(vec![]);
                expr_result_error.push_internal_error(internal_error);
                expr_result_error
            }
        }
    }
}

pub type ExprEvaluateResult = Result<(Value, SrcCharsRange), ExprEvaluateErr>;

pub trait ExprTrait {
    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult;

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize;

    fn collect_metadata(&self) -> SrcCharsRange;

    fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult;
}

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

    pub fn emit_native_call(&mut self, native_call_expr: NativeCallExpr) {
        self.exprs.push(Expr::NativeCallExpr(native_call_expr))
    }

    pub fn emit_fn_call(&mut self, fn_call_expr: FnCallExpr) {
        self.exprs.push(Expr::FnCallExpr(fn_call_expr))
    }

    pub fn emit_ident_lookup(&mut self, identifier_expr: IdentifierExpr) {
        self.exprs.push(Expr::IdentifierExpr(identifier_expr))
    }

    pub fn emit_constant_literal(&mut self, literal_expr: LiteralExpr) {
        self.exprs.push(Expr::LiteralExpr(literal_expr))
    }

    pub fn emit_unary_op(
        &mut self,
        op: UnaryOp,
        metadata: TokenMetadata
    ) -> Result<(), CompileError> {
        let rhs = match self.exprs.pop() {
            Some(expr) => expr,
            None => {
                Err(CompileError::new("Expected right hand side".to_string(), metadata.into()))?
            }
        };

        let unary_expr = UnaryExpr::new(op, rhs);

        self.exprs.push(Expr::UnaryExpr(unary_expr));

        Ok(())
    }

    pub fn emit_binary_op(
        &mut self,
        op: BinaryOp,
        metadata: TokenMetadata
    ) -> Result<(), CompileError> {
        let popped_right = self.exprs.pop();
        let popped_left = self.exprs.pop();

        print_todo("Not reporting errors in emit_binary_op");

        let (lhs, rhs) = match (popped_left, popped_right) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            (Some(lhs), None) => panic!("Missing right hand side of binary operation"),
            (lhs, rhs) =>
                panic!("Error in emit_binary_op: ExprBuilder: lhs: {:?}, rhs: {:?}", lhs, rhs),
        };

        let binary_expr = BinaryExpr::new(lhs, op, rhs);

        self.exprs.push(Expr::BinaryExpr(binary_expr));

        Ok(())
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    LiteralExpr(LiteralExpr),
    IdentifierExpr(IdentifierExpr),
    FnCallExpr(FnCallExpr),
    NativeCallExpr(NativeCallExpr),
}

impl Dissasemble for Expr {
    fn dissasemble(&self) -> String {
        match self {
            Self::BinaryExpr(expr) => expr.dissasemble(),
            Self::UnaryExpr(expr) => expr.dissasemble(),
            Self::LiteralExpr(expr) => expr.dissasemble(),
            Self::IdentifierExpr(expr) => expr.dissasemble(),
            Self::FnCallExpr(expr) => expr.dissasemble(),
            Self::NativeCallExpr(expr) => expr.dissasemble(),
        }
    }
}

impl ExprTrait for Expr {
    fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
        match self {
            Self::BinaryExpr(expr) => expr.evaluate(ast_symbol_table),
            Self::UnaryExpr(expr) => expr.evaluate(ast_symbol_table),
            Self::LiteralExpr(expr) => expr.evaluate(ast_symbol_table),
            Self::IdentifierExpr(expr) => expr.evaluate(ast_symbol_table),
            Self::FnCallExpr(expr) => expr.evaluate(ast_symbol_table),
            Self::NativeCallExpr(expr) => expr.evaluate(ast_symbol_table),
        }
    }

    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
        match self {
            Self::BinaryExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
            Self::UnaryExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
            Self::LiteralExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
            Self::IdentifierExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
            Self::FnCallExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
            Self::NativeCallExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
        }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        match self {
            Expr::BinaryExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::UnaryExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::LiteralExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::IdentifierExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::FnCallExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::NativeCallExpr(expr) => expr.compile_to_dag_node(dag),
        }
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        match self {
            Self::BinaryExpr(expr) => expr.collect_metadata(),
            Self::UnaryExpr(expr) => expr.collect_metadata(),
            Self::LiteralExpr(expr) => expr.collect_metadata(),
            Self::IdentifierExpr(expr) => expr.collect_metadata(),
            Self::FnCallExpr(expr) => expr.collect_metadata(),
            Self::NativeCallExpr(expr) => expr.collect_metadata(),
        }
    }
}

impl Expr {
    pub fn compile_to_dag(&self) -> DAG {
        let mut dag = DAG::new();

        let entry_node_id = self.compile_to_dag_node(&mut dag);

        dag.set_entry_node_id(entry_node_id);

        dag
    }
}
