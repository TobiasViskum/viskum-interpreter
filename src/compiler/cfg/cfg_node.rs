use super::dag::DAG;

#[derive(Debug)]
pub enum CFGNodeState {
    Alive,
    Dead,
    Ignore,
}

#[derive(Debug)]
pub struct CFGProcessNode {
    pub dag: DAG,
    pub next_id: usize,
    pub state: CFGNodeState,
}

impl CFGProcessNode {
    pub fn new(dag: DAG, next_id: usize, node_state: CFGNodeState) -> Self {
        Self {
            dag,
            next_id,
            state: node_state,
        }
    }
}

#[derive(Debug)]
pub struct CFGDecisionNode {
    pub condition: Option<DAG>,
    pub true_branch_id: usize,
    pub false_branch_id: Option<usize>,
    pub state: CFGNodeState,
}

impl CFGDecisionNode {
    pub fn new(
        condition: Option<DAG>,
        true_branch_id: usize,
        false_branch_id: Option<usize>,
        state: CFGNodeState
    ) -> Self {
        Self {
            condition,
            true_branch_id,
            false_branch_id,
            state,
        }
    }
}
