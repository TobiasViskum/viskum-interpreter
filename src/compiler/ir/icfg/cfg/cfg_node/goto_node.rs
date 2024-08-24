use crate::compiler::llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        ir::icfg::cfg::CFG,
        traits::{
            AllocLLVM,
            CFGNodeGenerateLLVM,
            CFGNodeTrait,
            Dissasemble,
            GenerateLLVM,
            ParseConnectedNodes,
        },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGGotoNode;

impl AllocLLVM for CFGGotoNode {
    fn alloc_llvm(
        &self,
        llvm_builder: &mut LLVMBuilder,
        module: &mut crate::compiler::llvm_builder::Module,
        func: &mut Function
    ) {}
}

impl CFGNodeGenerateLLVM for CFGGotoNode {
    fn build_llvm(
        &self,
        node_id: usize,
        llvm_builder: &mut LLVMBuilder,
        func: &mut Function,
        cfg: &CFG
    ) {
        let connected_node = self.parse_connected_nodes(cfg.get_connected_nodes(node_id));

        func.add_instr(format!("br label %lbl{}", connected_node))
    }
}

impl CFGNodeTrait for CFGGotoNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        vec![Instruction::Goto { jmp_pos: 0 }]
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
