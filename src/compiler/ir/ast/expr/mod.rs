use std::{ fmt::Debug, rc::Rc };

mod binary_expr;
mod unary_expr;
mod literal_expr;
mod identifier_expr;
mod fn_call_expr;
// mod native_call_expr;
mod grouping_expr;

pub use binary_expr::BinaryExpr;
use grouping_expr::GroupingExpr;
pub use unary_expr::UnaryExpr;
pub use literal_expr::LiteralExpr;
pub use identifier_expr::IdentifierExpr;
pub use fn_call_expr::FnCallExpr;

use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::{ ops::{ BinaryOp, UnaryOp }, ValueType } },
    error_handler::{ CompileError, InternalError, ReportedError, SrcCharsRange },
    ir::icfg::dag::DAG,
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::AstArena;
// pub use native_call_expr::NativeCallExpr;

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

// #[derive(Debug)]
// pub enum TypeCheckErr {
//     CompileError(CompileError),
//     InternalError(InternalError),
// }

// impl TypeCheckErr {
//     pub fn new(compile_error: CompileError) -> Self {
//         Self::CompileError(compile_error)
//     }
// }

// impl From<TypeCheckErr> for ExprResultErr {
//     fn from(value: TypeCheckErr) -> Self {
//         match value {
//             TypeCheckErr::CompileError(compile_error) => ExprResultErr::new(vec![compile_error]),
//             TypeCheckErr::InternalError(internal_error) => {
//                 let mut expr_result_error = ExprResultErr::new(vec![]);
//                 expr_result_error.push_internal_error(internal_error);
//                 expr_result_error
//             }
//         }
//     }
// }

// pub type ExprEvaluateResult = Result<(Value, SrcCharsRange), TypeCheckErr>;

pub trait ExprTrait where Self: Dissasemble + Debug {
    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError>;

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize;

    fn collect_metadata(&self) -> SrcCharsRange;
}

#[derive(Debug)]
pub struct ExprBuilder<'ast> {
    exprs: Vec<Expr<'ast>>,
    ast_arena: &'ast AstArena<'ast>,
}

impl<'ast> ExprBuilder<'ast> {
    pub fn new(ast_arena: &'ast AstArena<'ast>) -> Self {
        Self {
            exprs: Vec::new(),
            ast_arena,
        }
    }

    pub fn get_built_expr(mut self) -> Expr<'ast> {
        self.exprs.pop().unwrap()
    }

    // pub fn emit_native_call(&mut self, native_call_expr: NativeCallExpr<'ast>) {
    //     self.exprs.push(Expr::NativeCallExpr(native_call_expr))
    // }

    pub fn emit_grouping(&mut self, metadata: TokenMetadata) -> Result<(), CompileError> {
        let expr = match self.exprs.pop() {
            Some(expr) => expr,
            None => {
                let mut src_chars_range: SrcCharsRange = metadata.into();
                src_chars_range.inc_char_by(1);
                Err(
                    CompileError::new(
                        ReportedError::new(
                            "Expected expression between '()'".to_string(),
                            src_chars_range
                        )
                    )
                )?
            }
        };

        self.exprs.push(Expr::GroupingExpr(GroupingExpr::new(self.ast_arena.alloc_mut_expr(expr))));

        Ok(())
    }

    pub fn emit_fn_call(&mut self, fn_call_expr: FnCallExpr<'ast>, arena: &'ast AstArena<'ast>) {
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
                Err(
                    CompileError::new(
                        ReportedError::new("Expected right hand side".to_string(), metadata.into())
                    )
                )?
            }
        };

        let unary_expr = UnaryExpr::new(op, self.ast_arena.alloc_mut_expr(rhs));

        self.exprs.push(Expr::UnaryExpr(unary_expr));

        Ok(())
    }

    pub fn emit_binary_op(
        &mut self,
        op: BinaryOp,
        metadata: TokenMetadata,
        arena: &'ast AstArena<'ast>
    ) -> Result<(), CompileError> {
        let popped_right = self.exprs.pop();
        let popped_left = self.exprs.pop();

        let (lhs, rhs) = match (popped_left, popped_right) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            (Some(lhs), None) => {
                let src_chars_range = unsafe {
                    let mut src_chars_range = lhs.collect_metadata();
                    src_chars_range.merge(&metadata.into());
                    src_chars_range
                };
                Err(
                    CompileError::new(
                        ReportedError::new(
                            "Expected right-hand side of binary operation".to_string(),
                            src_chars_range
                        )
                    )
                )?
            }
            (None, Some(rhs)) => {
                let src_chars_range = unsafe {
                    let mut src_chars_range = rhs.collect_metadata();
                    src_chars_range.merge(&metadata.into());
                    src_chars_range
                };
                Err(
                    CompileError::new(
                        ReportedError::new(
                            "Expected left-hand side of binary operation".to_string(),
                            src_chars_range
                        )
                    )
                )?
            }
            (None, None) => {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            format!("Unexpected token '{}'", op.dissasemble()),
                            metadata.into()
                        )
                    )
                )?
            }
        };

        let binary_expr = BinaryExpr::new(
            self.ast_arena.alloc_mut_expr(lhs),
            op,
            self.ast_arena.alloc_mut_expr(rhs)
        );

        self.exprs.push(Expr::BinaryExpr(binary_expr));

        Ok(())
    }
}

