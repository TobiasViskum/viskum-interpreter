use crate::{ BuildLLVM, TypeI32 };

use super::{ llvm_operand::Operand, llvm_var::Var };

pub enum Instruction {
    AddI32 {
        dst_var: Var,
        op1: Operand,
        op2: Operand,
    },
    RetI32 {
        op: Operand,
    },
}

impl BuildLLVM for Instruction {
    fn build(&self) -> String {
        match self {
            Self::AddI32 { dst_var, op1, op2 } => {
                format!("{} = add i32 {}, {}", dst_var.build(), op1.build(), op2.build())
            }
            Self::RetI32 { op } => { format!("ret i32 {}", op.build()) }
        }
    }
}
