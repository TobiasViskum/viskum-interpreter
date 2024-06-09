use std::fmt::Debug;

use crate::{
    vm_util::{ ConstPtr, Dst, Register, RegisterValue, Src, Stack, StackPtr, StackValue, Value },
    Registers,
};

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Halt,
    Add,
    Sub,
    Div,
    Mul,
    PushFunction,
    CmpLtJmp,
    Call,
    Ret,
    PushStack,
    PushInstrPtr,
}

impl From<Opcode> for usize {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Halt => 0,
            Opcode::Add => 1,
            Opcode::Sub => 2,
            Opcode::Mul => 3,
            Opcode::Div => 4,
            Opcode::PushFunction => 5,
            Opcode::CmpLtJmp => 6,
            Opcode::Call => 7,
            Opcode::Ret => 8,
            Opcode::PushStack => 9,
            Opcode::PushInstrPtr => 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Src(Src),
    Dst(Register),
    InstrsAmount(usize),
    InstrPtr(usize),
    JmpPos(usize),
    StackPtr(StackPtr),
    ArgsCount(usize),
    Empty,
}

impl Operand {
    #[inline]
    pub fn unwrap_src(&self) -> &Src {
        match self {
            Self::Src(src) => src,
            _ => panic!("Expected src"),
        }
    }

    #[inline]
    pub fn unwrap_dst(&self) -> &Register {
        match self {
            Self::Dst(reg) => reg,
            _ => panic!("Expected dst"),
        }
    }

    #[inline]
    pub fn unwrap_instrs_amount(&self) -> usize {
        match self {
            Self::InstrsAmount(v) => *v,
            _ => panic!("Expected instrs_amount"),
        }
    }

    #[inline]
    pub fn unwrap_instr_ptr(&self) -> usize {
        match self {
            Self::InstrPtr(v) => *v,
            _ => panic!("Expected instr_ptr"),
        }
    }

    #[inline]
    pub fn unwrap_jmp_pos(&self) -> usize {
        match self {
            Self::JmpPos(v) => *v,
            _ => panic!("Expected jmp_pos"),
        }
    }

    #[inline]
    pub fn unwrap_stack_ptr(&self) -> &StackPtr {
        match self {
            Self::StackPtr(v) => v,
            _ => panic!("Expected stack_ptr"),
        }
    }

