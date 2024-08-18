mod block_stmt;
mod break_stmt;
mod continue_stmt;
// mod drop_stmt;
mod expr_stmt;
mod fn_stmt;
mod if_stmt;
mod implicit_ret_stmt;
mod loop_stmt;
mod return_stmt;
mod var_assign_stmt;
mod var_def_stmt;
// mod block_stmt;

use std::{ collections::VecDeque, ops::Index, rc::Rc };

use ahash::AHashMap;
pub use block_stmt::BlockStmt;
pub use break_stmt::BreakStmt;
pub use expr_stmt::ExprStmt;
pub use var_assign_stmt::VarAssignStmt;
pub use var_def_stmt::VarDefStmt;
// pub use block_stmt::BlockStmt;
pub use continue_stmt::ContinueStmt;
// pub use drop_stmt::DropStmt;
pub use fn_stmt::FunctionStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use return_stmt::ReturnStmt;

use crate::compiler::{
    ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::ValueType },
    error_handler::{ CompileError, ErrorHandler },
    ir::icfg::{
        cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGProcessNode, CFG },
        dag::DAG,
        icfg_builder::{ CFGBuilder, ICFGBuilder },
        ICFG,
    },
    parser::token::TokenMetadata,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

pub struct GotoNodeIds {
    break_node_ids: Vec<usize>,
    continue_node_ids: Vec<usize>,
    return_node_ids: Vec<usize>,
}

macro_rules! def_ops {
    ($name:ident) => {
        paste::paste! {
            pub fn [<get_ $name _ids>](&self) -> &Vec<usize> {
                &self.[<$name _ids>]
            }

            pub fn [<push_ $name _id>](&mut self, [<$name _id>]: usize) {
                self.[<$name _ids>].push([<$name _id>])
            }

            pub fn [<take_ $name _ids>](&mut self) -> Vec<usize> {
                std::mem::take(&mut self.[<$name _ids>])
            }

            pub fn [<replace_ $name _ids>](&mut self, [<$name _ids>]: Vec<usize>) {
                self.[<$name _ids>] = [<$name _ids>];
            }
        }
    };
}

impl GotoNodeIds {
    pub fn new() -> Self {
        Self {
            break_node_ids: vec![],
            continue_node_ids: vec![],
            return_node_ids: vec![],
        }
    }

    def_ops!(break_node);
    def_ops!(continue_node);
    def_ops!(return_node);
}

#[derive(Debug)]
pub enum Stmt<'ast> {
    ExprStmt(ExprStmt<'ast>),
    VarDefStmt(VarDefStmt<'ast>),
    VarAssignStmt(VarAssignStmt<'ast>),
    BlockStmt(BlockStmt<'ast>),
    FunctionStmt(FunctionStmt<'ast>),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ReturnStmt(ReturnStmt<'ast>),
    // DropStmt(DropStmt),
    IfStmt(IfStmt<'ast>),
    LoopStmt(LoopStmt<'ast>),
}

impl<'ast> Dissasemble for Stmt<'ast> {
    fn dissasemble(&self) -> String {
        match self {
            Self::ExprStmt(expr_stmt) => format!("{}\n", expr_stmt.dissasemble()),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.dissasemble(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.dissasemble(),
            Self::BlockStmt(scope_stmt) => scope_stmt.dissasemble(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.dissasemble(),
            Self::BreakStmt(break_stmt) => break_stmt.dissasemble(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.dissasemble(),
            Self::ReturnStmt(return_stmt) => return_stmt.dissasemble(),
            Self::IfStmt(if_stmt) => if_stmt.dissasemble(),
            Self::LoopStmt(loop_stmt) => loop_stmt.dissasemble(),
            // Self::DropStmt(drop_stmt) => drop_stmt.dissasemble(),
        }
    }
}

impl<'ast> StmtTrait for Stmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        match self {
            Self::ExprStmt(expr_stmt) =>
                expr_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::VarDefStmt(var_def_stmt) => {
                var_def_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::VarAssignStmt(var_assign_stmt) => {
                var_assign_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::BlockStmt(scope_stmt) => {
                scope_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::FunctionStmt(fn_stmt) =>
                fn_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::BreakStmt(break_stmt) => {
                break_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::ContinueStmt(continue_stmt) => {
                continue_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::ReturnStmt(return_stmt) => {
                return_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::IfStmt(if_stmt) =>
                if_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::LoopStmt(loop_stmt) =>
                loop_stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            // Self::DropStmt(drop_stmt) =>
            //     drop_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids),
        }
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::VarDefStmt(var_def_stmt) => {
                var_def_stmt.validate_stmt(symbol_table_ref, error_handler)
            }
            Self::VarAssignStmt(var_assign_stmt) => {
                var_assign_stmt.validate_stmt(symbol_table_ref, error_handler)
            }
            Self::BlockStmt(scope_stmt) => {
                scope_stmt.validate_stmt(symbol_table_ref, error_handler)
            }
            Self::FunctionStmt(fn_stmt) => fn_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::BreakStmt(break_stmt) => {
                break_stmt.validate_stmt(symbol_table_ref, error_handler)
            }
            Self::ContinueStmt(continue_stmt) => {
                continue_stmt.validate_stmt(symbol_table_ref, error_handler)
            }
            Self::ReturnStmt(return_stmt) => {
                return_stmt.validate_stmt(symbol_table_ref, error_handler)
            }
            Self::IfStmt(if_stmt) => if_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::LoopStmt(loop_stmt) => loop_stmt.validate_stmt(symbol_table_ref, error_handler),
            // Self::DropStmt(drop_stmt) => drop_stmt.validate_stmt(symbol_table_ref, error_handler),
        }
    }

    fn is_linear_control_flow(&self) -> bool {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.is_linear_control_flow(),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.is_linear_control_flow(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.is_linear_control_flow(),
            Self::BlockStmt(scope_stmt) => scope_stmt.is_linear_control_flow(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.is_linear_control_flow(),
            Self::BreakStmt(break_stmt) => break_stmt.is_linear_control_flow(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.is_linear_control_flow(),
            Self::ReturnStmt(return_stmt) => return_stmt.is_linear_control_flow(),
            Self::IfStmt(if_stmt) => if_stmt.is_linear_control_flow(),
            Self::LoopStmt(loop_stmt) => loop_stmt.is_linear_control_flow(),
            // Self::DropStmt(drop_stmt) => drop_stmt.is_linear_control_flow(),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.as_linear_control_flow(),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.as_linear_control_flow(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.as_linear_control_flow(),
            Self::BlockStmt(scope_stmt) => scope_stmt.as_linear_control_flow(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.as_linear_control_flow(),
            Self::BreakStmt(break_stmt) => break_stmt.as_linear_control_flow(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.as_linear_control_flow(),
            Self::ReturnStmt(return_stmt) => return_stmt.as_linear_control_flow(),
            Self::IfStmt(if_stmt) => if_stmt.as_linear_control_flow(),
            Self::LoopStmt(loop_stmt) => loop_stmt.as_linear_control_flow(),
            // Self::DropStmt(drop_stmt) => drop_stmt.as_linear_control_flow(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnArg {
    ssa_ident: SSAKey,
    value_type: ValueType,
    mut_keyword_metadata: Option<TokenMetadata>,
    ident_metadata: TokenMetadata,
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: Rc<str>,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub metadata: TokenMetadata,
}
