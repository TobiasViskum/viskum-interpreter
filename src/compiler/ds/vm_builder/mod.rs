use ahash::{ AHashMap, AHashSet };

use crate::{ vm::Heap, U16_MAX, U8_MAX };

use super::{
    register_allocator::RegisterAllocator,
    ssa_ident::SSAIdent,
    value::{ ComplexConst, SimpleConst },
};

#[derive(Debug)]
pub struct VMBuilder {
    allocated_const_registers: AHashMap<SimpleConst, usize>,
    allocated_var_registers: AHashMap<SSAIdent, usize>,
    heap: Heap,
}

impl VMBuilder {
    pub fn new() -> Self {
        Self {
            allocated_const_registers: AHashMap::new(),
            allocated_var_registers: AHashMap::new(),
            heap: Heap::new(),
        }
    }

    pub fn take_registers(&self) -> [i64; U8_MAX] {
        let mut registers = [-2937i64; U8_MAX];
        for (simple_const, const_reg) in self.allocated_const_registers.iter() {
            registers[*const_reg] = simple_const.get_as_i64();
        }
        registers
    }

    pub fn get_register_allocator(&self) -> RegisterAllocator {
        RegisterAllocator::new(
            self.allocated_const_registers.len(),
            &self.allocated_const_registers
        )
    }

    pub fn alloc_const_reg(&mut self, simple_const: SimpleConst) -> usize {
        if let Some(const_reg) = self.allocated_const_registers.get(&simple_const) {
            *const_reg
        } else {
            let const_reg = self.allocated_const_registers.len();
            self.allocated_const_registers.insert(simple_const, const_reg);
            const_reg
        }
    }

    pub fn alloc_var_reg(&mut self, ssa_key: SSAIdent) -> usize {
        todo!()
    }

    // pub fn get_available_reg

    pub fn alloc_heap(&mut self, complex_const: ComplexConst) -> usize {
        // self.heap.insert(idx, heap_value);
        todo!()
    }
}
