use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        ir::icfg::dag::DAG,
        traits::{ CFGNodeTrait, Dissasemble, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGProcessNode {
    dag: DAG,
}

impl CFGNodeTrait for CFGProcessNode {
    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = vec![];
        self.dag.generate_instructions(&mut instructions, register_allocator);
        instructions
    }
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

impl ParseConnectedNodes for CFGProcessNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        if connected_nodes.len() != 1 {
            panic!(
                "Expected one node connected to CFGProcessNode, but found {}",
                connected_nodes.len()
            );
        }

        connected_nodes[0]
    }
}

impl Dissasemble for CFGProcessNode {
    fn dissasemble(&self) -> String {
        self.dag.dissasemble()
    }
}
