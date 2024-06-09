use crate::{
    instructions::{
        AddInstruction,
        CallInstruction,
        CmpLtJmpInstruction,
        DivInstruction,
        HaltInstruction,
        Instruction2,
        MulInstruction,
        Operand,
        PushFunctionInstruction,
        PushInstrPtrInstruction,
        PushStackInstruction,
        RetInstruction,
        SubInstruction,
    },
    vm_util::{ RegisterValue, Stack, StackValue, Value },
    Registers,
};

#[inline]
fn halt_rule(
    instruction: &Instruction2,
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &HaltInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn add_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &AddInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn sub_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &SubInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn mul_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &MulInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn div_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &DivInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn push_function_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &PushFunctionInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn cmp_lt_jump_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &CmpLtJmpInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn call_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &CallInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn ret_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &RetInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn push_stack_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &PushStackInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

#[inline]
fn push_instr_ptr_rule(
    instruction: &Instruction2, // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    unsafe {
        let instr: &PushInstrPtrInstruction = std::mem::transmute(instruction);
        instr.execute(constant_pool, stack, registers);
    }
}

pub const RULES_4: [fn(&Instruction2, &[Value], &mut Stack, &mut Registers); 11] = [
    halt_rule,
    add_rule,
    sub_rule,
    mul_rule,
    div_rule,
    push_function_rule,
    cmp_lt_jump_rule,
    call_rule,
    ret_rule,
    push_stack_rule,
    push_instr_ptr_rule,
];
