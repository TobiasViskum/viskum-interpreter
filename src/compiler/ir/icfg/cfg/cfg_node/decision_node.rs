use crate::compiler::llvm_builder::{ Function, LLVMBuilder };

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        ir::icfg::{ cfg::CFG, dag::DAG },
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

use super::CFGLabelNode;

#[derive(Debug)]
pub struct CFGDecisionNode {
    condition: DAG,
}

impl AllocLLVM for CFGDecisionNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        self.condition.alloc_llvm(llvm_builder, func)
    }
}

impl CFGNodeGenerateLLVM for CFGDecisionNode {
    fn build_llvm(
        &self,
        node_id: usize,
        llvm_builder: &mut LLVMBuilder,
        func: &mut Function,
        cfg: &CFG
    ) {
        let connected_nodes = self.parse_connected_nodes(cfg.get_connected_nodes(node_id));

        self.condition.build_llvm(llvm_builder, func);

        let cmp_ssa_key = llvm_builder.get_latest_ssa_key();

        func.add_instr(
            format!(
                "br i1 %v{}, label %lbl{}, label %lbl{}",
                cmp_ssa_key,
                connected_nodes.0,
                connected_nodes.1
            )
        );
    }
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
