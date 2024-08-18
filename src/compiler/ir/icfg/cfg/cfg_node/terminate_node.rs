use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ AllocLLVM, CFGNodeTrait, Dissasemble, GenerateLLVM, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug, PartialEq)]
pub enum TerminationState {
    Start,
    End,
}

#[derive(Debug)]
pub struct CFGTerminateNode {
    termination_state: TerminationState,
}

impl AllocLLVM for CFGTerminateNode {
    fn alloc_llvm(
        &self,
        _llvm_builder: &mut LLVMBuilder,
        _module: &mut llvm_builder::Module,
        _func: &mut Function
    ) {}
}

impl GenerateLLVM for CFGTerminateNode {
    fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        unimplemented!()
    }
}

impl CFGNodeTrait for CFGTerminateNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        if !self.is_start() { vec![Instruction::Halt] } else { vec![] }
    }
}

impl CFGTerminateNode {
    pub fn new_start() -> Self {
        Self { termination_state: TerminationState::Start }
    }

    pub fn new_end() -> Self {
        Self { termination_state: TerminationState::End }
    }

    pub fn is_start(&self) -> bool {
        self.termination_state == TerminationState::Start
    }
}

impl ParseConnectedNodes for CFGTerminateNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        *connected_nodes.get(0).expect("Expected one node connected to CFGTerminateNode")
    }
}

impl Dissasemble for CFGTerminateNode {
    fn dissasemble(&self) -> String {
        "".to_string()
    }
}
