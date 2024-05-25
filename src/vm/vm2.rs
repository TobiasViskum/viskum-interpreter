use std::{ rc::Rc, time::Instant };
use crate::value::Value;

use super::{
    helper_structs::{ ConstantsTable, RuntimeInformation, VMRegisters, VMStack, VMRegisters2 },
    instructions::{ fast_instr::FastInstr, Instruction },
};

fn halt_rule(
    (_, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    _: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    ip + 1
}
fn load_bool_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, bool_value) = instr.get_load_bool();
    *registers.get_mut(dst as usize) = Some(Value::Bool(bool_value));

    ip + 1
}

fn load_i32_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, value) = instr.get_load_i32();
    *registers.get_mut(dst as usize) = Some(Value::Int32(value));

    ip + 1
}
fn load_string_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let instr_amount = instr.get_string_instr_amount() as usize;

    panic!("String not impl yet. TODO: Pass instructions as immutable reference");
    // let instrs = runtime_information.get_next_instrs_range(instr_amount);
    // let (dst, string) = instr.get_load_string(instrs);

    // *registers.get_mut(dst as usize) = Some(Value::String(string.into()));

    ip + 1 + instr_amount
}
fn add_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_binary();
    let lhs = &registers.get(reg1 as usize);
    let rhs = &registers.get(reg2 as usize);

    *registers.get_mut(dst as usize) = Some(unsafe { lhs.add(rhs).unwrap_unchecked() });
    ip + 1
}
fn sub_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_binary();
    let lhs = &registers.get(reg1 as usize);
    let rhs = &registers.get(reg2 as usize);

    *registers.get_mut(dst as usize) = Some(unsafe { lhs.sub(rhs).unwrap_unchecked() });

    ip + 1
}
fn mul_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_binary();
    let lhs = &registers.get(reg1 as usize);
    let rhs = &registers.get(reg2 as usize);

    *registers.get_mut(dst as usize) = Some(unsafe { lhs.mul(rhs).unwrap_unchecked() });
    ip + 1
}
fn div_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_binary();
    let lhs = &registers.get(reg1 as usize);
    let rhs = &registers.get(reg2 as usize);

    *registers.get_mut(dst as usize) = Some(unsafe { lhs.div(rhs).unwrap_unchecked() });
    ip + 1
}
fn neg_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg) = instr.get_unary();
    let operand = &registers.get(reg as usize);

    *registers.get_mut(dst as usize) = Some(unsafe { operand.neg().unwrap_unchecked() });
    ip + 1
}
fn not_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg) = instr.get_unary();
    let operand = &registers.get(reg as usize);

    *registers.get_mut(dst as usize) = Some(operand.not());
    ip + 1
}
fn jmp_rule(
    (instr, _): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    _: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    instr.get_jmp() as usize
}
fn jmp_pop_rule(
    (instr, _): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    _: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let (jmp_pos, pop_amount) = instr.get_jmp_pop();

    stack.decrement_stack_height(pop_amount as usize);

    jmp_pos as usize
}
fn cmp_eq_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_cmp();

    let val = registers
        .get(reg1 as usize)
        .cmp_e(&registers.get(reg2 as usize))
        .unwrap();

    *registers.get_mut(dst as usize) = Some(Value::Bool(val));

    ip + 1
}
fn cmp_neq_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_cmp();

    let val = registers
        .get(reg1 as usize)
        .cmp_ne(&registers.get(reg2 as usize))
        .unwrap();

    *registers.get_mut(dst as usize) = Some(Value::Bool(val));

    ip + 1
}
fn cmp_gt_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_cmp();

    let val = registers
        .get(reg1 as usize)
        .cmp_g(&registers.get(reg2 as usize))
        .unwrap();

    *registers.get_mut(dst as usize) = Some(Value::Bool(val));

    ip + 1
}
fn cmp_geq_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_cmp();

    let val = registers
        .get(reg1 as usize)
        .cmp_ge(&registers.get(reg2 as usize))
        .unwrap();

    *registers.get_mut(dst as usize) = Some(Value::Bool(val));

    ip + 1
}
fn cmp_lt_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_cmp();

    let value = unsafe {
        registers
            .get(reg1 as usize)
            .cmp_l(&registers.get(reg2 as usize))
            .unwrap_unchecked()
    };

    *registers.get_mut(dst as usize) = Some(Value::Bool(value));

    ip + 1
}
fn cmp_leq_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (dst, reg1, reg2) = instr.get_cmp();

    let val = registers
        .get(reg1 as usize)
        .cmp_le(&registers.get(reg2 as usize))
        .unwrap();

    *registers.get_mut(dst as usize) = Some(Value::Bool(val));

    ip + 1
}
fn cmp_eq_jmp_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (reg1, reg2, true_pos, false_pos) = instr.get_cmp_jmp();

    let condition = registers
        .get(reg1 as usize)
        .cmp_e(&registers.get(reg2 as usize))
        .unwrap();

    if condition {
        true_pos as usize
    } else {
        false_pos as usize
    }
}
fn cmp_neq_jmp_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (reg1, reg2, true_pos, false_pos) = instr.get_cmp_jmp();

    let condition = registers
        .get(reg1 as usize)
        .cmp_ne(&registers.get(reg2 as usize))
        .unwrap();

    if condition {
        true_pos as usize
    } else {
        false_pos as usize
    }
}
fn cmp_gt_jmp_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (reg1, reg2, true_pos, false_pos) = instr.get_cmp_jmp();

    let condition = registers
        .get(reg1 as usize)
        .cmp_g(&registers.get(reg2 as usize))
        .unwrap();

    if condition {
        true_pos as usize
    } else {
        false_pos as usize
    }
}
fn cmp_geq_jmp_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (reg1, reg2, true_pos, false_pos) = instr.get_cmp_jmp();

    let condition = registers
        .get(reg1 as usize)
        .cmp_ge(&registers.get(reg2 as usize))
        .unwrap();

    if condition {
        true_pos as usize
    } else {
        false_pos as usize
    }
}
fn cmp_lt_jmp_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (reg1, reg2, true_pos, false_pos) = instr.get_cmp_jmp();

    let lhs = registers.get(reg1 as usize);

    let rhs = registers.get(reg2 as usize);

    let condition = unsafe { lhs.cmp_l(&rhs).unwrap_unchecked() };

    if condition {
        true_pos as usize
    } else {
        false_pos as usize
    }
}
fn cmp_leq_jmp_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    _: &mut VMStack
) -> usize {
    let (reg1, reg2, true_pos, false_pos) = instr.get_cmp_jmp();

    let condition = registers
        .get(reg1 as usize)
        .cmp_le(&registers.get(reg2 as usize))
        .unwrap();

    if condition {
        true_pos as usize
    } else {
        false_pos as usize
    }
}
fn return_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let return_reg = instr.get_return();

    let new_ip = stack.pop_as_ip();

    let value = registers.get(return_reg as usize);

    stack.push_val(value);

    runtime_information.pop_stack_height();

    new_ip
}
fn return_pop_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let (return_reg, pop_amount) = instr.get_return_pop();

    stack.decrement_stack_height(pop_amount as usize);

    let new_ip = stack.pop_as_ip();

    let value = registers.get(return_reg as usize);

    stack.push_val(value);

    runtime_information.pop_stack_height();

    new_ip
}
fn pop_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    _: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let pop_amount = instr.get_pop();

    stack.decrement_stack_height(pop_amount as usize);

    ip + 1
}
fn push_stack_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let reg = instr.get_push_stack() as usize;

    let value = registers.get(reg);

    stack.push_val(value.clone());

    *registers.get_mut(reg) = Some(value);

    ip + 1
}
fn load_stack_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    registers: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let (reg, is_relative, stack_pos) = instr.get_load_stack();

    let stack_pos = if is_relative {
        (stack_pos as usize) + runtime_information.get_stack_offset()
    } else {
        stack_pos as usize
    };

    let value = stack.get_as_val(stack_pos);
    let hei = stack.get_ref(stack_pos);

    *registers.get_mut(reg as usize) = Some(value);

    ip + 1
}
fn push_function_rule(
    (instr, ip): (&FastInstr, usize),
    _: &mut RuntimeInformation,
    _: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let instr_amount = instr.get_push_function() as usize;

    stack.push_ip(ip);

    ip + 1 + instr_amount
}
fn call_function_rule(
    (instr, ip): (&FastInstr, usize),
    runtime_information: &mut RuntimeInformation,
    _: &mut VMRegisters,
    stack: &mut VMStack
) -> usize {
    let stack_pos = instr.get_call_function();

    stack.push_ip(ip + 1);

    let function_ip = stack.get_as_ip(stack_pos as usize);

    runtime_information.push_stack_height(stack.len());

    function_ip + 1
}

