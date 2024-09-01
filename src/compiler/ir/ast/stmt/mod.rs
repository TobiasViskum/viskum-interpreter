mod block_stmt;
mod break_stmt;
mod continue_stmt;
mod expr_stmt;
mod fn_stmt;
mod if_stmt;
mod implicit_ret_stmt;
mod loop_stmt;
mod return_stmt;
mod var_assign_stmt;
mod var_def_stmt;
mod var_declaration_stmt;
// mod drop_stmt;

pub use block_stmt::BlockStmt;
pub use break_stmt::BreakStmt;
pub use expr_stmt::ExprStmt;
pub use var_assign_stmt::VarAssignStmt;
pub use var_def_stmt::VarDefStmt;
pub use continue_stmt::ContinueStmt;
pub use fn_stmt::FunctionStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use return_stmt::ReturnStmt;
pub use var_declaration_stmt::VarDeclarationStmt;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::{ ErrorHandler, SrcCharsRange },
    ir::icfg::icfg_builder::{ CFGBuilder, ICFGBuilder },
    parser::token::TokenMetadata,
    traits::{ AstDissasemble, Dissasemble, LinearControlFlow, StmtTrait },
    ProgramSymbolTablePhase1,
};

use super::AST_DISSASEMBLE_INDENTATION;

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
    VarDeclarationStmt(VarDeclarationStmt),
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

impl<'ast> StmtTrait for Stmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        match self {
            Self::ExprStmt(stmt) =>
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::VarDefStmt(stmt) => {
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::VarAssignStmt(stmt) => {
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::BlockStmt(stmt) => {
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::FunctionStmt(stmt) =>
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::BreakStmt(stmt) => {
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::ContinueStmt(stmt) => {
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::ReturnStmt(stmt) => {
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids)
            }
            Self::IfStmt(stmt) => stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::LoopStmt(stmt) =>
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            Self::VarDeclarationStmt(stmt) =>
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids),
            // Self::DropStmt(drop_stmt) =>
            //     drop_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids),
        }
    }

    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        match self {
            Self::ExprStmt(stmt) => stmt.validate_stmt(program_symbol_table, error_handler),
            Self::VarDefStmt(stmt) => { stmt.validate_stmt(program_symbol_table, error_handler) }
            Self::VarAssignStmt(stmt) => { stmt.validate_stmt(program_symbol_table, error_handler) }
            Self::BlockStmt(stmt) => { stmt.validate_stmt(program_symbol_table, error_handler) }
            Self::FunctionStmt(stmt) => stmt.validate_stmt(program_symbol_table, error_handler),
            Self::BreakStmt(stmt) => { stmt.validate_stmt(program_symbol_table, error_handler) }
            Self::ContinueStmt(stmt) => { stmt.validate_stmt(program_symbol_table, error_handler) }
            Self::ReturnStmt(stmt) => { stmt.validate_stmt(program_symbol_table, error_handler) }
            Self::IfStmt(stmt) => stmt.validate_stmt(program_symbol_table, error_handler),
            Self::LoopStmt(stmt) => stmt.validate_stmt(program_symbol_table, error_handler),
            Self::VarDeclarationStmt(stmt) =>
                stmt.validate_stmt(program_symbol_table, error_handler),
            // Self::DropStmt(drop_stmt) => stmt.validate_stmt(symbol_table_ref, error_handler),
        }
    }

    fn is_linear_control_flow(&self) -> bool {
        match self {
            Self::ExprStmt(stmt) => stmt.is_linear_control_flow(),
            Self::VarDefStmt(stmt) => stmt.is_linear_control_flow(),
            Self::VarAssignStmt(stmt) => stmt.is_linear_control_flow(),
            Self::BlockStmt(stmt) => stmt.is_linear_control_flow(),
            Self::FunctionStmt(stmt) => stmt.is_linear_control_flow(),
            Self::BreakStmt(stmt) => stmt.is_linear_control_flow(),
            Self::ContinueStmt(stmt) => stmt.is_linear_control_flow(),
            Self::ReturnStmt(stmt) => stmt.is_linear_control_flow(),
            Self::IfStmt(stmt) => stmt.is_linear_control_flow(),
            Self::LoopStmt(stmt) => stmt.is_linear_control_flow(),
            Self::VarDeclarationStmt(stmt) => stmt.is_linear_control_flow(),
            // Self::DropStmt(drop_stmt) => drop_stmt.is_linear_control_flow(),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        match self {
            Self::ExprStmt(stmt) => stmt.as_linear_control_flow(),
            Self::VarDefStmt(stmt) => stmt.as_linear_control_flow(),
            Self::VarAssignStmt(stmt) => stmt.as_linear_control_flow(),
            Self::BlockStmt(stmt) => stmt.as_linear_control_flow(),
            Self::FunctionStmt(stmt) => stmt.as_linear_control_flow(),
            Self::BreakStmt(stmt) => stmt.as_linear_control_flow(),
            Self::ContinueStmt(stmt) => stmt.as_linear_control_flow(),
            Self::ReturnStmt(stmt) => stmt.as_linear_control_flow(),
            Self::IfStmt(stmt) => stmt.as_linear_control_flow(),
            Self::LoopStmt(stmt) => stmt.as_linear_control_flow(),
            Self::VarDeclarationStmt(stmt) => stmt.as_linear_control_flow(),
            // Self::DropStmt(drop_stmt) => drop_stmt.as_linear_control_flow(),
        }
    }
}

