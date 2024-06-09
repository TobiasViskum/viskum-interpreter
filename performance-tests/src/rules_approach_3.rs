use crate::{
    instructions::Operand,
    vm_util::{ RegisterValue, Stack, StackValue, Value },
    Registers,
};

fn halt_rule(operands: &[Operand; 4], _: &[Value], stack: &mut Stack, registers: &mut Registers) {
    registers.set_ip(registers.get_ip() + 1);
}

fn add_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let (dst, src1, src2) = unsafe {
        (
            operands.get_unchecked(0).unwrap_dst(),
            operands.get_unchecked(1).unwrap_src(),
            operands.get_unchecked(2).unwrap_src(),
        )
    };

    let v1 = src1.get_val(&constant_pool, &stack, &registers);
    let v2 = src2.get_val(&constant_pool, &stack, &registers);

    registers.set_reg(*dst, RegisterValue::Value(v1.add(v2)));

    registers.inc_ip();
}

fn sub_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let (dst, src1, src2) = unsafe {
        (
            operands.get_unchecked(0).unwrap_dst(),
            operands.get_unchecked(1).unwrap_src(),
            operands.get_unchecked(2).unwrap_src(),
        )
    };

    let v1 = src1.get_val(&constant_pool, &stack, &registers);
    let v2 = src2.get_val(&constant_pool, &stack, &registers);

    registers.set_reg(*dst, RegisterValue::Value(v1.sub(v2)));

    registers.inc_ip();
}

fn mul_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let (dst, src1, src2) = unsafe {
        (
            operands.get_unchecked(0).unwrap_dst(),
            operands.get_unchecked(1).unwrap_src(),
            operands.get_unchecked(2).unwrap_src(),
        )
    };

    let v1 = src1.get_val(&constant_pool, &stack, &registers);
    let v2 = src2.get_val(&constant_pool, &stack, &registers);

    registers.set_reg(*dst, RegisterValue::Value(v1.mul(v2)));

    registers.inc_ip();
}

fn div_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let (dst, src1, src2) = unsafe {
        (
            operands.get_unchecked(0).unwrap_dst(),
            operands.get_unchecked(1).unwrap_src(),
            operands.get_unchecked(2).unwrap_src(),
        )
    };

    let v1 = src1.get_val(&constant_pool, &stack, &registers);
    let v2 = src2.get_val(&constant_pool, &stack, &registers);

    registers.set_reg(*dst, RegisterValue::Value(v1.div(v2)));

    registers.inc_ip();
}

fn push_function_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    _: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let instrs_amount = unsafe { operands.get_unchecked(0).unwrap_instrs_amount() };

    stack.push(StackValue::FnPtr(registers.get_ip()));

    registers.add_ip(instrs_amount + 1);
}

fn cmp_lt_jump_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let (src1, src2, true_pos, false_pos) = unsafe {
        (
            operands.get_unchecked(0).unwrap_src(),
            operands.get_unchecked(1).unwrap_src(),
            operands.get_unchecked(2).unwrap_jmp_pos(),
            operands.get_unchecked(3).unwrap_jmp_pos(),
        )
    };

    let v1 = src1.get_val(constant_pool, stack, registers);
    let v2 = src2.get_val(constant_pool, stack, registers);

    match v1.cmp_l(v2) {
        true => registers.set_ip(true_pos),
        false => registers.set_ip(false_pos),
    }
}

fn call_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    _: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let (stack_ptr, args_count) = unsafe {
        (
            operands.get_unchecked(0).unwrap_stack_ptr(),
            operands.get_unchecked(1).unwrap_args_count(),
        )
    };

    stack.push_stack_frame(stack.len() - args_count);

    let fn_ptr = stack.get(stack_ptr).unwrap_fn_ptr();

    registers.set_ip(fn_ptr + 1);
}

fn ret_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let src = unsafe { operands.get_unchecked(0).unwrap_src() };

    let v1 = src.get_val(constant_pool, stack, registers).clone();

    stack.truncate(stack.read_stack_frame());
    stack.pop_stack_frame();

    // let instr_ptr = stack.pop().unwrap_instr_ptr();

    stack.push(StackValue::Value(v1));

    // registers.set_ip(instr_ptr);
}

fn push_stack_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let src = unsafe { operands.get_unchecked(0).unwrap_src() };

    let v1 = src.get_val(constant_pool, stack, registers).clone();
    stack.push(StackValue::Value(v1));

    registers.inc_ip();
}

fn push_instr_ptr_rule(
    operands: &[Operand; 4], // I want to specify it's an ADD instruction
    constant_pool: &[Value],
    stack: &mut Stack,
    registers: &mut Registers
) {
    let idx = unsafe { operands.get_unchecked(0).unwrap_instr_ptr() };

    // stack.push(StackValue::InstrPtr(idx));

    registers.inc_ip();
}

pub const RULES_3: [fn(&[Operand; 4], &[Value], &mut Stack, &mut Registers); 11] = [
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
