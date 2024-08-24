pub mod cfg;
pub mod dag;
pub mod icfg_builder;

use std::fmt::Debug;

use cfg::CFG;
pub use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module };

use crate::{
    compiler::{
        ds::{ register_allocator::RegisterAllocator, value::ValueType, vm_builder::VMBuilder },
        llvm_builder::{ self, BuildLLVM, Type },
        traits::{ AllocLLVM, Dissasemble, GenerateLLVM, LoadConstants },
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

    pub fn get_cfg(&self, cfg_id: usize) -> &CFG {
        self.cfgs.get(cfg_id).expect("Expected cfg, got none")
    }

    pub fn set_entry_cfg(&mut self, entry_id: CFGId) {
        self.entry_cfg = entry_id;
    }

    fn get_main_fn_id(&self) -> usize {
        self.cfgs
            .iter()
            .position(|cfg| cfg.get_ssa_name().is("main", 0))
            .unwrap()
    }

    fn get_global_fn_id(&self) -> usize {
        self.cfgs
            .iter()
            .position(|cfg| cfg.get_ssa_name().is("global", 0))
            .unwrap()
    }

    pub fn build_llvm(&self) -> LLVMBuilder {
        let mut llvm_builder = LLVMBuilder::new(self);

        let mut module = Module::new();

        let main_fn_id = self.get_main_fn_id();
        let global_scope_id = self.get_global_fn_id();

        for i in 0..self.cfgs.len() {
            let cfg = &self.cfgs[i];

            if i == main_fn_id || i == global_scope_id {
                continue;
            }

            let mut func = Function::new(
                cfg.get_ssa_name().clone(),
                cfg.get_ret_type().to_llvm_type()
            );
            cfg.alloc_llvm(&mut llvm_builder, &mut module, &mut func);
            cfg.build_llvm(&mut llvm_builder, &mut func);
            match cfg.get_ret_type() {
                ValueType::Void => func.add_instr("ret void".to_string()),
                _ => {}
            }

            module.push_func(func);
        }

        let mut func = Function::new(self.cfgs[main_fn_id].get_ssa_name().clone(), Type::I32);
        self.cfgs[main_fn_id].alloc_llvm(&mut llvm_builder, &mut module, &mut func);
        self.cfgs[main_fn_id].build_llvm(&mut llvm_builder, &mut func);
        func.add_instr("ret i32 0".to_string());

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
