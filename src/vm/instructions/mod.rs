use crate::{ operations::{ BinaryOp, UnaryOp }, value_v2::Value };

#[derive(Debug, Clone, Copy)]
pub struct InstructionRegister {
    pub register: usize,
    pub scope: usize,
    pub is_variable: bool,
}

impl InstructionRegister {
    pub fn new(register: usize, scope: usize, is_variable: bool) -> Self {
        Self { register, scope, is_variable }
    }

    pub fn dissassemble(&self) -> String {
        format!("S{}:R{}", self.scope, self.register)
    }
}

#[derive(Debug, Clone)]
pub enum InstructionSrc {
    Register(InstructionRegister),
    Constant(Value),
}

impl InstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(value) => { format!("{}", value.dissassemble()) }
            Self::Constant(value) => { format!("{}", value.to_string()) }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Halt,
    StartScope,
    EndScope,
    Load {
        reg: InstructionRegister,
        src: InstructionSrc,
    },
    Add {
        dest: InstructionRegister,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Sub {
        dest: InstructionRegister,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Mul {
        dest: InstructionRegister,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Div {
        dest: InstructionRegister,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Neg {
        dest: InstructionRegister,
        src: InstructionSrc,
    },
    Truthy {
        dest: InstructionRegister,
        src: InstructionSrc,
    },
    Define {
        dest: InstructionRegister,
        src: InstructionSrc,
    },
    Assign {
        dest: InstructionRegister,
        src: InstructionSrc,
    },
}

impl Instruction {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Halt => { "HALT".to_string() }
            Self::StartScope => { "STARTSCOPE".to_string() }
            Self::EndScope => { "ENDSCOPE".to_string() }

            Self::Load { reg, src } => {
                format!("LOAD {} {}", reg.dissassemble(), src.dissassemble())
            }
            Self::Add { dest, src1, src2 } => {
                format!(
                    "ADD {} {} {}",
                    dest.dissassemble(),
                    src1.dissassemble(),
                    src2.dissassemble()
                )
            }
            Self::Sub { dest, src1, src2 } => {
                format!(
                    "SUB {} {} {}",
                    dest.dissassemble(),
                    src1.dissassemble(),
                    src2.dissassemble()
                )
            }
            Self::Mul { dest, src1, src2 } => {
                format!(
                    "MUL {} {} {}",
                    dest.dissassemble(),
                    src1.dissassemble(),
                    src2.dissassemble()
                )
            }
            Self::Div { dest, src1, src2 } => {
                format!(
                    "DIV {} {} {}",
                    dest.dissassemble(),
                    src1.dissassemble(),
                    src2.dissassemble()
                )
            }
            Self::Neg { dest, src } => {
                format!("NEG {} {}", dest.dissassemble(), src.dissassemble())
            }
            Self::Truthy { dest, src } => {
                format!("TRUTHY {} {}", dest.dissassemble(), src.dissassemble())
            }
            Self::Define { dest, src } => {
                format!("DEFINE {} {}", dest.dissassemble(), src.dissassemble())
            }
            Self::Assign { dest, src } => {
                format!("ASSIGN {} {}", dest.dissassemble(), src.dissassemble())
            }
        }
    }

    pub fn new_binary(
        operation: &BinaryOp,
        dest: InstructionRegister,
        src1: InstructionSrc,
        src2: InstructionSrc
    ) -> Self {
        match operation {
            BinaryOp::Add => Self::Add { dest, src1, src2 },
            BinaryOp::Sub => Self::Sub { dest, src1, src2 },
            BinaryOp::Mul => Self::Mul { dest, src1, src2 },
            BinaryOp::Div => Self::Div { dest, src1, src2 },
        }
    }

    pub fn new_define(dest: InstructionRegister, src: InstructionSrc) -> Self {
        Self::Define { dest, src }
    }

    pub fn new_assign(dest: InstructionRegister, src: InstructionSrc) -> Self {
        Self::Assign { dest, src }
    }

    pub fn new_unary(operation: &UnaryOp, dest: InstructionRegister, src: InstructionSrc) -> Self {
        match operation {
            UnaryOp::Neg => Self::Neg { dest, src },
            UnaryOp::Truthy => Self::Truthy { dest, src },
        }
    }

    pub fn new_load(reg: InstructionRegister, src: InstructionSrc) -> Self {
        Self::Load { reg, src }
    }
}
