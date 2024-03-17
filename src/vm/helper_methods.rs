use crate::value::Value;

use super::{ instructions::VMInstruction, VM };

impl VM {
    pub(super) fn get_instruction(&self) -> &VMInstruction {
        &self.program[self.pc]
    }
}