const RULES: [
    fn((&FastInstr, usize), &mut RuntimeInformation, &mut VMRegisters, &mut VMStack) -> usize;
    31
] = [
    halt_rule,
    load_bool_rule,
    load_i32_rule,
    load_string_rule,
    add_rule,
    sub_rule,
    mul_rule,
    div_rule,
    neg_rule,
    not_rule,
    jmp_rule,
    jmp_pop_rule,
    cmp_eq_rule,
    cmp_neq_rule,
    cmp_gt_rule,
    cmp_geq_rule,
    cmp_lt_rule,
    cmp_leq_rule,
    cmp_eq_jmp_rule,
    cmp_neq_jmp_rule,
    cmp_gt_jmp_rule,
    cmp_geq_jmp_rule,
    cmp_lt_jmp_rule,
    cmp_leq_jmp_rule,
    return_rule,
    return_pop_rule,
    pop_rule,
    push_stack_rule,
    load_stack_rule,
    push_function_rule,
    call_function_rule,
];

pub struct Instructions2 {
    instructions: Rc<[FastInstr]>,
    ip: usize,
    stack_heights: Vec<usize>,
}

impl Instructions2 {
    pub fn from(instructions: Vec<FastInstr>) -> Self {
        Self {
            instructions: instructions.into(),
            ip: 0,
            stack_heights: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn pop_stack_height(&mut self) {
        self.stack_heights.pop();
    }

    pub fn push_stack_height(&mut self, height: usize) {
        self.stack_heights.push(height);
    }

    pub fn get_stack_offset(&self) -> usize {
        unsafe { *self.stack_heights.get_unchecked(self.stack_heights.len() - 1) - 1 }
    }

    pub fn get_next_instrs_range(&self, amount: usize) -> Vec<&FastInstr> {
        let start = self.ip + 1;
        let end = start + amount;
        let mut instrs_range = Vec::with_capacity(amount);

        for i in start..end {
            instrs_range.push(self.get_instr(i));
        }

        instrs_range
    }

    #[inline(always)]
    pub fn get_next_inst(&self) -> &FastInstr {
        &self.get_instr(self.ip)
    }

    #[inline(always)]
    fn get_instr(&self, pos: usize) -> &FastInstr {
        unsafe { self.instructions.get_unchecked(pos) }
    }

    // #[profiler::function_tracker]
    // pub fn execute(&mut self, registers: &mut VMRegisters, stack: &mut VMStack) {
    //     while self.ip < self.instructions.len() {
    //         let instr = self.get_next_inst();

    //         println!("IP({}), op: {:?}", self.ip, instr.get_op());

    //         self.ip = unsafe {
    //             RULES.get_unchecked(instr.get_op() as u8 as usize)(
    //                 instr,
    //                 &mut self,
    //                 registers,
    //                 stack
    //             )
    //         };
    //     }
    // }
}

pub struct VM2<'a> {
    program: &'a [FastInstr],
    constants_table: &'a ConstantsTable<'a>,
    ip: usize,
}

impl<'a> VM2<'a> {
    pub fn new(program: &'a [FastInstr], constants_table: &'a ConstantsTable) -> Self {
        Self {
            program,
            constants_table,
            ip: 0,
        }
    }

