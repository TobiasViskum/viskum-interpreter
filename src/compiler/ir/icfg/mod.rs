pub mod cfg;
pub mod dag;
pub mod icfg_builder;

use cfg::CFG;

use crate::{
    compiler::{
        ds::{ register_allocator::RegisterAllocator, vm_builder::VMBuilder },
        traits::{ Dissasemble, GenerateBytecode },
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
}

impl GenerateBytecode for ICFG {
    fn load_constants(&self, vm_builder: &mut VMBuilder) {
        self.cfgs[0].load_constants(vm_builder)
    }

    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = self.cfgs[0].generate_instructions(register_allocator);
        instructions.push(Instruction::Halt);
        instructions
    }
}

impl Dissasemble for ICFG {
    fn dissasemble(&self) -> String {
        self.cfgs[self.entry_cfg].dissasemble()
    }
}
