use std::{ fmt::Debug, rc::Rc };

mod binary_expr;
mod unary_expr;
mod literal_expr;
mod identifier_expr;
mod fn_call_expr;
// mod native_call_expr;
mod grouping_expr;
mod array_expr;
mod index_expr;

use ahash::AHashMap;
pub use array_expr::ArrayExpr;
pub use binary_expr::BinaryExpr;
pub use grouping_expr::GroupingExpr;
pub use unary_expr::UnaryExpr;
pub use literal_expr::LiteralExpr;
pub use identifier_expr::IdentifierExpr;
pub use fn_call_expr::FnCallExpr;
pub use index_expr::IndexExpr;

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

    pub fn emit_indexing(&mut self, metadata: TokenMetadata) -> Result<(), CompileError> {
        let index_expr = match self.exprs.pop() {
            Some(expr) => expr,
            None => {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            "Expected expression in index expression".to_string(),
                            metadata.into()
                        )
                    )
                )?
            }
        };

        let field_expr = match self.exprs.pop() {
            Some(expr) => expr,
            None => {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            "Expected expression before index expression".to_string(),
                            metadata.into()
                        )
                    )
                )?
            }
        };

        self.exprs.push(
            Expr::IndexExpr(
                IndexExpr::new(
                    self.ast_arena.alloc_expr(field_expr),
                    self.ast_arena.alloc_expr(index_expr)
                )
            )
        );

        Ok(())
    }

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

    pub fn emit_array(&mut self, args_count: usize) {
        let items = self.exprs.split_off(self.exprs.len() - args_count);

        self.exprs.push(Expr::ArrayExpr(ArrayExpr::new(items)));
    }

    pub fn emit_call(
        &mut self,
        args_count: usize,
        left_paren_metadata: TokenMetadata
    ) -> Result<(), CompileError> {
        let args = self.exprs.split_off(self.exprs.len() - args_count);
        let ident_expr = match self.exprs.pop() {
            Some(expr) => {
                match expr {
                    Expr::IdentifierExpr(ident_expr) => ident_expr,
                    _ => panic!("Only ident expr supported in call for now"),
                }
            }
            None =>
                Err(
                    CompileError::new(
                        ReportedError::new(
                            "Expected identifier in call".to_string(),
                            left_paren_metadata.into()
                        )
                    )
                )?,
        };
        self.exprs.push(
            Expr::FnCallExpr(
                FnCallExpr::new(ident_expr.get_ssa_ident().clone(), ident_expr.get_metadata(), args)
            )
        );

        Ok(())
    }

    pub fn emit_ident(&mut self, identifier_expr: IdentifierExpr) {
        self.exprs.push(Expr::IdentifierExpr(identifier_expr))
    }

    pub fn emit_const_lit(&mut self, literal_expr: LiteralExpr) {
        self.exprs.push(Expr::LiteralExpr(literal_expr))
    }

    pub fn emit_prefix_unary_op(
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
    ArrayExpr(ArrayExpr<'ast>),
    IndexExpr(IndexExpr<'ast>),
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
    fn get_result_type(&self) -> ValueType {
        match self {
            Self::GroupingExpr(expr) => expr.get_result_type(),
            Self::BinaryExpr(expr) => expr.get_result_type(),
            Self::UnaryExpr(expr) => expr.get_result_type(),
            Self::LiteralExpr(expr) => expr.get_result_type(),
            Self::IdentifierExpr(expr) => expr.get_result_type(),
            Self::FnCallExpr(expr) => expr.get_result_type(),
            Self::ArrayExpr(expr) => expr.get_result_type(),
            Self::IndexExpr(expr) => expr.get_result_type(),
        }
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<(ValueType, Option<SSAIdent>), CompileError> {
        match self {
            Self::GroupingExpr(expr) => expr.type_check(program_symbol_table),
            Self::BinaryExpr(expr) => expr.type_check(program_symbol_table),
            Self::UnaryExpr(expr) => expr.type_check(program_symbol_table),
            Self::LiteralExpr(expr) => expr.type_check(program_symbol_table),
            Self::IdentifierExpr(expr) => expr.type_check(program_symbol_table),
            Self::FnCallExpr(expr) => expr.type_check(program_symbol_table),
            Self::ArrayExpr(expr) => expr.type_check(program_symbol_table),
            Self::IndexExpr(expr) => expr.type_check(program_symbol_table),
        }
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder,
        declaring_ssa_ident: Option<&SSAIdent>
    ) -> usize {
        match self {
            Self::GroupingExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::BinaryExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::UnaryExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::LiteralExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::IdentifierExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::FnCallExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::ArrayExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
            Self::IndexExpr(expr) =>
                expr.compile_into_dag(dag, ident_node_id_map, icfg_builder, declaring_ssa_ident),
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
            Self::ArrayExpr(expr) => expr.collect_metadata(),
            Self::IndexExpr(expr) => expr.collect_metadata(),
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
            Self::ArrayExpr(expr) => expr.dissasemble(),
            Self::IndexExpr(expr) => expr.dissasemble(),
        }
    }
}