impl<'ast> Dissasemble for Stmt<'ast> {
    fn dissasemble(&self) -> String {
        match self {
            Self::ExprStmt(stmt) => format!("{}\n", stmt.dissasemble()),
            Self::VarDefStmt(stmt) => stmt.dissasemble(),
            Self::VarAssignStmt(stmt) => stmt.dissasemble(),
            Self::BlockStmt(stmt) => stmt.dissasemble(),
            Self::FunctionStmt(stmt) => stmt.dissasemble(),
            Self::BreakStmt(stmt) => stmt.dissasemble(),
            Self::ContinueStmt(stmt) => stmt.dissasemble(),
            Self::ReturnStmt(stmt) => stmt.dissasemble(),
            Self::IfStmt(stmt) => stmt.dissasemble(),
            Self::LoopStmt(stmt) => stmt.dissasemble(),
            Self::VarDeclarationStmt(stmt) => stmt.dissasemble(),
            // Self::DropStmt(drop_stmt) => drop_stmt.dissasemble(),
        }
    }
}

impl<'ast> AstDissasemble for Stmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        match self {
            Self::ExprStmt(stmt) =>
                format!(
                    "[{}]: {}{}\n",
                    program_symbol_table.get_current_symbol_table_id(),
                    " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
                    stmt.ast_dissasemble(program_symbol_table, scope_depth)
                ),
            Self::VarDefStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::VarAssignStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::BlockStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::FunctionStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::BreakStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::ContinueStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::ReturnStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::IfStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::LoopStmt(stmt) => stmt.ast_dissasemble(program_symbol_table, scope_depth),
            Self::VarDeclarationStmt(stmt) =>
                stmt.ast_dissasemble(program_symbol_table, scope_depth),
            // Self::DropStmt(drop_stmt) => drop_stmt.dissasemble(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnArg {
    ssa_ident: SSAIdent,
    value_type: ValueType,
    mut_keyword_metadata: Option<TokenMetadata>,
    ident_metadata: TokenMetadata,
}

impl FnArg {
    pub fn new(
        ssa_ident: SSAIdent,
        value_type: ValueType,
        mut_keyword_metadata: Option<TokenMetadata>,
        ident_metadata: TokenMetadata
    ) -> Self {
        Self { ssa_ident, value_type, mut_keyword_metadata, ident_metadata }
    }

    pub fn get_src_chars_range(&self) -> SrcCharsRange {
        self.ident_metadata.into()
    }
}
