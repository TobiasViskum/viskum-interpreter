use std::fmt::format;

use crate::{ compiler::ir_graph::{ IROperation, IRValue }, value::Value };

#[derive(Debug, Clone)]
pub enum InstructionSrc {
    Register(usize),
    Constant(Value),
}

impl InstructionSrc {
    pub fn dissassemble(&self) -> String {
        match self {
            InstructionSrc::Register(register) => { format!("R{}", register) }
            InstructionSrc::Constant(value) => { format!("{}", value.to_string()) }
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

    pub fn new_binary(operation: IROperation, dest: usize, src1: IRValue, src2: IRValue) -> Self {
        let src1 = match src1 {
            IRValue::Register(register) => InstructionSrc::Register(register),
            IRValue::Constant(value) => InstructionSrc::Constant(value),
        };

        let src2 = match src2 {
            IRValue::Register(register) => InstructionSrc::Register(register),
            IRValue::Constant(value) => InstructionSrc::Constant(value),
        };

        match operation {
            IROperation::Add => Instruction::Add { dest, src1, src2 },
            IROperation::Sub => Instruction::Sub { dest, src1, src2 },
            IROperation::Mul => Instruction::Mul { dest, src1, src2 },
            IROperation::Div => Instruction::Div { dest, src1, src2 },
            _ => panic!("Operation not supported in instruction"),
        }
    }

    pub fn new_unary(operation: IROperation, dest: usize, src: IRValue) -> Self {
        let src = match src {
            IRValue::Register(register) => InstructionSrc::Register(register),
            IRValue::Constant(value) => InstructionSrc::Constant(value),
        };

        match operation {
            IROperation::Neg => Instruction::Neg { dest, src },
            IROperation::Truthy => Instruction::Truthy { dest, src },
            _ => panic!("Operation not supported in instruction"),
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
        }
    }
}
