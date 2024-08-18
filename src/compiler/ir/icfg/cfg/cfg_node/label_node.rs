use llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        ir::icfg::{ cfg::CFG, icfg_builder::CFGBuilder },
        traits::{ AllocLLVM, CFGNodeGenerateLLVM, CFGNodeTrait, GenerateLLVM, ParseConnectedNodes },
        Dissasemble,
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGLabelNode;

impl AllocLLVM for CFGLabelNode {
    fn alloc_llvm(
        &self,
        _llvm_builder: &mut LLVMBuilder,
        _module: &mut llvm_builder::Module,
        _func: &mut Function
    ) {}
}

impl CFGNodeGenerateLLVM for CFGLabelNode {
    fn build_llvm(&self, node_id: usize, _: &mut LLVMBuilder, func: &mut Function, _: &CFG) {
        func.add_instr(format!("lbl{}:", node_id))
    }
}

impl CFGNodeTrait for CFGLabelNode {
    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        vec![]
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