    #[inline]
    pub fn unwrap_args_count(&self) -> usize {
        match self {
            Self::ArgsCount(v) => *v,
            _ => panic!("Expected args_count"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) enum Instruction2 {
    Halt(HaltInstruction),
    Add(AddInstruction),
    Sub(SubInstruction),
    Mul(MulInstruction),
    Div(DivInstruction),
    PushFunction(PushFunctionInstruction),
    CmpLtJmp(CmpLtJmpInstruction),
    Call(CallInstruction),
    Ret(RetInstruction),
    PushStack(PushStackInstruction),
    PushInstrPtr(PushInstrPtrInstruction),
}

impl From<Instruction2> for usize {
    fn from(value: Instruction2) -> Self {
        match value {
            Instruction2::Halt(_) => 0,
            Instruction2::Add(_) => 1,
            Instruction2::Sub(_) => 2,
            Instruction2::Mul(_) => 3,
            Instruction2::Div(_) => 4,
            Instruction2::PushFunction(_) => 5,
            Instruction2::CmpLtJmp(_) => 6,
            Instruction2::Call(_) => 7,
            Instruction2::Ret(_) => 8,
            Instruction2::PushStack(_) => 9,
            Instruction2::PushInstrPtr(_) => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct HaltInstruction;

impl HaltInstruction {
    pub fn new() -> Self {
        Self
    }

    #[inline]
    pub fn execute(&self, _: &[Value], stack: &mut Stack, registers: &mut Registers) {
        // println!("Final stack: {:?}", stack);
        registers.inc_ip();
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct AddInstruction {
    dst: Register,
    src1: Src,
    src2: Src,
}

impl AddInstruction {
    pub fn new(dst: Register, src1: Src, src2: Src) -> Self {
        Self { dst, src1, src2 }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = self.src1.get_val(&constant_pool, &stack, &registers);
        let v2 = self.src2.get_val(&constant_pool, &stack, &registers);

        registers.set_reg(self.dst, RegisterValue::Value(v1.add(v2)));

        registers.inc_ip();
    }
}
#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct SubInstruction {
    dst: Register,
    src1: Src,
    src2: Src,
}

impl SubInstruction {
    pub fn new(dst: Register, src1: Src, src2: Src) -> Self {
        Self { dst, src1, src2 }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = self.src1.get_val(constant_pool, stack, registers);
        let v2 = self.src2.get_val(constant_pool, stack, registers);

        registers.set_reg(self.dst, RegisterValue::Value(v1.sub(v2)));

        registers.inc_ip();
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct MulInstruction {
    dst: Register,
    src1: Src,
    src2: Src,
}

impl MulInstruction {
    pub fn new(dst: Register, src1: Src, src2: Src) -> Self {
        Self { dst, src1, src2 }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = self.src1.get_val(constant_pool, stack, registers);
        let v2 = self.src2.get_val(constant_pool, stack, registers);

        registers.set_reg(self.dst, RegisterValue::Value(v1.mul(v2)));

        registers.inc_ip();
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct DivInstruction {
    dst: Register,
    src1: Src,
    src2: Src,
}

impl DivInstruction {
    pub fn new(dst: Register, src1: Src, src2: Src) -> Self {
        Self { dst, src1, src2 }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = self.src1.get_val(constant_pool, stack, registers);
        let v2 = self.src2.get_val(constant_pool, stack, registers);

        registers.set_reg(self.dst, RegisterValue::Value(v1.div(v2)));

        registers.inc_ip();
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct PushFunctionInstruction {
    instructions_amount: usize,
}

impl PushFunctionInstruction {
    pub fn new(instructions_amount: usize) -> Self {
        Self { instructions_amount }
    }

    #[inline]
    pub fn execute(&self, _: &[Value], stack: &mut Stack, registers: &mut Registers) {
        stack.push(StackValue::FnPtr(registers.get_ip()));
        registers.add_ip(self.instructions_amount + 1);
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct CmpLtJmpInstruction {
    src1: Src,
    src2: Src,
    true_pos: usize,
    false_pos: usize,
}

impl CmpLtJmpInstruction {
    pub fn new(src1: Src, src2: Src, true_pos: usize, false_pos: usize) -> Self {
        Self {
            src1,
            src2,
            true_pos,
            false_pos,
        }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = self.src1.get_val(constant_pool, stack, registers);
        let v2 = self.src2.get_val(constant_pool, stack, registers);

        if v1.cmp_l(v2) {
            registers.set_ip(self.true_pos);
        } else {
            registers.set_ip(self.false_pos);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct CallInstruction {
    stack_ptr: StackPtr,
    args_count: usize,
}

impl CallInstruction {
    pub fn new(stack_ptr: StackPtr, args_count: usize) -> Self {
        Self { stack_ptr, args_count }
    }

    #[inline]
    pub fn execute(&self, _: &[Value], stack: &mut Stack, registers: &mut Registers) {
        stack.push_stack_frame(stack.len() - self.args_count);

        let fn_ptr = stack.get(&self.stack_ptr).unwrap_fn_ptr();

        registers.set_ip(fn_ptr + 1);
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct RetInstruction {
    src: Src,
}

impl RetInstruction {
    pub fn new(src: Src) -> Self {
        Self { src }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = *self.src.get_val(constant_pool, stack, registers);

        stack.truncate(stack.read_stack_frame());
        stack.pop_stack_frame();

        // let instr_ptr = stack.pop().unwrap_instr_ptr();

        stack.push(StackValue::Value(v1));

        // registers.set_ip(instr_ptr);
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct PushStackInstruction {
    src: Src,
}

impl PushStackInstruction {
    pub fn new(src: Src) -> Self {
        Self { src }
    }

    #[inline]
    pub fn execute(&self, constant_pool: &[Value], stack: &mut Stack, registers: &mut Registers) {
        let v1 = *self.src.get_val(constant_pool, stack, registers);
        stack.push(StackValue::Value(v1));

        registers.inc_ip();
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub(super) struct PushInstrPtrInstruction {
    idx: usize,
}

impl PushInstrPtrInstruction {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }

    #[inline]
    pub fn execute(&self, _: &[Value], stack: &mut Stack, registers: &mut Registers) {
        // stack.push(StackValue::InstrPtr(self.idx));

        registers.inc_ip();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Src1 {
    data: (usize, bool),
    fn_ptr: usize,
}

impl Src1 {
    pub fn new(data: (usize, bool), fn_ptr: usize) -> Self {
        Self { data, fn_ptr }
    }

    #[inline]
    pub fn get_get_data(&self) -> (usize, bool) {
        self.data
    }

    #[inline]
    pub fn get_fn_ptr(&self) -> usize {
        self.fn_ptr
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Halt,
    Add {
        dst: Register,
        src1: StackPtr,
        src2: StackPtr,
    },
    Sub {
        dst: Dst,
        src1: StackPtr,
        src2: ConstPtr,
    },
    Mul {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    Div {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    PushFunction {
        instructions_amount: usize,
    },
    CmpEJmp {
        src1: Src,
        src2: Src,
        true_pos: usize,
        false_pos: usize,
    },
    CmpNeJmp {
        src1: Src,
        src2: Src,
        true_pos: usize,
        false_pos: usize,
    },
    CmpLJmp {
        src1: Src,
        src2: Src,
        true_pos: usize,
        false_pos: usize,
    },
    CmpLeJmp {
        src1: Src,
        src2: Src,
        true_pos: usize,
        false_pos: usize,
    },
    CmpGJmp {
        src1: Src,
        src2: Src,
        true_pos: usize,
        false_pos: usize,
    },
    CmpGeJmp {
        src1: Src,
        src2: Src,
        true_pos: usize,
        false_pos: usize,
    },
    CmpE {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    CmpNe {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    CmpG {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    CmpGe {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    CmpL {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    CmpLe {
        dst: Dst,
        src1: Src,
        src2: Src,
    },
    Call {
        stack_ptr: StackPtr,
        args_count: usize,
        tco: bool,
    },
    RetStack {
        stack_ptr: StackPtr,
    },
    RetConst {
        const_ptr: ConstPtr,
    },
    RetRegister {
        reg: Register,
    },
    PushStackConst {
        const_ptr: ConstPtr,
    },
    PushStackRegister {
        reg: Register,
    },
}

impl From<Instruction> for usize {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Halt => 0,
            Instruction::Add { .. } => 1,
            Instruction::Sub { .. } => 2,
            Instruction::Mul { .. } => 3,
            Instruction::Div { .. } => 4,
            Instruction::PushFunction { .. } => 5,
            Instruction::CmpEJmp { .. } => 6,
            Instruction::CmpNeJmp { .. } => 7,
            Instruction::CmpGJmp { .. } => 8,
            Instruction::CmpGeJmp { .. } => 9,
            Instruction::CmpLJmp { .. } => 10,
            Instruction::CmpLeJmp { .. } => 11,
            Instruction::CmpE { .. } => 12,
            Instruction::CmpNe { .. } => 13,
            Instruction::CmpG { .. } => 14,
            Instruction::CmpGe { .. } => 15,
            Instruction::CmpL { .. } => 16,
            Instruction::CmpLe { .. } => 17,
            Instruction::Call { .. } => 18,
            Instruction::RetStack { .. } => 19,
            Instruction::RetConst { .. } => 20,
            Instruction::RetRegister { .. } => 21,
            Instruction::PushStackConst { .. } => 22,
            Instruction::PushStackRegister { .. } => 23,
        }
    }
}

impl Instruction {
    pub fn dissasemble(&self, stack_frame: Option<usize>) -> String {
        match self {
            Self::Halt => String::from("HALT"),
            Self::Add { dst, .. } => "ADD".to_string(),
            Self::Sub { dst, src1, src2 } => format!("SUB"),
            Self::Mul { dst, src1, src2 } =>
                format!(
                    "MUL {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                ),
            Self::Div { dst, src1, src2 } =>
                format!(
                    "DIV {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                ),
            Self::PushFunction { instructions_amount } => {
                format!("PUSH_FUNCTION {}", instructions_amount)
            }
            Self::CmpEJmp { src1, src2, true_pos, false_pos } => {
                format!(
                    "CMP_E_JMP {} {} {} {}",
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame),
                    true_pos,
                    false_pos
                )
            }
            Self::CmpNeJmp { src1, src2, true_pos, false_pos } => {
                format!(
                    "CMP_NE_JMP {} {} {} {}",
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame),
                    true_pos,
                    false_pos
                )
            }
            Self::CmpLJmp { src1, src2, true_pos, false_pos } => {
                format!(
                    "CMP_L_JMP {} {} {} {}",
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame),
                    true_pos,
                    false_pos
                )
            }
            Self::CmpLeJmp { src1, src2, true_pos, false_pos } => {
                format!(
                    "CMP_LE_JMP {} {} {} {}",
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame),
                    true_pos,
                    false_pos
                )
            }
            Self::CmpGJmp { src1, src2, true_pos, false_pos } => {
                format!(
                    "CMP_G_JMP {} {} {} {}",
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame),
                    true_pos,
                    false_pos
                )
            }
            Self::CmpGeJmp { src1, src2, true_pos, false_pos } => {
                format!(
                    "CMP_GE_JMP {} {} {} {}",
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame),
                    true_pos,
                    false_pos
                )
            }
            Self::CmpE { dst, src1, src2 } => {
                format!(
                    "CMP_E {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                )
            }
            Self::CmpNe { dst, src1, src2 } => {
                format!(
                    "CMP_NE {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                )
            }
            Self::CmpL { dst, src1, src2 } => {
                format!(
                    "CMP_L {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                )
            }
            Self::CmpLe { dst, src1, src2 } => {
                format!(
                    "CMP_LE {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                )
            }
            Self::CmpG { dst, src1, src2 } => {
                format!(
                    "CMP_G {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                )
            }
            Self::CmpGe { dst, src1, src2 } => {
                format!(
                    "CMP_GE {} {} {}",
                    dst.dissasemble(),
                    src1.dissasemble(stack_frame),
                    src2.dissasemble(stack_frame)
                )
            }
            Self::Call { stack_ptr, args_count, .. } => {
                format!("CALL {} {}\n", stack_ptr.dissasemble(stack_frame), args_count)
            }
            // Self::Ret { src } => { format!("RET_POP {} \n", src.dissasemble(stack_frame)) }
            _ => { "hei".to_string() }
            // Self::PushStack { src } => { format!("PUSH_STACK {}", src.dissasemble(stack_frame)) }
        }
    }
}
