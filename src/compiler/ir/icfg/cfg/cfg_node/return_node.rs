use crate::compiler::ds::value::ValueType;
use crate::compiler::ir::icfg::cfg::CFG;
use crate::compiler::ir::icfg::dag::DAG;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module };

use crate::compiler::print_todo;
use crate::compiler::traits::CFGNodeGenerateLLVM;
use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ AllocLLVM, CFGNodeTrait, Dissasemble, GenerateLLVM, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGReturnNode {
    ret_val: Option<DAG>,
    ret_type: ValueType,
}

impl CFGReturnNode {
    pub fn new(ret_val: Option<DAG>, ret_type: ValueType) -> Self {
        print_todo("ret_val has to be a cfg node(s) when making expression based");
        Self { ret_val, ret_type }
    }

    pub fn get_ret_val(&self) -> Option<&DAG> {
        self.ret_val.as_ref()
    }
}

impl AllocLLVM for CFGReturnNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        self.ret_val.as_ref().map(|dag| dag.alloc_llvm(llvm_builder, func));
    }
}

impl CFGNodeGenerateLLVM for CFGReturnNode {
    fn build_llvm(
        &self,
        node_id: usize,
        llvm_builder: &mut LLVMBuilder,
        func: &mut Function,
        cfg: &CFG
    ) {
        let ret_key = self.ret_val.as_ref().map(|dag| dag.build_llvm(llvm_builder, func));
        if let Some(ret_key) = ret_key {
            func.add_instr(
                format!("ret {} {}", self.ret_type.to_llvm_type().build(), ret_key.build())
            )
        } else {
            func.add_instr(format!("ret {}", self.ret_type.to_llvm_type().build()))
        }
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
        "RET".to_string()
    }
}
