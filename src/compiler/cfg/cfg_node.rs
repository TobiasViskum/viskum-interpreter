use super::dag::DAG;

#[derive(Debug, PartialEq)]
pub enum CFGNodeState {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct CFGBreakNode;

impl CFGBreakNode {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct CFGProcessNode {
    pub dag: DAG,
    pub next_id: usize,
    pub state: CFGNodeState,
}

impl CFGProcessNode {
    pub fn new(dag: DAG, next_id: usize) -> Self {
        Self {
            dag,
            next_id,
            state: CFGNodeState::Dead,
        }
    }
}

#[derive(Debug)]
pub struct CFGGotoNode {
    pub goto_node_id: usize,
    pub return_value: Option<DAG>,
    pub state: CFGNodeState,
}

impl CFGGotoNode {
    pub fn new(goto_node_id: usize, return_value: Option<DAG>) -> Self {
        Self { goto_node_id, return_value, state: CFGNodeState::Dead }
    }
}

#[derive(Debug)]
pub struct CFGDecisionNode {
    pub condition: Option<DAG>,
    pub true_branch_id: usize,
    pub false_branch_id: usize,
    pub state: CFGNodeState,
}

impl CFGDecisionNode {
    pub fn new(condition: Option<DAG>, true_branch_id: usize, false_branch_id: usize) -> Self {
        Self {
            condition,
            true_branch_id,
            false_branch_id,

            state: CFGNodeState::Dead,
        }
    }
}