    #[profiler::function_tracker]
    fn execute(&mut self) {
        let mut registers = VMRegisters::new();
        let mut stack = VMStack::new();
        let mut runtime_information = RuntimeInformation::new();

        let now = Instant::now();

        while self.ip < self.program.len() {
            let instr = self.get_next_inst();

            self.ip = unsafe {
                RULES.get_unchecked(instr.get_op() as u8 as usize)(
                    (instr, self.ip),
                    &mut runtime_information,
                    &mut registers,
                    &mut stack
                )
            };
        }

        println!("elapsed: {:?}", now.elapsed());

        println!("{:#?}", stack)
    }

    pub fn run(&mut self) {
        // let mut registers = VMRegisters::new();
        // let mut stack = VMStack::new();

        // for _ in 0..1000000000 {
        self.execute();
        // }
    }

    pub fn get_next_instrs_range(&self, amount: usize) -> Vec<&FastInstr> {
        let start = self.ip + 1;
        let end = start + amount;
        let mut instrs_range = Vec::with_capacity(amount);

        for i in start..end {
            instrs_range.push(self.get_instr(i));
        }

        instrs_range
    }

    #[inline(always)]
    pub fn get_next_inst(&self) -> &FastInstr {
        &self.get_instr(self.ip)
    }

    #[inline(always)]
    fn get_instr(&self, pos: usize) -> &FastInstr {
        unsafe { self.program.get_unchecked(pos) }
    }
}
