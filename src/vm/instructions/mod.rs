mod helper_methods;
pub mod helper_structs;
use crate::value::Value;
use self::helper_structs::InstructionRegister;

#[derive(Debug, Clone)]
pub enum VMInstructionSrc {
    Register(InstructionRegister),
    Constant(Value),
}

impl VMInstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(value) => { format!("{}", value.dissassemble()) },
            Self::Constant(value) => { format!("{}", value.to_string()) },
        }
    }
}

#[derive(Debug, Clone)]
pub enum VMInstruction {
    Halt,
    StartScope,
    EndScope,
    Load {
        reg: InstructionRegister,
        src: VMInstructionSrc,
    },
    Add {
        dest: InstructionRegister,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Sub {
        dest: InstructionRegister,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Mul {
        dest: InstructionRegister,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Div {
        dest: InstructionRegister,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Neg {
        dest: InstructionRegister,
        src: VMInstructionSrc,
    },
    Truthy {
        dest: InstructionRegister,
        src: VMInstructionSrc,
    },
    Define {
        dest: InstructionRegister,
        src: VMInstructionSrc,
    },
    Assign {
        dest: InstructionRegister,
        src: VMInstructionSrc,
    },
}

impl VMInstruction {
    pub fn dissassemble(&self) -> String {
        match self {
           Self::Halt => { "HALT".to_string() }
           Self::StartScope => { "STARTSCOPE".to_string() }
           Self::EndScope => { "ENDSCOPE".to_string() }

            Self::Load { reg, src } => {
                format!("LOAD {} {}", reg.dissassemble(), src.dissassemble())
            }
            Self::Add { dest, src1, src2 } => {
                format!("ADD {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
            }
            Self::Sub { dest, src1, src2 } => {
                format!("SUB {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
            }
            Self::Mul { dest, src1, src2 } => {
                format!("MUL {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
            }
            Self::Div { dest, src1, src2 } => {
                format!("DIV {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
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

}

#[derive(Debug, Clone)]
pub enum IRInstructionSrc {
    Register(InstructionRegister),
    Constant(Value),
    VariableRegister(InstructionRegister),
}

impl IRInstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(value) => { format!("{}", value.dissassemble()) },
            Self::Constant(value) => { format!("{}", value.to_string()) },
            Self::VariableRegister(value) => { format!("{}", value.dissassemble()) },
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
    Halt,
    StartScope,
    EndScope,
    Load {
        reg: InstructionRegister,
        src: IRInstructionSrc,
    },
    Add {
        dest: InstructionRegister,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Sub {
        dest: InstructionRegister,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Mul {
        dest: InstructionRegister,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Div {
        dest: InstructionRegister,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Neg {
        dest: InstructionRegister,
        src: IRInstructionSrc,
    },
    Truthy {
        dest: InstructionRegister,
        src: IRInstructionSrc,
    },
    Define {
        dest: InstructionRegister,
        src: IRInstructionSrc,
    },
    Assign {
        dest: InstructionRegister,
        src: IRInstructionSrc,
    },
}

impl IRInstruction {
    pub fn dissassemble(&self) -> String {
        match self {
           Self::Halt => { "HALT".to_string() }
           Self::StartScope => { "STARTSCOPE".to_string() }
           Self::EndScope => { "ENDSCOPE".to_string() }

            Self::Load { reg, src } => {
                format!("LOAD {} {}", reg.dissassemble(), src.dissassemble())
            }
            Self::Add { dest, src1, src2 } => {
                format!("ADD {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
            }
            Self::Sub { dest, src1, src2 } => {
                format!("SUB {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
            }
            Self::Mul { dest, src1, src2 } => {
                format!("MUL {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
            }
            Self::Div { dest, src1, src2 } => {
                format!("DIV {} {} {}", dest.dissassemble(), src1.dissassemble(), src2.dissassemble())
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

}

