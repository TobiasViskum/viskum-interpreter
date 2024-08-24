use std::{ fmt::Debug, rc::Rc };

mod binary_expr;
mod unary_expr;
mod literal_expr;
mod identifier_expr;
mod fn_call_expr;
// mod native_call_expr;
mod grouping_expr;

use ahash::AHashMap;
pub use binary_expr::BinaryExpr;
use grouping_expr::GroupingExpr;
pub use unary_expr::UnaryExpr;
pub use literal_expr::LiteralExpr;
pub use identifier_expr::IdentifierExpr;
pub use fn_call_expr::FnCallExpr;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::{ ops::{ BinaryOp, UnaryOp }, ValueType } },
    error_handler::{ CompileError, ReportedError, SrcCharsRange },
    ir::icfg::{ dag::DAG, icfg_builder::ICFGBuilder },
    parser::token::TokenMetadata,
    traits::{ AstDissasemble, Dissasemble, ExprTrait },
    ProgramSymbolTablePhase1,
};

use super::AstArena;

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

        self.exprs.push(Expr::GroupingExpr(GroupingExpr::new(self.ast_arena.alloc_expr(expr))));

        Ok(())
    }

    pub fn emit_fn_call(&mut self, fn_call_expr: FnCallExpr<'ast>) {
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

        let unary_expr = UnaryExpr::new(op, self.ast_arena.alloc_expr(rhs));

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

        let (lhs, rhs) = match (popped_left, popped_right) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            (Some(lhs), None) => {
                let src_chars_range = {
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
                let src_chars_range = {
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
            self.ast_arena.alloc_expr(lhs),
            op,
            self.ast_arena.alloc_expr(rhs)
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

    pub fn compile_to_dag(&self) -> DAG {
        todo!()
    }
}

impl<'ast> ExprTrait for Expr<'ast> {
    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError> {
        match self {
            Self::GroupingExpr(expr) => expr.type_check(program_symbol_table),
            Self::BinaryExpr(expr) => expr.type_check(program_symbol_table),
            Self::UnaryExpr(expr) => expr.type_check(program_symbol_table),
            Self::LiteralExpr(expr) => expr.type_check(program_symbol_table),
            Self::IdentifierExpr(expr) => expr.type_check(program_symbol_table),
            Self::FnCallExpr(expr) => expr.type_check(program_symbol_table),
        }
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        match self {
            Self::GroupingExpr(expr) => expr.compile_into_dag(dag, ident_node_id_map, icfg_builder),
            Self::BinaryExpr(expr) => expr.compile_into_dag(dag, ident_node_id_map, icfg_builder),
            Self::UnaryExpr(expr) => expr.compile_into_dag(dag, ident_node_id_map, icfg_builder),
            Self::LiteralExpr(expr) => expr.compile_into_dag(dag, ident_node_id_map, icfg_builder),
            Self::IdentifierExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder),
            Self::FnCallExpr(expr) => expr.compile_into_dag(dag, ident_node_id_map, icfg_builder),
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
        }
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
