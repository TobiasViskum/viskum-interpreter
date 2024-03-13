use crate::value::Value;

use super::{ instructions::Instruction, VM };

impl VM {
    pub fn get_register(&self, register: usize) -> &Value {
        &self.registers[register]
    }

    pub(super) fn get_instruction(&self) -> &Instruction {
        &self.program[self.pc]
    }
}
