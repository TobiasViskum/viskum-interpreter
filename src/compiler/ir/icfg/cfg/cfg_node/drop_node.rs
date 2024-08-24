use crate::compiler::llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ AllocLLVM, CFGNodeTrait, Dissasemble, GenerateLLVM, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGDropNode {}

impl AllocLLVM for CFGDropNode {
    fn alloc_llvm(
        &self,
        llvm_builder: &mut LLVMBuilder,
        module: &mut crate::compiler::llvm_builder::Module,
        func: &mut Function
    ) {
        unimplemented!()
    }
}

impl GenerateLLVM for CFGDropNode {
    fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        unimplemented!()
    }
}

impl CFGNodeTrait for CFGDropNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        todo!()
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
