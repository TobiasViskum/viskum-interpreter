use crate::{ BuildLLVM, LLVMType, TypeI32 };

use super::{ llvm_type::RawOperand, llvm_var::Var };

pub enum Operand {
    Var(Var),
    Raw(RawOperand),
}

impl BuildLLVM for Operand {
    fn build(&self) -> String {
        match self {
            Self::Var(var) => var.build(),
            Self::Raw(i32) => i32.build(),
        }
    }
}
