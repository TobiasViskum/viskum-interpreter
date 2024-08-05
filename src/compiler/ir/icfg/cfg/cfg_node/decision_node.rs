use crate::compiler::{ ir::icfg::dag::DAG, traits::Dissasemble };

#[derive(Debug)]
pub struct CFGDecisionNode {
    condition: Option<DAG>,
}

impl CFGDecisionNode {
    pub fn new(condition: Option<DAG>) -> Self {
        Self { condition }
    }
}

impl Dissasemble for CFGDecisionNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
