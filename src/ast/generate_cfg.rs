use std::{ cell::Cell, rc::Rc };

use crate::{
    ast::stmt::Stmt,
    compiler::cfg::{
        cfg_node::{ CFGDecisionNode, CFGGotoNode, CFGProcessNode },
        dag::{ DAGNode, DAGOp, DAG },
        CFGNode,
        CFG,
    },
};

use super::{ stmt::{ IfStmt, LoopStmt }, Ast };

struct CFGHelper<'a> {
    pub get_next_node_id: &'a Box<dyn Fn() -> usize>,
    pub get_current_node_id: &'a Box<dyn Fn() -> usize>,
    pub scope: usize,
}

struct GotoNodeIds {
    pub break_node_ids: Vec<usize>,
    pub continue_node_ids: Vec<usize>,
}

impl GotoNodeIds {
    pub fn new() -> Self {
        Self {
            break_node_ids: vec![],
            continue_node_ids: vec![],
        }
    }

    pub fn extend(&mut self, other: GotoNodeIds) {
        self.break_node_ids.extend(other.break_node_ids);
        self.continue_node_ids.extend(other.continue_node_ids)
    }
}

impl<'a> CFGHelper<'a> {
    pub fn new(
        get_next_node_id: &'a Box<dyn Fn() -> usize>,
        get_current_node_id: &'a Box<dyn Fn() -> usize>
    ) -> Self {
        Self {
            get_next_node_id,
            get_current_node_id,
            scope: 0,
        }
    }

    pub fn increase_scope(&mut self) {
        self.scope += 1;
    }

    pub fn decrease_scope(&mut self) {
        self.scope -= 1;
    }
}

impl Ast {
    #[profiler::function_tracker]
    pub fn generate_cfg(&self) -> CFG {
        let mut cfg = CFG::new();

        self.type_check();

        let next_cfg_node_id: Rc<Cell<usize>> = Rc::new(Cell::new(1));

        let get_next_node_id = {
            let next_cfg_node_id = Rc::clone(&next_cfg_node_id);
            Box::new(move || {
                let next_node_id = next_cfg_node_id.get();
                next_cfg_node_id.set(next_node_id + 1);
                next_node_id
            }) as Box<dyn Fn() -> usize>
        };

        let get_current_node_id = {
            let next_cfg_node_id = Rc::clone(&next_cfg_node_id);
            Box::new(move || next_cfg_node_id.get() - 1) as Box<dyn Fn() -> usize>
        };

        let mut cfg_helper = CFGHelper::new(&get_next_node_id, &get_current_node_id);

        cfg.add_node(
            CFGNode::ProgramStart { next_id: get_next_node_id(), scope: cfg_helper.scope },
            &get_next_node_id
        );

        for stmt in &self.main_scope.cf_stmts {
            self.generate_cfg_nodes(stmt, &mut cfg, &mut cfg_helper);
        }

        cfg.add_node(CFGNode::ProgramEnd { scope: cfg_helper.scope }, &get_next_node_id);

        cfg
    }

