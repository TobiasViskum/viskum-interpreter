use std::{ collections::VecDeque, ops::Index, rc::Rc };

mod expr_stmt;
mod break_stmt;
mod var_def_stmt;
mod var_assign_stmt;
mod scope_stmt;
mod continue_stmt;
mod return_stmt;
mod if_stmt;
mod loop_stmt;
mod fn_stmt;

pub use expr_stmt::ExprStmt;
pub use break_stmt::BreakStmt;
pub use var_def_stmt::VarDefStmt;
pub use var_assign_stmt::VarAssignStmt;
pub use scope_stmt::ScopeStmt;
pub use continue_stmt::ContinueStmt;
pub use return_stmt::ReturnStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use fn_stmt::FunctionStmt;

use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::ValueType },
    error_handler::ErrorHandler,
    ir::icfg::dag::DAG,
    parser::token::TokenMetadata,
    Dissasemble,
};

pub trait LinearControlFlow: StmtTrait {
    fn compile_into_dag(&self, dag: &mut DAG);
}

pub trait StmtTrait {
    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    );

    fn is_linear_control_flow(&self) -> bool;

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow>;
}

#[derive(Debug)]
pub struct Stmts<'ast> {
    stmts: VecDeque<Stmt<'ast>>,
}

impl<'ast> Dissasemble for Stmts<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("");
        for stmt in &self.stmts {
            string_builder += stmt.dissasemble().as_str();
        }
        string_builder
    }
}

impl<'ast> Stmts<'ast> {
    pub fn new() -> Self {
        Self {
            stmts: VecDeque::new(),
        }
    }

    pub fn push(&mut self, stmt: Stmt<'ast>, symbol_table_ref: &SymbolTableRef) {
        match stmt {
            Stmt::FunctionStmt(ref fn_stmt) => {
                symbol_table_ref.get_mut().declare_fn(fn_stmt);
                self.stmts.push_front(stmt);
            }
            _ => self.stmts.push_back(stmt),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Stmt<'ast>> {
        self.stmts.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Stmt<'ast>> {
        self.stmts.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.stmts.len()
    }

    pub fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        for stmt in self.iter_mut() {
            stmt.validate_stmt(symbol_table_ref, error_handler);
        }
    }

    pub fn is_linear_control_flow(&self) -> bool {
        self.stmts
            .iter()
            .filter(|&stmt| !stmt.is_linear_control_flow())
            .count() == 0
    }
}

impl<'ast> Index<usize> for Stmts<'ast> {
    type Output = Stmt<'ast>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stmts[index]
    }
}

#[derive(Debug)]
pub enum Stmt<'ast> {
    ExprStmt(ExprStmt<'ast>),
    VarDefStmt(VarDefStmt<'ast>),
    VarAssignStmt(VarAssignStmt<'ast>),
    ScopeStmt(ScopeStmt<'ast>),
    FunctionStmt(FunctionStmt<'ast>),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ReturnStmt(ReturnStmt<'ast>),
    IfStmt(IfStmt<'ast>),
    LoopStmt(LoopStmt<'ast>),
}

impl<'ast> Dissasemble for Stmt<'ast> {
    fn dissasemble(&self) -> String {
        match self {
            Self::ExprStmt(expr_stmt) => format!("{}\n", expr_stmt.dissasemble()),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.dissasemble(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.dissasemble(),
            Self::ScopeStmt(scope_stmt) => scope_stmt.dissasemble(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.dissasemble(),
            Self::BreakStmt(break_stmt) => break_stmt.dissasemble(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.dissasemble(),
            Self::ReturnStmt(return_stmt) => return_stmt.dissasemble(),
            Self::IfStmt(if_stmt) => if_stmt.dissasemble(),
            Self::LoopStmt(loop_stmt) => loop_stmt.dissasemble(),
        }
    }
}

impl<'ast> StmtTrait for Stmt<'ast> {
    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::VarDefStmt(var_def_stmt) =>
                var_def_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::VarAssignStmt(var_assign_stmt) =>
                var_assign_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::ScopeStmt(scope_stmt) =>
                scope_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::FunctionStmt(fn_stmt) => fn_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::BreakStmt(break_stmt) =>
                break_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::ContinueStmt(continue_stmt) =>
                continue_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::ReturnStmt(return_stmt) =>
                return_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::IfStmt(if_stmt) => if_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::LoopStmt(loop_stmt) => loop_stmt.validate_stmt(symbol_table_ref, error_handler),
        }
    }

    fn is_linear_control_flow(&self) -> bool {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.is_linear_control_flow(),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.is_linear_control_flow(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.is_linear_control_flow(),
            Self::ScopeStmt(scope_stmt) => scope_stmt.is_linear_control_flow(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.is_linear_control_flow(),
            Self::BreakStmt(break_stmt) => break_stmt.is_linear_control_flow(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.is_linear_control_flow(),
            Self::ReturnStmt(return_stmt) => return_stmt.is_linear_control_flow(),
            Self::IfStmt(if_stmt) => if_stmt.is_linear_control_flow(),
            Self::LoopStmt(loop_stmt) => loop_stmt.is_linear_control_flow(),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.as_linear_control_flow(),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.as_linear_control_flow(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.as_linear_control_flow(),
            Self::ScopeStmt(scope_stmt) => scope_stmt.as_linear_control_flow(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.as_linear_control_flow(),
            Self::BreakStmt(break_stmt) => break_stmt.as_linear_control_flow(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.as_linear_control_flow(),
            Self::ReturnStmt(return_stmt) => return_stmt.as_linear_control_flow(),
            Self::IfStmt(if_stmt) => if_stmt.as_linear_control_flow(),
            Self::LoopStmt(loop_stmt) => loop_stmt.as_linear_control_flow(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
    pub name: Rc<str>,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub metadata: TokenMetadata,
}
