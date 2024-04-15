use super::{ instructions::Instruction, VM };

impl VM {
    pub(super) fn get_instruction(&self) -> &Instruction {
        &self.program[self.pc]
    }
}
