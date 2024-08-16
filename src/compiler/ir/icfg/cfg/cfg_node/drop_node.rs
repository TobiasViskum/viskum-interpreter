use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ CFGNodeTrait, Dissasemble, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGDropNode {}

impl CFGNodeTrait for CFGDropNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        todo!()
    }

    fn build_llvm(&self, func: &mut Function, llvm_builder: &mut LLVMBuilder) {
        unimplemented!()
    }
}

impl ParseConnectedNodes for CFGDropNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, _: Vec<usize>) -> Self::ConnectedNodes {
        ()
    }
}

impl Dissasemble for CFGDropNode {
    fn dissasemble(&self) -> String {
        "DROP".to_string()
    }
}
