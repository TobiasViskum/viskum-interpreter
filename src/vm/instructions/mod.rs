use crate::{ compiler::ir_graph::IRValue, operations::{ BinaryOp, UnaryOp }, value::Value };

#[derive(Debug, Clone)]
pub enum InstructionSrc {
    Register(usize),
    Constant(Value),
    VariableRegister(usize),
}

impl InstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            InstructionSrc::Register(register) => { format!("R{}", register) }
            InstructionSrc::Constant(value) => { format!("{}", value.to_string()) }
            InstructionSrc::VariableRegister(register) => { format!("R{}", register) }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Halt,
    Load {
        reg: usize,
        value: Value,
    },

    // Binary
    Add {
        dest: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Sub {
        dest: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Mul {
        dest: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Div {
        dest: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },

    Define {
        dest: usize,
        src: InstructionSrc,
    },

    // Unary
    Neg {
        dest: usize,
        src: InstructionSrc,
    },
    Truthy {
        dest: usize,
        src: InstructionSrc,
    },
}

impl Instruction {
    pub fn modify_binary_instruction(
        &mut self,
        dest: usize,
        src1: InstructionSrc,
        src2: InstructionSrc
    ) {
        match self {
            Instruction::Add { dest: _, src1: _, src2: _ } => {
                *self = Instruction::Add { dest, src1, src2 };
            }
            Instruction::Sub { dest: _, src1: _, src2: _ } => {
                *self = Instruction::Sub { dest, src1, src2 };
            }
            Instruction::Mul { dest: _, src1: _, src2: _ } => {
                *self = Instruction::Mul { dest, src1, src2 };
            }
            Instruction::Div { dest: _, src1: _, src2: _ } => {
                *self = Instruction::Div { dest, src1, src2 };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_definement_instruction(&mut self, dest: usize, src: InstructionSrc) {
        match self {
            Instruction::Define { dest: _, src: _ } => {
                *self = Instruction::Define { dest: dest, src };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_unary_instruction(&mut self, dest: usize, src: InstructionSrc) {
        match self {
            Instruction::Neg { dest: _, src: _ } => {
                *self = Instruction::Neg { dest, src };
            }
            Instruction::Truthy { dest: _, src: _ } => {
                *self = Instruction::Truthy { dest, src };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_load_instruction(&mut self, reg: usize, value: Value) {
        match self {
            Instruction::Load { reg: _, value: _ } => {
                *self = Instruction::Load { reg, value };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn new_binary(operation: &BinaryOp, dest: usize, src1: IRValue, src2: IRValue) -> Self {
        let src1 = src1.to_instruction_src();

        let src2 = src2.to_instruction_src();

        match operation {
            BinaryOp::Add => Instruction::Add { dest, src1, src2 },
            BinaryOp::Sub => Instruction::Sub { dest, src1, src2 },
            BinaryOp::Mul => Instruction::Mul { dest, src1, src2 },
            BinaryOp::Div => Instruction::Div { dest, src1, src2 },
        }
    }

    pub fn new_define(dest: usize, src: IRValue) -> Self {
        let src = src.to_instruction_src();

        Instruction::Define { dest, src }
    }

    pub fn new_unary(operation: &UnaryOp, dest: usize, src: IRValue) -> Self {
        let src = src.to_instruction_src();

        match operation {
            UnaryOp::Neg => Instruction::Neg { dest, src },
            UnaryOp::Truthy => Instruction::Truthy { dest, src },
        }
    }

    pub fn new_load(reg: IRValue, value: Value) -> Self {
        let reg = match reg {
            IRValue::Register(register) => register,
            _ => panic!("Destination must be a register"),
        };

        Instruction::Load { reg, value }
    }

    pub fn dissassemble(&self) -> String {
        match self {
            Instruction::Halt => { "HALT".to_string() }
            Instruction::Load { reg, value } => { format!("LOAD R{} {}", reg, value.to_string()) }
            Instruction::Add { dest, src1, src2 } => {
                format!("ADD R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Instruction::Sub { dest, src1, src2 } => {
                format!("SUB R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Instruction::Mul { dest, src1, src2 } => {
                format!("MUL R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Instruction::Div { dest, src1, src2 } => {
                format!("DIV R{} {} {}", dest, src1.dissassemble(), src2.dissassemble())
            }
            Instruction::Neg { dest, src } => { format!("NEG R{} {}", dest, src.dissassemble()) }
            Instruction::Truthy { dest, src } => {
                format!("TRUTHY R{} {}", dest, src.dissassemble())
            }
            Instruction::Define { dest, src } => {
                format!("DEFINE R{} {}", dest, src.dissassemble())
            }
        }
    }
}
