use crate::compiler::Dissasemble;

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    Abs(usize),
    Rel(usize),
}

impl Reg {
    pub(super) fn get_idx(&self) -> usize {
        match self {
            Self::Abs(reg) => *reg,
            Self::Rel(reg) => *reg,
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Halt,
    AddInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    SubInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    MulInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    DivInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    CmpEqInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    CmpNeInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    CmpGeInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    CmpGtInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    CmpLeInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    CmpLtInt {
        dst_reg: usize,
        src1_reg: Reg,
        src2_reg: Reg,
    },
    NegInt {
        dst_reg: usize,
        src_reg: Reg,
    },
    NotInt {
        dst_reg: usize,
        src_reg: Reg,
    },
}

fn instr_name(name: &str) -> String {
    let max_name_len = 13;
    format!("{}{}", name, " ".repeat(max_name_len - name.len()))
}

fn arg_reg(reg: &Reg) -> String {
    let max_arg_len = 4;
    format!("R{}{}", reg.get_idx(), " ".repeat(max_arg_len - reg.get_idx().to_string().len()))
}

fn arg_const(const_pos: &usize) -> String {
    let max_arg_len = 3;
    format!("[{}]{}", const_pos, " ".repeat(max_arg_len - const_pos.to_string().len()))
}

impl Instruction {
    pub fn to_op_name(&self) -> String {
        String::from(match self {
            Self::Halt => "HALT",
            Self::AddInt { .. } => "ADD_INT",
            Self::SubInt { .. } => "SUB_INT",
            Self::MulInt { .. } => "MUL_INT",
            Self::DivInt { .. } => "DIV_INT",
            Self::CmpEqInt { .. } => "CMP_EQ_INT",
            Self::CmpNeInt { .. } => "CMP_NE_INT",
            Self::CmpGeInt { .. } => "CMP_GE_INT",
            Self::CmpGtInt { .. } => "CMP_GT_INT",
            Self::CmpLeInt { .. } => "CMP_LE_INT",
            Self::CmpLtInt { .. } => "CMP_LT_INT",
            Self::NegInt { .. } => "NEG_INT",
            Self::NotInt { .. } => "NOT_INT",
        })
    }
}

impl Dissasemble for Instruction {
    fn dissasemble(&self) -> String {
        match self {
            Self::Halt => "HALT".to_string(),
            | Self::AddInt { dst_reg, src1_reg, src2_reg }
            | Self::SubInt { dst_reg, src1_reg, src2_reg }
            | Self::MulInt { dst_reg, src1_reg, src2_reg }
            | Self::DivInt { dst_reg, src1_reg, src2_reg }
            | Self::CmpEqInt { dst_reg, src1_reg, src2_reg }
            | Self::CmpNeInt { dst_reg, src1_reg, src2_reg }
            | Self::CmpGeInt { dst_reg, src1_reg, src2_reg }
            | Self::CmpGtInt { dst_reg, src1_reg, src2_reg }
            | Self::CmpLeInt { dst_reg, src1_reg, src2_reg }
            | Self::CmpLtInt { dst_reg, src1_reg, src2_reg } => {
                format!(
                    "{} {} {} {}",
                    instr_name(self.to_op_name().as_str()),
                    arg_reg(&Reg::Rel(*dst_reg)),
                    arg_reg(src1_reg),
                    arg_reg(src2_reg)
                )
            }
            Self::NegInt { dst_reg, src_reg } | Self::NotInt { dst_reg, src_reg } => {
                format!(
                    "{} {} {}",
                    instr_name(self.to_op_name().as_str()),
                    arg_reg(&Reg::Rel(*dst_reg)),
                    arg_reg(src_reg)
                )
            }
        }
    }
}
