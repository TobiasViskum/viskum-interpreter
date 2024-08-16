use crate::{ BuildLLVM, TypeI32 };

use super::{ llvm_instruction::Instruction, llvm_operand::Operand, llvm_type::Type, llvm_var::Var };

pub struct Function {
    name: String,
    ret_type: Type,
    instructions: Vec<String>,
}

impl Function {
    pub fn new(name: String, ret_type: Type) -> Self {
        Self { name, ret_type, instructions: vec![] }
    }

    pub fn add_instr(&mut self, instr: String) {
        self.instructions.push(format!("    {}", instr))
    }
}

impl BuildLLVM for Function {
    fn build(&self) -> String {
        let mut string_builder = format!("define {} @{}() {{\n", self.ret_type.build(), self.name);
        for instr in self.instructions.iter() {
            string_builder += instr;
            string_builder += "\n";
        }
        string_builder += "}\n";
        string_builder
    }
}
