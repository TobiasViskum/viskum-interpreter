mod fn_ptrs;
mod optimize_instructions;

use super::{ instructions::{ Instruction, Reg }, VM };
use fn_ptrs::{ binary_fn_ptrs::{ *, comparison_fn_ptrs::* }, unary_fn_ptrs::*, util_fn_ptrs::* };
pub use optimize_instructions::optimize_instructions;

#[derive(Clone, Copy)]
pub struct Void {}

pub union Param {
    const_idx: usize,
    register_offset: usize,
    reg_idx: usize,
    jmp: *const OptimizedInstruction,
    __: Void,
}

impl Param {
    pub fn void() -> Self {
        Self { __: Void {} }
    }

    pub fn reg(reg: usize) -> Self {
        Self {
            reg_idx: reg,
        }
    }
}

pub struct OptimizedInstruction {
    pub handler: unsafe fn(*const Self, &mut VM),
    pub param1: Param,
    pub param2: Param,
    pub param3: Param,
}

impl OptimizedInstruction {
    fn halt() -> Self {
        Self {
            handler: halt,
            param1: Param::void(),
            param2: Param::void(),
            param3: Param::void(),
        }
    }
}

macro_rules! make_unary_instruction {
    ($method_name:ident, $dst_reg:ident, $src_reg:ident) => {
        paste::paste! {
            Self {
                handler: match $src_reg {
                    Reg::Rel(_) => [<$method_name _r>],
                    Reg::Abs(_) => [<$method_name _a>],
                },
                param1: Param::reg($dst_reg),
                param2: Param::reg($src_reg.get_idx()),
                param3: Param::void(),
            }
        }
    };
}

macro_rules! make_binary_instruction {
    ($method_name:ident, $dst_reg:ident, $src1_reg:ident, $src2_reg:ident) => {
        paste::paste! {
            Self {
                handler: match ($src1_reg, $src2_reg) {
                    (Reg::Rel(_), Reg::Rel(_)) => [<$method_name _rr>],
                    (Reg::Rel(_), Reg::Abs(_)) => [<$method_name _ra>],
                    (Reg::Abs(_), Reg::Rel(_)) => [<$method_name _ar>],
                    (Reg::Abs(_), Reg::Abs(_)) => [<$method_name _aa>],
                },
                param1: Param::reg($dst_reg),
                param2: Param::reg($src1_reg.get_idx()),
                param3: Param::reg($src2_reg.get_idx()),
            }
        }
    };
}

impl From<&Instruction> for OptimizedInstruction {
    fn from(instruction: &Instruction) -> Self {
        match *instruction {
            Instruction::Halt => Self::halt(),
            Instruction::AddInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(add_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::SubInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(sub_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::MulInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(mul_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::DivInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(div_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::CmpEqInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(cmp_eq_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::CmpNeInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(cmp_ne_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::CmpGeInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(cmp_ge_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::CmpGtInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(cmp_gt_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::CmpLeInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(cmp_le_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::CmpLtInt { dst_reg, src1_reg, src2_reg } => {
                make_binary_instruction!(cmp_lt_int, dst_reg, src1_reg, src2_reg)
            }
            Instruction::NegInt { dst_reg, src_reg } => {
                make_unary_instruction!(neg_int, dst_reg, src_reg)
            }
            Instruction::NotInt { dst_reg, src_reg } => {
                make_unary_instruction!(not_int, dst_reg, src_reg)
            }
            Instruction::Copy { dst_reg, src_reg } => {
                make_unary_instruction!(copy, dst_reg, src_reg)
            }
        }
    }
}
