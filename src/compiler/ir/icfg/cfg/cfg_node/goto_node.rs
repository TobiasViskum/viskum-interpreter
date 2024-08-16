use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ CFGNodeTrait, Dissasemble, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGGotoNode;

impl CFGNodeTrait for CFGGotoNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        vec![Instruction::Goto { jmp_pos: 0 }]
    }

    fn build_llvm(&self, func: &mut Function, llvm_builder: &mut LLVMBuilder) {
        unimplemented!()
    }
}

impl ParseConnectedNodes for CFGGotoNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        *connected_nodes.get(0).expect("Expected one node connected to CFGGotoNode")
    }
}

impl Dissasemble for CFGGotoNode {
    fn dissasemble(&self) -> String {
        "GOTO".to_string()
    }
}
