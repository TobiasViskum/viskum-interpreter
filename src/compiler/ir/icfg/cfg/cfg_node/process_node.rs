use crate::compiler::{ ir::icfg::dag::DAG, traits::Dissasemble };

#[derive(Debug)]
pub struct CFGProcessNode {
    dag: DAG,
}

impl CFGProcessNode {
    pub fn new(dag: DAG) -> Self {
        Self { dag }
    }

    pub fn get_mut_dag(&mut self) -> &mut DAG {
        &mut self.dag
    }

    pub fn get_dag(&self) -> &DAG {
        &self.dag
    }
}

impl Dissasemble for CFGProcessNode {
    fn dissasemble(&self) -> String {
        self.dag.dissasemble()
    }
}
