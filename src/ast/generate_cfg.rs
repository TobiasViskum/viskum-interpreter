use std::cell::Cell;

use crate::{
    ast::stmt::Stmt,
    compiler::cfg::{
        cfg_node::{ CFGNodeState, CFGProcessNode },
        dag::{ DAGNode, DAGOp, DAG },
        CFGNode,
        CFG,
    },
};

use super::Ast;

impl Ast {
    pub fn generate_cfg(&self) -> CFG {
        let mut cfg = CFG::new();

        self.type_check();

        let next_cfg_node_id: Cell<usize> = Cell::new(1);
        let get_next_node_id = || {
            let next_node_id = next_cfg_node_id.get();
            next_cfg_node_id.set(next_node_id + 1);
            next_node_id
        };

        cfg.add_node(CFGNode::ProgramStart(get_next_node_id()));

        for stmt in &self.main_scope.cf_stmts {
            self.generate_cfg_node(stmt, &mut cfg, &get_next_node_id);
        }

        cfg.add_node(CFGNode::ProgramEnd);

        cfg
    }

    fn generate_cfg_node(&self, stmt: &Stmt, cfg: &mut CFG, get_next_node_id: &impl Fn() -> usize) {
        let temp_default_state = CFGNodeState::Alive;

        match stmt {
            Stmt::IfStmt(_) => {}
            Stmt::TypeDefStmt(_) => {}
            Stmt::FunctionStmt(_) => { panic!("FunctionStmt is not supported yet") }
            Stmt::VariableDefinition(variable_definition) => {
                let mut dag = DAG::new();

                let value_id = match &variable_definition.value {
                    Some(value) => Some(value.compile_to_dag_node(&mut dag)),
                    None => None,
                };

                let lexeme_id = dag.add_node(
                    DAGNode::new(DAGOp::Identifier(variable_definition.name.clone()), None)
                );

                let operands = match value_id {
                    Some(value_id) => vec![lexeme_id, value_id],
                    None => vec![lexeme_id],
                };

                let entry_node_id = dag.add_node(DAGNode::new(DAGOp::Define, Some(operands)));
                dag.set_entry_node_id(entry_node_id);

                let cfg_process_node = CFGProcessNode::new(
                    dag,
                    get_next_node_id(),
                    temp_default_state
                );

                cfg.add_node(CFGNode::Process(cfg_process_node));
            }
            Stmt::ExprStmt(expr_stmt) => {
                let dag = expr_stmt.compile_to_dag();

                let cfg_process_node = CFGProcessNode::new(
                    dag,
                    get_next_node_id(),
                    temp_default_state
                );

                cfg.add_node(CFGNode::Process(cfg_process_node));
            }
            Stmt::VariableAssignment(variable_assignment) => {
                let mut dag = DAG::new();

                let value_id = variable_assignment.value.compile_to_dag_node(&mut dag);

                let lexeme_id = dag.add_node(
                    DAGNode::new(
                        DAGOp::Identifier(variable_assignment.field.get_lexeme().clone()),
                        None
                    )
                );

                let entry_node_id = dag.add_node(
                    DAGNode::new(DAGOp::Assign, Some(vec![lexeme_id, value_id]))
                );
                dag.set_entry_node_id(entry_node_id);

                let cfg_process_node = CFGProcessNode::new(
                    dag,
                    get_next_node_id(),
                    temp_default_state
                );

                cfg.add_node(CFGNode::Process(cfg_process_node));
            }
            Stmt::ScopeStmt(scope_stmt) => {
                cfg.add_node(CFGNode::ScopeStart(get_next_node_id()));

                for stmt in &scope_stmt.cf_stmts {
                    self.generate_cfg_node(stmt, cfg, get_next_node_id);
                }

                cfg.add_node(CFGNode::ScopeEnd(get_next_node_id()));
            }
        }
    }
}
