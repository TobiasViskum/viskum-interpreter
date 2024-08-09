use std::collections::{ BTreeSet, BinaryHeap };

use ahash::{ AHashMap, AHashSet };
use indexmap::IndexSet;

use crate::{ compiler::{ ir::icfg::dag::DAGNode, print_todo }, vm::instructions::Reg };

use super::{ symbol_table::SSAKey, value::SimpleConst };

#[derive(Debug)]
pub struct RegisterAllocator<'a> {
    register_offet: usize,
    used_regs: BTreeSet<usize>,
    allocated_const_registers: &'a AHashMap<SimpleConst, usize>,
    allocated_var_registers: AHashMap<SSAKey, Reg>,
}

impl<'a> RegisterAllocator<'a> {
    pub fn new(
        register_offet: usize,
        allocated_const_registers: &'a AHashMap<SimpleConst, usize>
    ) -> Self {
        Self {
            register_offet,
            used_regs: BTreeSet::new(),
            allocated_const_registers,
            allocated_var_registers: AHashMap::new(),
        }
    }

    pub fn free_from_dag_node(&mut self, dag_node: &DAGNode) {
        if let DAGNode::IdentNode(ident_node) = dag_node {
            let ident = ident_node.get_ssa_key();
            if let Some(removed_var_reg) = self.allocated_var_registers.remove(ident) {
                self.free_temp_reg(removed_var_reg);
            }
        }
    }

    pub fn alloc_var_reg(&mut self, ssa_key: SSAKey) -> usize {
        let allocated_var_reg = self.alloc_reg();

        self.allocated_var_registers.insert(
            ssa_key,
            Reg::Rel(allocated_var_reg + self.register_offet)
        );

        allocated_var_reg + self.register_offet
    }

    pub fn mark_reg_as_var(&mut self, ssa_key: SSAKey, reg: Reg) {
        self.allocated_var_registers.insert(ssa_key, reg);
    }

    pub fn alloc_temp_reg(&mut self) -> usize {
        self.alloc_reg() + self.register_offet
    }

    fn alloc_reg(&mut self) -> usize {
        let available_reg = self.used_regs
            .iter()
            .enumerate()
            .position(|(i, &reg)| i != reg)
            .unwrap_or_else(|| self.used_regs.len());

        self.used_regs.insert(available_reg);

        available_reg
    }

    pub fn free_temp_reg(&mut self, reg: Reg) {
        match reg {
            Reg::Rel(reg_idx) => {
                if
                    self.allocated_var_registers
                        .iter()
                        .filter(|&(_, val_reg)| *val_reg == reg)
                        .count() == 0
                {
                    self.used_regs.remove(&(reg_idx - self.register_offet));
                }
            }
            Reg::Abs(_) => {}
        }
    }

    pub fn get_var_reg(&self, ident_ssa: &SSAKey) -> Reg {
        let reg = self.allocated_var_registers
            .get(&ident_ssa)
            .copied()
            .expect("Expected ssa_key in allocated_var_registers, but it was not found");

        reg
    }

    pub fn get_const_reg(&self, simple_const: SimpleConst) -> Reg {
        let abs_reg = self.allocated_const_registers
            .get(&simple_const)
            .copied()
            .expect("Failed to get SimpleConst (get_const_reg). It hasn't been loaded");
        Reg::Abs(abs_reg)
    }
}
