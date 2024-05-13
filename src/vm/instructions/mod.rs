use crate::{ operations::{ BinaryOp, UnaryOp }, value::Value };

#[derive(Debug, Clone)]
pub enum InstructionPos {
    Register(usize),
    Stack(usize),
}

impl InstructionPos {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(pos) => { format!("R{}", pos) }
            Self::Stack(pos) => { format!("[{}]", pos) }
        }
    }
}

#[derive(Debug, Clone)]
pub enum InstructionSrc {
    Register(usize),
    Constant(Value),
    Stack(usize),
}

impl InstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Register(pos) => { format!("R{}", pos) }
            Self::Constant(value) => { format!("C({})", value.to_string()) }
            Self::Stack(pos) => { format!("[{}]", pos) }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Halt,
    JmpPop {
        pos: usize,
        amount: usize,
    },
    Pop {
        amount: usize,
    },
    Add {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Sub {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Mul {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Div {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Neg {
        reg: usize,
        src: InstructionSrc,
    },
    Truthy {
        reg: usize,
        src: InstructionSrc,
    },
    Define {
        stack_pos: usize,
        src: InstructionSrc,
    },
    Assign {
        stack_pos: usize,
        src: InstructionSrc,
    },
    Cmp {
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Jmp {
        pos: usize,
    },
    JE {
        true_pos: usize,
        false_pos: usize,
    },
    JNE {
        true_pos: usize,
        false_pos: usize,
    },
    JG {
        true_pos: usize,
        false_pos: usize,
    },
    JGE {
        true_pos: usize,
        false_pos: usize,
    },
    JL {
        true_pos: usize,
        false_pos: usize,
    },
    JLE {
        true_pos: usize,
        false_pos: usize,
    },
}

impl Instruction {
    pub fn dissassemble(&self) -> String {
        match self {
            Self::Cmp { src1, src2 } => {
                format!("CMP {} {}", src1.dissassemble(), src2.dissassemble())
            }
            Self::Pop { amount } => { format!("POP {}", amount) }
            Self::Jmp { pos } => { format!("JMP {}", pos) }
            Self::JE { true_pos, false_pos } => { format!("JE {} {}", true_pos, false_pos) }
            Self::JNE { true_pos, false_pos } => { format!("JNE {} {}", true_pos, false_pos) }
            Self::JG { true_pos, false_pos } => { format!("JG {} {}", true_pos, false_pos) }
            Self::JGE { true_pos, false_pos } => { format!("JGE {} {}", true_pos, false_pos) }
            Self::JL { true_pos, false_pos } => { format!("JL {} {}", true_pos, false_pos) }
            Self::JLE { true_pos, false_pos } => { format!("JLE {} {}", true_pos, false_pos) }
            Self::Halt => { "HALT".to_string() }
            Self::JmpPop { pos, amount } => { format!("JMP_POP {} {}", pos, amount) }
            Self::Add { reg, src1, src2 } => {
                format!("ADD R{} {} {}", reg, src1.dissassemble(), src2.dissassemble())
            }
            Self::Sub { reg, src1, src2 } => {
                format!("SUB R{} {} {}", reg, src1.dissassemble(), src2.dissassemble())
            }
            Self::Mul { reg, src1, src2 } => {
                format!("MUL R{} {} {}", reg, src1.dissassemble(), src2.dissassemble())
            }
            Self::Div { reg, src1, src2 } => {
                format!("DIV R{} {} {}", reg, src1.dissassemble(), src2.dissassemble())
            }
            Self::Neg { reg, src } => { format!("NEG R{} {}", reg, src.dissassemble()) }
            Self::Truthy { reg, src } => { format!("TRUTHY R{} {}", reg, src.dissassemble()) }
            Self::Define { stack_pos, src } => {
                format!("DEFINE [{}] {}", stack_pos, src.dissassemble())
            }
            Self::Assign { stack_pos, src } => {
                format!("ASSIGN [{}] {}", stack_pos, src.dissassemble())
            }
        }
    }
}