    fn generate_cfg_nodes<'a>(
        &self,
        stmt: &Stmt,
        cfg: &mut CFG,
        cfg_helper: &'a mut CFGHelper
    ) -> GotoNodeIds {
        let mut goto_node_ids = GotoNodeIds::new();

        match stmt {
            Stmt::BreakStmt(_) => {
                let break_node_id = cfg.add_node(
                    CFGNode::Goto {
                        node: CFGGotoNode::new(0, None),
                        scope: cfg_helper.scope,
                    },
                    cfg_helper.get_next_node_id
                );

                goto_node_ids.break_node_ids.push(break_node_id);
            }
            Stmt::ContinueStmt(_) => {
                let break_node_id = cfg.add_node(
                    CFGNode::Goto {
                        node: CFGGotoNode::new(0, None),
                        scope: cfg_helper.scope,
                    },
                    cfg_helper.get_next_node_id
                );

                goto_node_ids.continue_node_ids.push(break_node_id);
            }
            Stmt::IfStmt(if_stmt) => {
                goto_node_ids.extend(
                    self.generate_cfg_nodes_from_if_stmt(if_stmt, cfg, cfg_helper)
                );
            }
            Stmt::LoopStmt(loop_stmt) => {
                goto_node_ids.extend(self.generate_cfg_nodes_from_loop(loop_stmt, cfg, cfg_helper));
            }
            Stmt::TypeDefStmt(_) => {}
            Stmt::FunctionStmt(_) => {
                panic!("FunctionStmt is not supported yet");
            }
            Stmt::VarDefStmt(variable_definition) => {
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

                let cfg_process_node = CFGProcessNode::new(dag, (cfg_helper.get_next_node_id)());

                cfg.add_node(
                    CFGNode::Process {
                        node: cfg_process_node,
                        scope: cfg_helper.scope,
                    },
                    cfg_helper.get_next_node_id
                );
            }
            Stmt::ExprStmt(expr_stmt) => {
                let dag = expr_stmt.compile_to_dag();

                let cfg_process_node = CFGProcessNode::new(dag, (cfg_helper.get_next_node_id)());

                cfg.add_node(
                    CFGNode::Process {
                        node: cfg_process_node,
                        scope: cfg_helper.scope,
                    },
                    cfg_helper.get_next_node_id
                );
            }
            Stmt::VarAssignStmt(variable_assignment) => {
                let mut dag = DAG::new();

                let value_id = variable_assignment.value.compile_to_dag_node(&mut dag);

                let lexeme_id = dag.add_node(
                    DAGNode::new(DAGOp::Identifier(variable_assignment.temp_get_lexeme()), None)
                );

                let entry_node_id = dag.add_node(
                    DAGNode::new(DAGOp::Assign, Some(vec![lexeme_id, value_id]))
                );
                dag.set_entry_node_id(entry_node_id);

                let cfg_process_node = CFGProcessNode::new(dag, (cfg_helper.get_next_node_id)());

                cfg.add_node(
                    CFGNode::Process {
                        node: cfg_process_node,
                        scope: cfg_helper.scope,
                    },
                    cfg_helper.get_next_node_id
                );
            }
            Stmt::ScopeStmt(scope_stmt) => {
                cfg_helper.increase_scope();

                for stmt in &scope_stmt.cf_stmts {
                    goto_node_ids.extend(self.generate_cfg_nodes(stmt, cfg, cfg_helper));
                }

                cfg_helper.decrease_scope();
            }
        }

        goto_node_ids
    }

    fn generate_cfg_nodes_from_loop<'a>(
        &self,
        loop_stmt: &LoopStmt,
        cfg: &mut CFG,
        cfg_helper: &'a mut CFGHelper
    ) -> GotoNodeIds {
        let mut goto_node_ids = GotoNodeIds::new();

        let condition = match &loop_stmt.condition {
            Some(expr) => Some(expr.compile_to_dag()),
            None => None,
        };

        let cfg_decision_node = CFGDecisionNode::new(condition, (cfg_helper.get_next_node_id)(), 0);

        let decision_node_id = cfg.add_node(
            CFGNode::Decision {
                node: cfg_decision_node,
                scope: cfg_helper.scope,
            },
            cfg_helper.get_next_node_id
        );

        cfg_helper.increase_scope();

        for stmt in &loop_stmt.body.cf_stmts {
            goto_node_ids.extend(self.generate_cfg_nodes(stmt, cfg, cfg_helper));
        }

        cfg.add_node(
            CFGNode::Goto {
                node: CFGGotoNode::new(decision_node_id, None),
                scope: cfg_helper.scope,
            },
            cfg_helper.get_next_node_id
        );

        cfg_helper.decrease_scope();

        let loop_exit_id = (cfg_helper.get_current_node_id)();

        match cfg.get_mut_node(decision_node_id).unwrap() {
            CFGNode::Decision { node, .. } => {
                node.false_branch_id = loop_exit_id;
            }
            _ => panic!("Could not set false_branch_id in generate_cfg_node"),
        }

        while goto_node_ids.continue_node_ids.len() != 0 {
            match cfg.get_mut_node(goto_node_ids.continue_node_ids.pop().unwrap()).unwrap() {
                CFGNode::Goto { node, .. } => {
                    node.goto_node_id = decision_node_id;
                }
                _ => panic!("Expected Goto node in generate_cfg_node"),
            }
        }

        while goto_node_ids.break_node_ids.len() != 0 {
            match cfg.get_mut_node(goto_node_ids.break_node_ids.pop().unwrap()).unwrap() {
                CFGNode::Goto { node, .. } => {
                    node.goto_node_id = loop_exit_id;
                }
                _ => panic!("Expected Goto node in generate_cfg_node"),
            }
        }

        goto_node_ids
    }

    fn generate_cfg_nodes_from_if_stmt<'a>(
        &self,
        if_stmt: &IfStmt,
        cfg: &mut CFG,
        cfg_helper: &'a mut CFGHelper
    ) -> GotoNodeIds {
        let mut goto_node_ids = GotoNodeIds::new();

        let condition = match &if_stmt.condition {
            Some(expr) => {
                let mut dag = expr.compile_to_dag();
                dag.ensure_comparison_to_entry_node();
                Some(dag)
            }
            None => None,
        };

        let cfg_decision_node = CFGDecisionNode::new(condition, (cfg_helper.get_next_node_id)(), 0);

        let decision_node_id = cfg.add_node(
            CFGNode::Decision {
                node: cfg_decision_node,
                scope: cfg_helper.scope,
            },
            cfg_helper.get_next_node_id
        );

        cfg_helper.increase_scope();

        for stmt in &if_stmt.true_block.cf_stmts {
            goto_node_ids.extend(self.generate_cfg_nodes(stmt, cfg, cfg_helper));
        }

        let goto_node_id = cfg.add_node(
            CFGNode::Goto { node: CFGGotoNode::new(0, None), scope: cfg_helper.scope },
            cfg_helper.get_next_node_id
        );

        cfg_helper.decrease_scope();

        let false_branch_id = if let Some(false_block) = &if_stmt.false_block {
            let false_branch_id = (cfg_helper.get_current_node_id)();
            goto_node_ids.extend(
                self.generate_cfg_nodes_from_if_stmt(false_block.as_ref(), cfg, cfg_helper)
            );

            false_branch_id
        } else {
            (cfg_helper.get_current_node_id)()
        };

        match cfg.get_mut_node(decision_node_id).unwrap() {
            CFGNode::Decision { node, .. } => {
                node.false_branch_id = false_branch_id;
            }
            _ => panic!("Could not set false_branch_id in generate_cfg_node"),
        }

        match cfg.get_mut_node(goto_node_id).unwrap() {
            CFGNode::Goto { node, .. } => {
                node.goto_node_id = (cfg_helper.get_current_node_id)();
            }
            _ => panic!("Could not set goto_node_id in generate_cfg_ndoe"),
        }

        goto_node_ids
    }
}
