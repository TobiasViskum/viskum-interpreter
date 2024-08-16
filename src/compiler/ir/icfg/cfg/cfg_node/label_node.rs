use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ CFGNodeTrait, ParseConnectedNodes },
        Dissasemble,
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGLabelNode;

impl CFGNodeTrait for CFGLabelNode {
    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        vec![]
    }

    fn build_llvm(&self, func: &mut Function, llvm_builder: &mut LLVMBuilder) {
        unimplemented!()
    }
}

impl ParseConnectedNodes for CFGLabelNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {}
}

impl Dissasemble for CFGLabelNode {
    fn dissasemble(&self) -> String {
        format!("Label:")
    }
}
