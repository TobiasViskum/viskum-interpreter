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

use std::{ collections::VecDeque, ops::Index, rc::Rc };

use ahash::AHashMap;
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
    ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::ValueType },
    error_handler::{ CompileError, ErrorHandler },
    ir::icfg::{
        cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGProcessNode, CFG },
        dag::DAG,
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    parser::token::TokenMetadata,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

#[derive(Debug)]
pub struct Stmts<'ast> {
    stmts: VecDeque<Stmt<'ast>>,
}

impl<'ast> Dissasemble for Stmts<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::new();
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

    pub fn push(
        &mut self,
        stmt: Stmt<'ast>,
        symbol_table_ref: &SymbolTableRef
    ) -> Result<(), CompileError> {
        match stmt {
            Stmt::FunctionStmt(ref fn_stmt) => {
                let result = symbol_table_ref.get_mut().declare_fn(fn_stmt);
                self.stmts.push_front(stmt);
                result?;
            }
            _ => self.stmts.push_back(stmt),
        }

        Ok(())
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
        self.iter_mut().for_each(|stmt| stmt.validate_stmt(symbol_table_ref, error_handler))
    }

    pub fn compile_linear_stmts_into_icfg(&self, i: &mut usize) -> CFGNode {
        let mut dag = DAG::new();
        let mut ident_node_id_map = AHashMap::new();
        let mut prev_dag_node_id: Option<usize> = None;

        while let Some(linear_cf) = self.index(*i).as_linear_control_flow() {
            let dag_node_id = linear_cf.compile_into_dag(&mut dag, &mut ident_node_id_map);

            if let Some(prev_dag_node_id) = prev_dag_node_id {
                dag.add_edge(dag_node_id, prev_dag_node_id);
            }
            prev_dag_node_id = Some(dag_node_id);

            if *i >= self.stmts.len() - 1 {
                break;
            } else {
                if
                    let Some(is_next_linear) = self.stmts
                        .get(*i + 1)
                        .map(|stmt| stmt.is_linear_control_flow())
                {
                    if is_next_linear {
                        *i += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        CFGNode::new(CFGNodeType::ProcessNode(CFGProcessNode::new(dag)))
    }

    pub fn compile_into_icfg(&self, icfg_builder: &mut ICFGBuilder) {
        self.stmts
            .iter()
            .enumerate()
            .for_each(|(i, stmt)| {
                if let Some(linear_stmt) = stmt.as_linear_control_flow() {
                    icfg_builder.build_into_linear_basic_block(linear_stmt);
                } else {
                    stmt.compile_into_icfg(icfg_builder);
                }

                // if i != 0 {
                //     let stmt = &self.stmts[i]
                //     if stmt.
                // }
            });

        // loop {
        //     if i >= self.stmts.len() - 1 {
        //         break;
        //     }

        //     let node_id = {
        //         let stmt = &self.stmts[i];

        //         if stmt.is_linear_control_flow() {
        //             let cfg_node = self.compile_linear_stmts_into_icfg(&mut i);
        //         }
        //     };
        // }

        // let mut i: usize = 0;
        // let mut prev_node_id: Option<usize> = None;
        // loop {
        //     if i >= self.stmts.len() - 1 {
        //         break prev_node_id.expect(
        //             "Right now this wont work if there are 0 stmts inside Stmts<'ast> struct"
        //         );
        //     }

        //     let node_id = {
        //         let stmt = self.index(i);

        //         if stmt.is_linear_control_flow() {
        //             let cfg_node = self.compile_linear_stmts_into_icfg(&mut i);

        //             let cfg_node_id = current_cfg.push_node(cfg_node);
        //             if let Some(prev_node_id) = prev_node_id {
        //                 current_cfg.add_edge(prev_node_id, cfg_node_id);
        //             }

        //             cfg_node_id
        //         } else {
        //             let node_id = stmt.compile_into_icfg(icfg, current_cfg);

        //             node_id
        //         }
        //     };

        //     if let Some(prev_node_id) = prev_node_id {
        //         current_cfg.add_edge(prev_node_id, node_id);
        //     }

        //     prev_node_id = Some(node_id);
        //     i += 1;
        // }
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
    fn compile_into_icfg(&self, icfg_builder: &mut ICFGBuilder) {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.compile_into_icfg(icfg_builder),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.compile_into_icfg(icfg_builder),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.compile_into_icfg(icfg_builder),
            Self::ScopeStmt(scope_stmt) => scope_stmt.compile_into_icfg(icfg_builder),
            Self::FunctionStmt(fn_stmt) => fn_stmt.compile_into_icfg(icfg_builder),
            Self::BreakStmt(break_stmt) => break_stmt.compile_into_icfg(icfg_builder),
            Self::ContinueStmt(continue_stmt) => continue_stmt.compile_into_icfg(icfg_builder),
            Self::ReturnStmt(return_stmt) => return_stmt.compile_into_icfg(icfg_builder),
            Self::IfStmt(if_stmt) => if_stmt.compile_into_icfg(icfg_builder),
            Self::LoopStmt(loop_stmt) => loop_stmt.compile_into_icfg(icfg_builder),
        }
    }

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
