use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        ir::icfg::dag::DAG,
        traits::{ AllocLLVM, CFGNodeTrait, Dissasemble, GenerateLLVM, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGProcessNode {
    dag: DAG,
}

impl AllocLLVM for CFGProcessNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        self.dag.alloc_llvm(llvm_builder, func)
    }
}

impl GenerateLLVM for CFGProcessNode {
    fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        self.dag.build_llvm(llvm_builder, func);
    }
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
