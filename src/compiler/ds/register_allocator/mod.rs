use ahash::{ AHashMap, AHashSet };

use crate::{ compiler::print_todo, vm::instructions::Reg };

use super::{ symbol_table::SSAKey, value::SimpleConst };

#[derive(Debug)]
pub struct RegisterAllocator<'a> {
    register_offet: usize,
    used_regs: AHashSet<usize>,
    allocated_const_registers: &'a AHashMap<SimpleConst, usize>,
    allocated_var_registers: AHashMap<SSAKey, usize>,
}

impl<'a> RegisterAllocator<'a> {
    pub fn new(
        register_offet: usize,
        allocated_const_registers: &'a AHashMap<SimpleConst, usize>
    ) -> Self {
        Self {
            register_offet,
            used_regs: AHashSet::new(),
            allocated_const_registers,
            allocated_var_registers: AHashMap::new(),
        }
    }

    // pub fn alloc_var(&mut self) -> Reg {}

    pub fn alloc_reg(&mut self) -> usize {
        let available_reg = self.used_regs
            .iter()
            .max()
            .map_or_else(
                || 0,
                |x| x + 1
            );
        self.used_regs.insert(available_reg);
        available_reg + self.register_offet
    }

    pub fn free_temp_reg(&mut self, reg: Reg) {
        match reg {
            Reg::Rel(reg) => {
                print_todo(
                    "Also check if it's a \"variable\" reg. If it is, it should not be removed"
                );
                self.used_regs.remove(&(reg - self.register_offet));
            }
            Reg::Abs(_) => {}
        }
    }

    pub fn get_const_reg(&self, simple_const: SimpleConst) -> Reg {
        let abs_reg = self.allocated_const_registers
            .get(&simple_const)
            .copied()
            .expect("Failed to get SimpleConst (get_const_reg). It hasn't been loaded");
        Reg::Abs(abs_reg)
    }
}
