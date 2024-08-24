use crate::compiler::ds::ssa_ident::SSAIdent;

use super::{
    llvm_instruction::Instruction,
    llvm_operand::Operand,
    llvm_type::Type,
    llvm_var::Var,
    BuildLLVM,
};

pub struct Function {
    ssa_name: SSAIdent,
    ret_type: Type,
    instructions: Vec<String>,
}

impl Function {
    pub fn new(ssa_name: SSAIdent, ret_type: Type) -> Self {
        Self { ssa_name, ret_type, instructions: vec![] }
    }

    pub fn new_main() -> Self {
        Self {
            ssa_name: SSAIdent::new("main".into(), 0),
            ret_type: Type::I32,
            instructions: vec![],
        }
    }

    pub fn add_instr(&mut self, instr: String) {
        self.instructions.push(format!("    {}", instr))
    }
}

impl BuildLLVM for Function {
    fn build(&self) -> String {
        let mut string_builder = format!(
            "define {} @{}() {{\n",
            self.ret_type.build(),
            self.ssa_name.build()
        );
        for instr in self.instructions.iter() {
            string_builder += instr;
            string_builder += "\n";
        }
        string_builder += "}\n";
        string_builder
    }
}