#[derive(Debug)]
pub enum Expr<'ast> {
    BinaryExpr(BinaryExpr<'ast>),
    GroupingExpr(GroupingExpr<'ast>),
    UnaryExpr(UnaryExpr<'ast>),
    LiteralExpr(LiteralExpr),
    IdentifierExpr(IdentifierExpr),
    FnCallExpr(FnCallExpr<'ast>),
    // NativeCallExpr(NativeCallExpr<'ast>),
}

impl<'ast> Expr<'ast> {
    pub fn is_ident_expr(&self) -> bool {
        match self {
            Self::IdentifierExpr(_) => true,
            _ => false,
        }
    }

    pub fn unwrap_ident_expr(&self) -> (Rc<str>, TokenMetadata) {
        match self {
            Self::IdentifierExpr(ident_expr) =>
                (ident_expr.get_lexeme(), ident_expr.get_raw_metadata()),
            _ => panic!("Expected identifier expr in unwrap_ident_expr"),
        }
    }

    pub fn compile_to_dag(&self) -> DAG {
        todo!()
    }
}

impl<'ast> Dissasemble for Expr<'ast> {
    fn dissasemble(&self) -> String {
        match self {
            Self::GroupingExpr(expr) => expr.dissasemble(),
            Self::BinaryExpr(expr) => expr.dissasemble(),
            Self::UnaryExpr(expr) => expr.dissasemble(),
            Self::LiteralExpr(expr) => expr.dissasemble(),
            Self::IdentifierExpr(expr) => expr.dissasemble(),
            Self::FnCallExpr(expr) => expr.dissasemble(),
            // Self::NativeCallExpr(expr) => expr.dissasemble(),
        }
    }
}

impl<'ast> ExprTrait for Expr<'ast> {
    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprEvaluateResult {
    //     match self {
    //         Self::GroupingExpr(expr) => expr.evaluate(ast_symbol_table),
    //         Self::BinaryExpr(expr) => expr.evaluate(ast_symbol_table),
    //         Self::UnaryExpr(expr) => expr.evaluate(ast_symbol_table),
    //         Self::LiteralExpr(expr) => expr.evaluate(ast_symbol_table),
    //         Self::IdentifierExpr(expr) => expr.evaluate(ast_symbol_table),
    //         Self::FnCallExpr(expr) => expr.evaluate(ast_symbol_table),
    //         Self::NativeCallExpr(expr) => expr.evaluate(ast_symbol_table),
    //     }
    // }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     match self {
    //         Self::GroupingExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //         Self::BinaryExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //         Self::UnaryExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //         Self::LiteralExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //         Self::IdentifierExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //         Self::FnCallExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //         Self::NativeCallExpr(expr) => expr.type_check_and_constant_fold(ast_symbol_table),
    //     }
    // }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        match self {
            Self::GroupingExpr(expr) => expr.type_check(symbol_table_ref),
            Self::BinaryExpr(expr) => expr.type_check(symbol_table_ref),
            Self::UnaryExpr(expr) => expr.type_check(symbol_table_ref),
            Self::LiteralExpr(expr) => expr.type_check(symbol_table_ref),
            Self::IdentifierExpr(expr) => expr.type_check(symbol_table_ref),
            Self::FnCallExpr(expr) => expr.type_check(symbol_table_ref),
            // Self::NativeCallExpr(expr) => expr.type_check(symbol_table_ref),
        }
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        match self {
            Self::GroupingExpr(expr) => expr.compile_to_dag_node(dag),
            Self::BinaryExpr(expr) => expr.compile_to_dag_node(dag),
            Self::UnaryExpr(expr) => expr.compile_to_dag_node(dag),
            Self::LiteralExpr(expr) => expr.compile_to_dag_node(dag),
            Self::IdentifierExpr(expr) => expr.compile_to_dag_node(dag),
            Self::FnCallExpr(expr) => expr.compile_to_dag_node(dag),
            // Self::NativeCallExpr(expr) => expr.compile_to_dag_node(dag),
        }
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        match self {
            Self::GroupingExpr(expr) => expr.collect_metadata(),
            Self::BinaryExpr(expr) => expr.collect_metadata(),
            Self::UnaryExpr(expr) => expr.collect_metadata(),
            Self::LiteralExpr(expr) => expr.collect_metadata(),
            Self::IdentifierExpr(expr) => expr.collect_metadata(),
            Self::FnCallExpr(expr) => expr.collect_metadata(),
            // Self::NativeCallExpr(expr) => expr.collect_metadata(),
        }
    }
}
