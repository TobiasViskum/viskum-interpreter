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
        icfg_builder::{ CFGBuilder },
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
        symbol_table_ref: &mut SymbolTableRef
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
}

impl<'ast> StmtTrait for Stmts<'ast> {
    fn compile_into_icfg(
        &self,
        icfg: &mut ICFG,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        self.stmts.iter().for_each(|stmt| {
            if let Some(linear_stmt) = stmt.as_linear_control_flow() {
            }
        })

        // let mut first_added_node_id: Option<usize> = None;
        // let mut last_added_node_id: Option<usize> = None;

        // macro_rules! updated_added_node_ids {
        //     ($node_id:expr) => {
        //         if first_added_node_id.is_none() {
        //             first_added_node_id = Some($node_id);
        //         }
        //         match last_added_node_id {
        //             Some(last_node_id) if $node_id > last_node_id => {
        //                 last_added_node_id = Some($node_id);
        //             }
        //             _ => last_added_node_id = Some($node_id)
        //         }
        //     };
        // }

        // let mut prev_node_ids_range: Option<NodeIdsRange> = None;
        // let mut curr_node_ids_range: Option<NodeIdsRange> = None;

        // self.stmts.iter().for_each(|stmt| {
        //     if let Some(linear_stmt) = stmt.as_linear_control_flow() {
        //         icfg_builder.build_into_linear_basic_block(linear_stmt);
        //     } else {
        //         let linear_block = icfg_builder.push_linear_block_if_exists();
        //         if let Some(linear_block_node_id) = linear_block {
        //             updated_added_node_ids!(linear_block_node_id);
        //             prev_node_ids_range = Some(
        //                 NodeIdsRange::new(linear_block_node_id, linear_block_node_id)
        //             );
        //         }

        //         if let Some(node_ids_range) = stmt.compile_into_icfg(icfg_builder, goto_node_ids) {
        //             curr_node_ids_range = Some(node_ids_range);
        //             updated_added_node_ids!(node_ids_range.get_first_added_node_id());
        //             updated_added_node_ids!(node_ids_range.get_last_added_node_id());
        //         }
        //     }

        //     match (prev_node_ids_range.clone(), curr_node_ids_range.clone()) {
        //         (Some(prev_ids_range), Some(curr_ids_range)) => {
        //             icfg_builder.push_cfg_edge(
        //                 prev_ids_range.get_last_added_node_id(),
        //                 curr_ids_range.get_first_added_node_id()
        //             );

        //             prev_node_ids_range = curr_node_ids_range.clone();
        //             curr_node_ids_range = None;
        //         }
        //         _ => {}
        //     }

        //     if goto_node_ids.get_if_branch_end_node_ids().len() > 0 {
        //         for if_branch_end_node_id in goto_node_ids.take_if_branch_end_node_ids() {
        //             println!(
        //                 "if_branch_end_node_id: {}, last_added_node_id: {:?}",
        //                 if_branch_end_node_id,
        //                 last_added_node_id
        //             );
        //             if let Some(last_added_node_id) = last_added_node_id.clone() {
        //                 icfg_builder.push_cfg_edge(if_branch_end_node_id, last_added_node_id + 1);
        //             }
        //         }
        //     }
        // });

        // match (first_added_node_id, last_added_node_id) {
        //     (Some(first_node_id), Some(last_node_id)) =>
        //         Some(NodeIdsRange::new(first_node_id, last_node_id)),
        //     (Some(first_node_id), None) => Some(NodeIdsRange::new(first_node_id, first_node_id)),
        //     (None, Some(last_node_id)) => Some(NodeIdsRange::new(last_node_id, last_node_id)),
        //     (None, None) => None,
        // }
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        self.iter_mut().for_each(|stmt| {
            stmt.validate_stmt(symbol_table_ref, error_handler);
        });

        let scope_symbol_table = symbol_table_ref.get();

        let all_vars_in_scope = scope_symbol_table.get_all_vars();
        // if all_vars_in_scope.len() > 0 {
        //     self.stmts.push_back(Stmt::DropStmt(DropStmt::new(all_vars_in_scope)))
        // }
    }

    fn is_linear_control_flow(&self) -> bool {
        self.stmts
            .iter()
            .filter(|&stmt| !stmt.is_linear_control_flow())
            .count() == 0
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        if self.is_linear_control_flow() { Some(self) } else { None }
    }
}

impl<'ast> LinearControlFlow for Stmts<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let linear_stmts = self.stmts
            .iter()
            .filter_map(|stmt| stmt.as_linear_control_flow())
            .collect::<Vec<_>>();

        for linear_stmt in linear_stmts {
            let node_id = linear_stmt.compile_into_dag(dag, ident_node_id_map);
            dag.set_entry_node_id(node_id);
        }

        dag.get_entry_node_id()
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
        icfg: &mut ICFG,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        match self {
            Self::ExprStmt(expr_stmt) =>
                expr_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids),
            Self::VarDefStmt(var_def_stmt) => {
                var_def_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids)
            }
            Self::VarAssignStmt(var_assign_stmt) => {
                var_assign_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids)
            }
            Self::BlockStmt(scope_stmt) => {
                scope_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids)
            }
            Self::FunctionStmt(fn_stmt) =>
                fn_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids),
            Self::BreakStmt(break_stmt) => {
                break_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids)
            }
            Self::ContinueStmt(continue_stmt) => {
                continue_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids)
            }
            Self::ReturnStmt(return_stmt) => {
                return_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids)
            }
            Self::IfStmt(if_stmt) => if_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids),
            Self::LoopStmt(loop_stmt) =>
                loop_stmt.compile_into_icfg(icfg, cfg_builder, goto_node_ids),
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
pub struct FunctionArgument {
    pub name: Rc<str>,
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub metadata: TokenMetadata,
}
