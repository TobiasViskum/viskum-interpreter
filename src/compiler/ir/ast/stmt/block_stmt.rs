use std::collections::VecDeque;

use ahash::AHashMap;

use crate::compiler::{
    ds::symbol_table::{ SSAKey, SymbolTableRef },
    error_handler::{ CompileError, ErrorHandler },
    ir::icfg::{
        cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGProcessNode, CFG },
        dag::DAG,
        icfg_builder::ICFGBuilder,
        ICFG,
    },
    print_todo,
    traits::{ Dissasemble, LinearControlFlow, StmtTrait },
};

use super::{ DropStmt, GotoNodeIds, Stmt, Stmts };

pub enum ScopeEnv {
    BasicBlock,
    If,
    Loop,
    WhileLoop,
}

#[derive(Debug)]
pub struct BlockStmt<'ast> {
    stmts: VecDeque<Stmt<'ast>>,
    symbol_table_ref: SymbolTableRef,
    is_basic_block: bool,
    // forwards_declarations: Stmts, // TypeDefStmt, FnStmt, (ClassStmt)
}

impl<'ast> BlockStmt<'ast> {
    pub fn new(symbol_table_ref: SymbolTableRef, is_basic_block: bool) -> Self {
        Self {
            stmts: VecDeque::new(),
            symbol_table_ref,
            is_basic_block,
            // symbol_table_ref,
        }
    }

    pub fn set_is_basic_block(&mut self, new_state: bool) {
        self.is_basic_block = new_state;
    }

    pub fn get_symbol_table_ref(&self) -> SymbolTableRef {
        self.symbol_table_ref
    }

    pub fn push_stmt(&mut self, stmt: Stmt<'ast>) {
        print_todo("Remove result from below function (wait to declare func until validate_stmt)");
        match stmt {
            Stmt::FunctionStmt(ref fn_stmt) => {
                // let result = self.symbol_table_ref.get_mut().declare_fn(fn_stmt);
                self.stmts.push_front(stmt);
                // result
            }
            _ => self.stmts.push_back(stmt),
        }
    }

    pub fn compile_linear_stmts_into_icfg(&self, i: &mut usize) -> CFGNode {
        let mut dag = DAG::new();
        let mut ident_node_id_map = AHashMap::new();
        let mut prev_dag_node_id: Option<usize> = None;

        while let Some(linear_cf) = self.stmts[*i].as_linear_control_flow() {
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

impl<'ast> Dissasemble for BlockStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::new();
        for stmt in &self.stmts {
            string_builder += stmt.dissasemble().as_str();
        }
        string_builder
    }
}

impl<'ast> StmtTrait for BlockStmt<'ast> {
    fn compile_into_icfg(&self, icfg_builder: &mut ICFGBuilder, goto_node_ids: &mut GotoNodeIds) {
        self.stmts.iter().for_each(|stmt| {
            if let Some(linear_stmt) = stmt.as_linear_control_flow() {
                icfg_builder.build_into_linear_basic_block(linear_stmt)
            } else {
                icfg_builder.push_linear_block_if_exists();
                stmt.compile_into_icfg(icfg_builder, goto_node_ids)
            }
        });
        if self.is_basic_block {
            icfg_builder.push_linear_block_if_exists()
        }
    }

    fn validate_stmt(&mut self, _: &mut SymbolTableRef, error_handler: &mut ErrorHandler) {
        let symbol_table_ref = &mut self.symbol_table_ref;
        self.stmts.iter_mut().for_each(|stmt| {
            stmt.validate_stmt(symbol_table_ref, error_handler);
        });

        let scope_symbol_table = symbol_table_ref.get();

        // let all_vars_in_scope = scope_symbol_table.get_all_vars();
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

impl<'ast> LinearControlFlow for BlockStmt<'ast> {
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
