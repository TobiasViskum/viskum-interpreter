mod expr_stmt;
mod break_stmt;
mod var_def_stmt;
mod var_assign_stmt;
mod basic_block_stmt;
mod continue_stmt;
mod return_stmt;
mod if_stmt;
mod loop_stmt;
mod fn_stmt;
mod drop_stmt;
mod implicit_ret_stmt;
mod block_stmt;

use std::{ collections::VecDeque, ops::Index, rc::Rc };

use ahash::AHashMap;
pub use expr_stmt::ExprStmt;
pub use break_stmt::BreakStmt;
pub use var_def_stmt::VarDefStmt;
pub use var_assign_stmt::VarAssignStmt;
pub use basic_block_stmt::BasicBlockStmt;
pub use block_stmt::BlockStmt;
pub use continue_stmt::ContinueStmt;
pub use return_stmt::ReturnStmt;
pub use if_stmt::IfStmt;
pub use loop_stmt::LoopStmt;
pub use fn_stmt::FunctionStmt;
pub use drop_stmt::DropStmt;

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

pub struct GotoNodeIds {
    break_node_ids: Vec<usize>,
    continue_node_ids: Vec<usize>,
    return_node_ids: Vec<usize>,
    if_branch_end_node_ids: Vec<usize>,
}

macro_rules! def_ops {
    ($name:ident) => {
        paste::paste! {
            pub fn [<push_ $name _id>](&mut self, [<$name _id>]: usize) {
                self.[<$name _ids>].push([<$name _id>])
            }

            pub fn [<take_ $name _ids>](&mut self) -> Vec<usize> {
                std::mem::take(&mut self.[<$name _ids>])
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
            if_branch_end_node_ids: vec![],
        }
    }

    def_ops!(break_node);
    def_ops!(continue_node);
    def_ops!(return_node);
    def_ops!(if_branch_end_node);
}

#[derive(Debug, Clone, Copy)]
pub struct NodeIdsRange {
    first_added_node_id: usize,
    last_added_node_id: usize,
}

impl NodeIdsRange {
    pub fn new(first_added_node_id: usize, last_added_node_id: usize) -> Self {
        Self {
            first_added_node_id,
            last_added_node_id,
        }
    }

    pub fn get_first_added_node_id(&self) -> usize {
        self.first_added_node_id
    }

    pub fn get_last_added_node_id(&self) -> usize {
        self.last_added_node_id
    }

    pub fn set_first_added_node_id(&mut self, node_id: usize) {
        self.first_added_node_id = node_id;
    }

    pub fn set_last_added_node_id(&mut self, node_id: usize) {
        self.last_added_node_id = node_id;
    }
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
    type ReturnTypeCompileIntoICFG = Option<NodeIdsRange>;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        let mut first_added_node_id: Option<usize> = None;
        let mut last_added_node_id: Option<usize> = None;

        let mut updated_added_node_ids = |node_id: usize| {
            if first_added_node_id.is_none() {
                first_added_node_id = Some(node_id);
            }
            match last_added_node_id {
                Some(last_node_id) if node_id > last_node_id => {
                    last_added_node_id = Some(node_id);
                }
                _ => {}
            }
        };

        let mut prev_node_ids_range: Option<NodeIdsRange> = None;
        let mut curr_node_ids_range: Option<NodeIdsRange> = None;

        self.stmts.iter().for_each(|stmt| {
            if let Some(linear_stmt) = stmt.as_linear_control_flow() {
                icfg_builder.build_into_linear_basic_block(linear_stmt);
            } else {
                let linear_block = icfg_builder.push_linear_block_if_exists();
                if let Some(linear_block_node_id) = linear_block {
                    updated_added_node_ids(linear_block_node_id);
                    prev_node_ids_range = Some(
                        NodeIdsRange::new(linear_block_node_id, linear_block_node_id)
                    );
                }

                if let Some(node_ids_range) = stmt.compile_into_icfg(icfg_builder, goto_node_ids) {
                    curr_node_ids_range = Some(node_ids_range);
                    updated_added_node_ids(node_ids_range.get_first_added_node_id());
                    updated_added_node_ids(node_ids_range.get_last_added_node_id());
                }
            }

            match (prev_node_ids_range, curr_node_ids_range) {
                (Some(prev_ids_range), Some(curr_ids_range)) => {
                    icfg_builder.push_cfg_edge(
                        prev_ids_range.get_last_added_node_id(),
                        curr_ids_range.get_first_added_node_id()
                    );

                    if
                        curr_ids_range.get_first_added_node_id() !=
                        curr_ids_range.get_last_added_node_id()
                    {
                        // icfg_builder.push_cfg_edge(
                        //     curr_ids_range.get_first_added_node_id(),
                        //     curr_ids_range.get_last_added_node_id()
                        // );
                    }

                    prev_node_ids_range = curr_node_ids_range;
                    curr_node_ids_range = None;
                }
                _ => {}
            }
        });

        match (first_added_node_id, last_added_node_id) {
            (Some(first_node_id), Some(last_node_id)) =>
                Some(NodeIdsRange::new(first_node_id, last_node_id)),
            (Some(first_node_id), None) => Some(NodeIdsRange::new(first_node_id, first_node_id)),
            (None, Some(last_node_id)) => Some(NodeIdsRange::new(last_node_id, last_node_id)),
            (None, None) => None,
        }
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        println!("I run before");
        self.iter_mut().for_each(|stmt| {
            println!("Stmt: {:?}", stmt);
            stmt.validate_stmt(symbol_table_ref, error_handler);
            println!("Succesfully compiled stmt")
        });

        println!("I run after 1");

        let scope_symbol_table = symbol_table_ref.get();

        println!("I run after 2");

        let all_vars_in_scope = scope_symbol_table.get_all_vars();
        if all_vars_in_scope.len() > 0 {
            self.stmts.push_back(Stmt::DropStmt(DropStmt::new(all_vars_in_scope)))
        }
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
    BasicBlockStmt(BasicBlockStmt<'ast>),
    FunctionStmt(FunctionStmt<'ast>),
    BreakStmt(BreakStmt),
    ContinueStmt(ContinueStmt),
    ReturnStmt(ReturnStmt<'ast>),
    DropStmt(DropStmt),
    IfStmt(IfStmt<'ast>),
    LoopStmt(LoopStmt<'ast>),
}

impl<'ast> Dissasemble for Stmt<'ast> {
    fn dissasemble(&self) -> String {
        match self {
            Self::ExprStmt(expr_stmt) => format!("{}\n", expr_stmt.dissasemble()),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.dissasemble(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.dissasemble(),
            Self::BasicBlockStmt(scope_stmt) => scope_stmt.dissasemble(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.dissasemble(),
            Self::BreakStmt(break_stmt) => break_stmt.dissasemble(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.dissasemble(),
            Self::ReturnStmt(return_stmt) => return_stmt.dissasemble(),
            Self::IfStmt(if_stmt) => if_stmt.dissasemble(),
            Self::LoopStmt(loop_stmt) => loop_stmt.dissasemble(),
            Self::DropStmt(drop_stmt) => drop_stmt.dissasemble(),
        }
    }
}

impl<'ast> StmtTrait for Stmt<'ast> {
    type ReturnTypeCompileIntoICFG = Option<NodeIdsRange>;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        match self {
            Self::ExprStmt(expr_stmt) => {
                Some(expr_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::VarDefStmt(var_def_stmt) => {
                Some(var_def_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::VarAssignStmt(var_assign_stmt) => {
                Some(var_assign_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::BasicBlockStmt(scope_stmt) => {
                scope_stmt.compile_into_icfg(icfg_builder, goto_node_ids)
            }
            Self::FunctionStmt(fn_stmt) => {
                Some(fn_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::BreakStmt(break_stmt) => {
                Some(break_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::ContinueStmt(continue_stmt) => {
                Some(continue_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::ReturnStmt(return_stmt) => {
                Some(return_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::IfStmt(if_stmt) => {
                Some(if_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::LoopStmt(loop_stmt) => {
                Some(loop_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
            Self::DropStmt(drop_stmt) => {
                Some(drop_stmt.compile_into_icfg(icfg_builder, goto_node_ids))
            }
        }
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::VarDefStmt(var_def_stmt) =>
                var_def_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::VarAssignStmt(var_assign_stmt) =>
                var_assign_stmt.validate_stmt(symbol_table_ref, error_handler),
            Self::BasicBlockStmt(scope_stmt) =>
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
            Self::DropStmt(drop_stmt) => drop_stmt.validate_stmt(symbol_table_ref, error_handler),
        }
    }

    fn is_linear_control_flow(&self) -> bool {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.is_linear_control_flow(),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.is_linear_control_flow(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.is_linear_control_flow(),
            Self::BasicBlockStmt(scope_stmt) => scope_stmt.is_linear_control_flow(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.is_linear_control_flow(),
            Self::BreakStmt(break_stmt) => break_stmt.is_linear_control_flow(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.is_linear_control_flow(),
            Self::ReturnStmt(return_stmt) => return_stmt.is_linear_control_flow(),
            Self::IfStmt(if_stmt) => if_stmt.is_linear_control_flow(),
            Self::LoopStmt(loop_stmt) => loop_stmt.is_linear_control_flow(),
            Self::DropStmt(drop_stmt) => drop_stmt.is_linear_control_flow(),
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        match self {
            Self::ExprStmt(expr_stmt) => expr_stmt.as_linear_control_flow(),
            Self::VarDefStmt(var_def_stmt) => var_def_stmt.as_linear_control_flow(),
            Self::VarAssignStmt(var_assign_stmt) => var_assign_stmt.as_linear_control_flow(),
            Self::BasicBlockStmt(scope_stmt) => scope_stmt.as_linear_control_flow(),
            Self::FunctionStmt(fn_stmt) => fn_stmt.as_linear_control_flow(),
            Self::BreakStmt(break_stmt) => break_stmt.as_linear_control_flow(),
            Self::ContinueStmt(continue_stmt) => continue_stmt.as_linear_control_flow(),
            Self::ReturnStmt(return_stmt) => return_stmt.as_linear_control_flow(),
            Self::IfStmt(if_stmt) => if_stmt.as_linear_control_flow(),
            Self::LoopStmt(loop_stmt) => loop_stmt.as_linear_control_flow(),
            Self::DropStmt(drop_stmt) => drop_stmt.as_linear_control_flow(),
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
