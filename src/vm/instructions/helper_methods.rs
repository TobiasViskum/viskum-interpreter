use crate::{ compiler::ir_graph::IRValue, operations::{ BinaryOp, UnaryOp } };

use super::{ IRInstruction, IRInstructionSrc, VMInstruction, VMInstructionSrc };

impl IRInstructionSrc {
    pub fn to_instruction_src(&self) -> VMInstructionSrc {
        match self {
            Self::Register(register) => { VMInstructionSrc::Register(*register) }
            Self::Constant(value) => { VMInstructionSrc::Constant(value.clone()) }
            Self::VariableRegister(register) => { VMInstructionSrc::Register(*register) }
        }
    }
}

impl IRInstruction {
    pub fn to_vm_instruction(&self) -> VMInstruction {
        match self {
            Self::Add { dest, src1, src2 } => {
                let src1 = src1.to_instruction_src();
                let src2 = src2.to_instruction_src();

                VMInstruction::Add { dest: *dest, src1, src2 }
            }
            Self::Sub { dest, src1, src2 } => {
                let src1 = src1.to_instruction_src();
                let src2 = src2.to_instruction_src();

                VMInstruction::Sub { dest: *dest, src1, src2 }
            }
            Self::Mul { dest, src1, src2 } => {
                let src1 = src1.to_instruction_src();
                let src2 = src2.to_instruction_src();

                VMInstruction::Mul { dest: *dest, src1, src2 }
            }
            Self::Div { dest, src1, src2 } => {
                let src1 = src1.to_instruction_src();
                let src2 = src2.to_instruction_src();

                VMInstruction::Div { dest: *dest, src1, src2 }
            }
            Self::Define { dest, src } => {
                let src = src.to_instruction_src();

                VMInstruction::Define { dest: *dest, src }
            }
            Self::Assign { dest, src } => {
                let src = src.to_instruction_src();

                VMInstruction::Assign { dest: *dest, src }
            }
            Self::Neg { dest, src } => {
                let src = src.to_instruction_src();

                VMInstruction::Neg { dest: *dest, src }
            }
            Self::Truthy { dest, src } => {
                let src = src.to_instruction_src();

                VMInstruction::Truthy { dest: *dest, src }
            }
            Self::Load { reg, src } => {
                let src = src.to_instruction_src();

                VMInstruction::Load { reg: *reg, src }
            }
            Self::Halt => VMInstruction::Halt,
        }
    }

    pub fn modify_binary_instruction(
        &mut self,
        dest: usize,
        src1: IRInstructionSrc,
        src2: IRInstructionSrc
    ) {
        match self {
            Self::Add { dest: _, src1: _, src2: _ } => {
                *self = Self::Add { dest, src1, src2 };
            }
            Self::Sub { dest: _, src1: _, src2: _ } => {
                *self = Self::Sub { dest, src1, src2 };
            }
            Self::Mul { dest: _, src1: _, src2: _ } => {
                *self = Self::Mul { dest, src1, src2 };
            }
            Self::Div { dest: _, src1: _, src2: _ } => {
                *self = Self::Div { dest, src1, src2 };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_definement_instruction(&mut self, dest: usize, src: IRInstructionSrc) {
        match self {
            Self::Define { dest: _, src: _ } => {
                *self = Self::Define { dest, src };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_assignment_instruction(&mut self, dest: usize, src: IRInstructionSrc) {
        match self {
            Self::Assign { dest: _, src: _ } => {
                *self = Self::Assign { dest, src };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_unary_instruction(&mut self, dest: usize, src: IRInstructionSrc) {
        match self {
            Self::Neg { dest: _, src: _ } => {
                *self = Self::Neg { dest, src };
            }
            Self::Truthy { dest: _, src: _ } => {
                *self = Self::Truthy { dest, src };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn modify_load_instruction(&mut self, reg: usize, src: IRInstructionSrc) {
        match self {
            Self::Load { reg: _, src: _ } => {
                *self = Self::Load { reg, src };
            }
            _ => {
                panic!("Operation not supported in instruction");
            }
        }
    }

    pub fn new_binary(operation: &BinaryOp, dest: usize, src1: IRValue, src2: IRValue) -> Self {
        let src1 = src1.to_ir_instruction_src();

        let src2 = src2.to_ir_instruction_src();

        match operation {
            BinaryOp::Add => Self::Add { dest, src1, src2 },
            BinaryOp::Sub => Self::Sub { dest, src1, src2 },
            BinaryOp::Mul => Self::Mul { dest, src1, src2 },
            BinaryOp::Div => Self::Div { dest, src1, src2 },
        }
    }

    pub fn new_define(dest: usize, src: IRValue) -> Self {
        let src = src.to_ir_instruction_src();

        Self::Define { dest, src }
    }

    pub fn new_assign(dest: usize, src: IRValue) -> Self {
        let src = src.to_ir_instruction_src();

        Self::Assign { dest, src }
    }

    pub fn new_unary(operation: &UnaryOp, dest: usize, src: IRValue) -> Self {
        let src = src.to_ir_instruction_src();

        match operation {
            UnaryOp::Neg => Self::Neg { dest, src },
            UnaryOp::Truthy => Self::Truthy { dest, src },
        }
    }

    pub fn new_load(reg: IRValue, src: IRValue) -> Self {
        let reg = match reg {
            IRValue::Register(register) => register,
            _ => panic!("Destination must be a register"),
        };

        let src = src.to_ir_instruction_src();

        Self::Load { reg, src }
    }
}
