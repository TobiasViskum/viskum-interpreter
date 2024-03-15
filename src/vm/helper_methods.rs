use crate::value::Value;

use super::{ instructions::VMInstruction, VM };

impl VM {
    pub fn _get_register(&self, register: usize) -> &Value {
        &self.registers[register]
    }

    pub(super) fn get_instruction(&self) -> &VMInstruction {
        &self.program[self.pc]
    }
}
