use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ AllocLLVM, CFGNodeTrait, Dissasemble, GenerateLLVM, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGReturnNode;

impl AllocLLVM for CFGReturnNode {
    fn alloc_llvm(
        &self,
        llvm_builder: &mut LLVMBuilder,
        module: &mut llvm_builder::Module,
        func: &mut Function
    ) {
        unimplemented!()
    }
}

impl GenerateLLVM for CFGReturnNode {
    fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        unimplemented!()
    }
}

impl CFGNodeTrait for CFGReturnNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        todo!()
    }
}

impl ParseConnectedNodes for CFGReturnNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, _connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        todo!()
    }
}

impl Dissasemble for CFGReturnNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
