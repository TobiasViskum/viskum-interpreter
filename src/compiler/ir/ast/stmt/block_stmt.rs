use std::collections::VecDeque;

use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, symbol_table::UserSymbolFn },
    error_handler::{ CompileError, ErrorHandler },
    ir::icfg::{
        cfg::{ CFGLabelNode, CFGNode, CFGNodeType, CFGProcessNode },
        dag::DAG,
        icfg_builder::{ CFGBuilder, ICFGBuilder },
    },
    print_todo,
    traits::{ AstDissasemble, Dissasemble, LinearControlFlow, StmtTrait },
    ProgramSymbolTablePhase1,
    SymbolFn,
};

use super::{ GotoNodeIds, Stmt };

#[derive(Debug)]
pub struct BlockStmt<'ast> {
    stmts: VecDeque<Stmt<'ast>>,
    symbol_table_id: usize,
    is_basic_block: bool,
}

impl<'ast> BlockStmt<'ast> {
    pub fn new(symbol_table_id: usize) -> Self {
        Self {
            stmts: VecDeque::new(),
            symbol_table_id,
            is_basic_block: true,
            // symbol_table_ref,
        }
    }

    pub fn iter_stmts(&self) -> std::collections::vec_deque::Iter<Stmt<'ast>> {
        self.stmts.iter()
    }

    pub fn iter_mut_stmts(&mut self) -> std::collections::vec_deque::IterMut<Stmt<'ast>> {
        self.stmts.iter_mut()
    }

    pub fn set_is_basic_block(&mut self, new_state: bool) {
        self.is_basic_block = new_state;
    }

    pub fn get_symbol_table_id(&self) -> usize {
        self.symbol_table_id
    }

    #[must_use]
    pub fn push_stmt(
        &mut self,
        stmt: Stmt<'ast>,
        program_symbol_table: &mut ProgramSymbolTablePhase1
    ) -> Result<(), CompileError> {
        print_todo("Remove result from below function (wait to declare func until validate_stmt)");
        match stmt {
            Stmt::FunctionStmt(ref fn_stmt) => {
                let result = program_symbol_table.insert_fn(
                    fn_stmt.get_ssa_ident().clone(),
                    UserSymbolFn::new(
                        fn_stmt.get_ident_metadata(),
                        fn_stmt.get_args().clone(),
                        fn_stmt.get_return_type().clone()
                    )
                );
                self.stmts.push_front(stmt);
                result
            }
            _ => {
                self.stmts.push_back(stmt);
                Ok(())
            }
        }
    }

    pub fn compile_linear_stmts_into_icfg(
        &self,
        i: &mut usize,
        icfg_builder: &mut ICFGBuilder
    ) -> CFGNode {
        let mut dag = DAG::new();
        let mut ident_node_id_map = AHashMap::new();
        let mut prev_dag_node_id: Option<usize> = None;

        while let Some(linear_cf) = self.stmts[*i].as_linear_control_flow() {
            let dag_node_id = linear_cf.compile_into_dag(
                &mut dag,
                &mut ident_node_id_map,
                icfg_builder
            );

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

impl<'ast> StmtTrait for BlockStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        self.stmts.iter().for_each(|stmt| {
            if let Some(linear_stmt) = stmt.as_linear_control_flow() {
                cfg_builder.build_into_linear_basic_block(linear_stmt, icfg_builder)
            } else {
                cfg_builder.push_linear_block_if_exists();
                stmt.compile_into_icfg(icfg_builder, cfg_builder, goto_node_ids);

                if let Stmt::IfStmt(_) = &stmt {
                    let label_node_id_after_if = cfg_builder.push_cfg_node(
                        CFGNode::new(CFGNodeType::LabelNode(CFGLabelNode))
                    );
                    cfg_builder.push_cfg_edge(
                        label_node_id_after_if,
                        cfg_builder.get_next_cfg_node_id()
                    )
                }
            }
        });
        if !self.is_basic_block {
            cfg_builder.push_linear_block_if_exists()
        }
    }

    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        let prev_symbol_table_id = program_symbol_table.get_current_symbol_table_id();
        program_symbol_table.set_current_symbol_table_id(self.symbol_table_id);

        self.stmts.iter_mut().for_each(|stmt| {
            stmt.validate_stmt(program_symbol_table, error_handler);
        });

        program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id)
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
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        let linear_stmts = self.stmts
            .iter()
            .filter_map(|stmt| stmt.as_linear_control_flow())
            .collect::<Vec<_>>();

        for linear_stmt in linear_stmts {
            let node_id = linear_stmt.compile_into_dag(dag, ident_node_id_map, icfg_builder);
            dag.set_entry_node_id(node_id);
        }

        dag.get_entry_node_id()
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

impl<'ast> AstDissasemble for BlockStmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        mut scope_depth: usize
    ) -> String {
        scope_depth += 1;
        let prev_symbol_table_id = program_symbol_table.get_current_symbol_table_id();
        program_symbol_table.set_current_symbol_table_id(self.symbol_table_id);

        let mut string_builder = String::new();
        for stmt in self.stmts.iter() {
            string_builder += stmt.ast_dissasemble(program_symbol_table, scope_depth).as_str();
        }

        program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id);

        string_builder
    }
}
