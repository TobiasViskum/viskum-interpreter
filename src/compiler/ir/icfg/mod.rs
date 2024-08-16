pub mod cfg;
pub mod dag;
pub mod icfg_builder;

use ahash::AHashMap;
use cfg::{ CFGNodeType, CFG };
use llvm_builder::{ Function, LLVMBuilder, Module };

use crate::{
    compiler::{
        ds::{ register_allocator::RegisterAllocator, vm_builder::VMBuilder },
        traits::{ Dissasemble, LoadConstants, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

pub type CFGId = usize;

#[derive(Debug)]
pub struct ICFG {
    cfgs: Vec<CFG>,
    entry_cfg: usize,
}

impl ICFG {
    pub fn new() -> Self {
        Self {
            cfgs: Vec::new(),
            entry_cfg: 0,
        }
    }

    pub fn print(&self) {
        println!("{}", "\n----- ICFG -----\n");
        println!("{}", self.dissasemble());
        println!("{}", "\n---------------\n")
    }

    pub fn push_cfg(&mut self, cfg: CFG) -> CFGId {
        self.cfgs.push(cfg);
        self.cfgs.len() - 1
    }

    pub fn set_entry_cfg(&mut self, entry_id: CFGId) {
        self.entry_cfg = entry_id;
    }

    pub fn build_llvm(&self) -> LLVMBuilder {
        let mut llvm_builder = llvm_builder::LLVMBuilder::new();

        let mut func = Function::new("main".to_string(), llvm_builder::Type::I32);
        self.cfgs[0].build_llvm(&mut llvm_builder, &mut func);
        func.add_instr("ret i32 0".to_string());
        let mut module = Module::new();
        module.push_func(func);
        llvm_builder.push_mod(module);
        llvm_builder
    }

    pub fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let instructions = self.cfgs[0].generate_instructions(register_allocator);
        instructions
    }
}

impl LoadConstants for ICFG {
    fn load_constants(&self, vm_builder: &mut VMBuilder) {
        self.cfgs[0].load_constants(vm_builder)
    }
}

impl Dissasemble for ICFG {
    fn dissasemble(&self) -> String {
        self.cfgs[self.entry_cfg].dissasemble()
    }
}
