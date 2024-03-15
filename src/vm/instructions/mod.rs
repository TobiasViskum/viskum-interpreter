mod helper_methods;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum VMInstructionSrc {
    Register(usize),
    Constant(Value),
}

impl VMInstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(value) => { format!("R{}", value) },
            Self::Constant(value) => { format!("{}", value.to_string()) },
        }
    }
}

#[derive(Debug, Clone)]
pub enum VMInstruction {
    Halt,
    Load {
        reg: usize,
        src: VMInstructionSrc,
    },
    Add {
        dest: usize,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Sub {
        dest: usize,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Mul {
        dest: usize,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Div {
        dest: usize,
        src1: VMInstructionSrc,
        src2: VMInstructionSrc,
    },
    Neg {
        dest: usize,
        src: VMInstructionSrc,
    },
    Truthy {
        dest: usize,
        src: VMInstructionSrc,
    },
    Define {
        dest: usize,
        src: VMInstructionSrc,
    },
    Assign {
        dest: usize,
        src: VMInstructionSrc,
    },
}

impl VMInstruction {
    pub fn dissassemble(&self) -> String {
        match self {
           Self::Halt => { "HALT".to_string() }

            Self::Load { reg, src } => {
                format!("LOAD R{} {}", reg, src.dissassemble())
            }
            Self::Add { dest, src1, src2 } => {
                format!("ADD R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Sub { dest, src1, src2 } => {
                format!("SUB R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Mul { dest, src1, src2 } => {
                format!("MUL R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Div { dest, src1, src2 } => {
                format!("DIV R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Neg { dest, src } => {
                format!("NEG R{} {}", dest, src.dissassemble())
            }
            Self::Truthy { dest, src } => {
                format!("TRUTHY R{} {}", dest, src.dissassemble())
            }
            Self::Define { dest, src } => {
                format!("DEFINE R{} {}", dest, src.dissassemble())
            }
            Self::Assign { dest, src } => {
                format!("ASSIGN R{} {}", dest, src.dissassemble())
            }
        }
    }

}

#[derive(Debug, Clone)]
pub enum IRInstructionSrc {
    Register(usize),
    Constant(Value),
    VariableRegister(usize),
}

impl IRInstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(value) => { format!("R{}", value) },
            Self::Constant(value) => { format!("{}", value.to_string()) },
            Self::VariableRegister(value) => { format!("R{}", value) },
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRInstruction {
    Halt,
    Load {
        reg: usize,
        src: IRInstructionSrc,
    },
    Add {
        dest: usize,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Sub {
        dest: usize,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Mul {
        dest: usize,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Div {
        dest: usize,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc,
    },
    Neg {
        dest: usize,
        src: IRInstructionSrc,
    },
    Truthy {
        dest: usize,
        src: IRInstructionSrc,
    },
    Define {
        dest: usize,
        src: IRInstructionSrc,
    },
    Assign {
        dest: usize,
        src: IRInstructionSrc,
    },
}

impl IRInstruction {
    pub fn dissassemble(&self) -> String {
        match self {
           Self::Halt => { "HALT".to_string() }

            Self::Load { reg, src } => {
                format!("LOAD R{} {}", reg, src.dissassemble())
            }
            Self::Add { dest, src1, src2 } => {
                format!("ADD R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Sub { dest, src1, src2 } => {
                format!("SUB R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Mul { dest, src1, src2 } => {
                format!("MUL R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Div { dest, src1, src2 } => {
                format!("DIV R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Self::Neg { dest, src } => {
                format!("NEG R{} {}", dest, src.dissassemble())
            }
            Self::Truthy { dest, src } => {
                format!("TRUTHY R{} {}", dest, src.dissassemble())
            }
            Self::Define { dest, src } => {
                format!("DEFINE R{} {}", dest, src.dissassemble())
            }
            Self::Assign { dest, src } => {
                format!("ASSIGN R{} {}", dest, src.dissassemble())
            }
        }
    }

}

