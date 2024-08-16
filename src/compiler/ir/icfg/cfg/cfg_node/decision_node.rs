use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        ir::icfg::dag::DAG,
        traits::{ CFGNodeTrait, Dissasemble, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGDecisionNode {
    condition: DAG,
}

impl CFGNodeTrait for CFGDecisionNode {
    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = vec![];
        self.condition.generate_instructions_as_condition(&mut instructions, register_allocator);
        instructions
    }

    fn build_llvm(&self, func: &mut Function, llvm_builder: &mut LLVMBuilder) {
        unimplemented!()
    }
}

impl CFGDecisionNode {
    pub fn new(condition: DAG) -> Self {
        Self { condition }
    }

    pub fn get_condition(&self) -> &DAG {
        &self.condition
    }
}

impl ParseConnectedNodes for CFGDecisionNode {
    type ConnectedNodes = (usize, usize);

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        if connected_nodes.len() != 2 {
            panic!(
                "Expected two nodes connected to CFGDecisionNode, found {}",
                connected_nodes.len()
            );
        }
        (connected_nodes[0], connected_nodes[1])
    }
}

impl Dissasemble for CFGDecisionNode {
    fn dissasemble(&self) -> String {
        self.condition.dissasemble()
    }
}
