use std::time::Instant;

use ahash::AHashMap;

use crate::{ convert_to_optimized::VM2, instructions::{ Instruction, Register } };

pub union ExtraParam {
    pub const_pos: usize,
    pub register_offset: usize,
    pub reg_pos: usize,
    pub jmp: *const CompiledInstruction,
    pub __: bool,
}
pub struct CompiledInstruction {
    pub handler: unsafe fn(*const Self, &mut VM2),
    pub param1: ExtraParam,
    pub param2: ExtraParam,
    pub param3: ExtraParam,
}

macro_rules! def_op {
    ($name:ident($instruction:ident, $vm:ident) $code:block) => {
        #[inline(always)]
        pub unsafe fn $name($instruction: *const CompiledInstruction, $vm: &mut VM2) {
            $code
        }
    };
}

macro_rules! get_rel_reg {
    ($vm:ident.$registers:ident [$r:expr]) => {
        *($vm.$registers.get_unchecked($r + $vm.read_register_offset()))
    };
    (mut $vm:ident.$registers:ident [$r:expr]) => {
        *{
        let register_offset = $vm.read_register_offset();
        ($vm.$registers.get_unchecked_mut($r + register_offset))
        }
    };
}

macro_rules! get_const {
    ($vm:ident.$constants_table:ident [$r:expr]) => {
        *($vm.$constants_table.get_unchecked($r))
    };
}

macro_rules! next_op {
    ($instruction:expr, $vm:ident) => {
        ((*($instruction)).handler)($instruction, $vm)
    };
}

def_op!(push_fn_ptr(instruction, vm) {
    next_op!((*instruction).param2.jmp, vm);
});

def_op!(call_abs(instruction, vm) {
    vm.push_stack_frame(instruction, instruction.offset(1), (*instruction).param3.register_offset);
    next_op!((*instruction).param2.jmp, vm)
});

def_op!(load_one_simple(instruction, vm) {
    get_rel_reg!(mut vm.registers[(*instruction).param1.reg_pos]) = get_const!(vm.constants_pool[(*instruction).param2.const_pos]);
    next_op!(instruction.offset(1), vm)
});

def_op!(load_two_simple(instruction, vm) {
    let dst = (*instruction).param1.reg_pos;
    let (reg1_pos, const1_pos) = (dst & 0xffff, (dst >> 16) & 0xffff);
    (get_rel_reg!(mut vm.registers[reg1_pos]) = get_const!(vm.constants_pool[const1_pos]));

    let reg1 = (*instruction).param2.reg_pos;
    let (reg2_pos, const2_pos) = (reg1 & 0xffff, (reg1 >> 16) & 0xffff);
    get_rel_reg!(mut vm.registers[reg2_pos]) = get_const!(vm.constants_pool[const2_pos]);

    next_op!(instruction.offset(1), vm)
});

def_op!(jmp_lt(instruction, vm) {
    let dst = (*instruction).param1.reg_pos;
    let (reg1, reg2) = (dst & 0xffff, (dst >> 16) & 0xffff);
    match get_rel_reg!(vm.registers[reg1]) < get_rel_reg!(vm.registers[reg2]) {
        true => next_op!((*instruction).param2.jmp, vm),
        false => next_op!((*instruction).param3.jmp, vm),
    }
});

def_op!(ret(instruction, vm) {
    let ret_val = get_rel_reg!(vm.registers[(*instruction).param1.reg_pos]);
    let (call_instruction_ptr, next_instruction_ptr) = vm.pop_stack_frame_as_ptr();
    get_rel_reg!(mut vm.registers[(*call_instruction_ptr).param1.reg_pos]) = ret_val;
    next_op!(next_instruction_ptr, vm)
});

def_op!(sub(instruction, vm) {
    let (v1, v2) = (get_rel_reg!(vm.registers[(*instruction).param2.reg_pos]), get_rel_reg!(vm.registers[(*instruction).param3.reg_pos]));
    get_rel_reg!(mut vm.registers[(*instruction).param1.reg_pos]) = v1 - v2;
    next_op!(instruction.offset(1), vm)
});

def_op!(add(instruction, vm) {
    let (v1, v2) = (get_rel_reg!(vm.registers[(*instruction).param2.reg_pos]), get_rel_reg!(vm.registers[(*instruction).param3.reg_pos]));
    get_rel_reg!(mut vm.registers[(*instruction).param1.reg_pos]) = v1 + v2;
    next_op!(instruction.offset(1), vm)
});

def_op!(halt(instruction, vm) {});

pub fn do_test() {
    let instructions = vec![
        Instruction::StoreFnPtr { instructions_amount: 9 },
        Instruction::LoadConstTwo {
            reg1: Register::new(1),
            const_pos1: 1,
            reg2: Register::new(2),
            const_pos2: 2,
        },
        Instruction::JmpLt {
            reg1: Register::new(0),
            reg2: Register::new(1),
            true_pos: 3,
            false_pos: 4,
        },
        Instruction::Ret { reg: Register::new(0) },
        Instruction::Sub { dst: Register::new(3), reg1: Register::new(0), reg2: Register::new(1) },
        Instruction::Call { dst: Register::new(4), args_count: 1 },
        Instruction::Sub { dst: Register::new(5), reg1: Register::new(0), reg2: Register::new(2) },
        Instruction::Call { dst: Register::new(6), args_count: 1 },
        Instruction::Add { dst: Register::new(7), reg1: Register::new(6), reg2: Register::new(4) },
        Instruction::Ret { reg: Register::new(7) },
        Instruction::Sub { dst: Register::new(3), reg1: Register::new(0), reg2: Register::new(1) },
        Instruction::LoadConstOne { reg: Register::new(1), const_pos: 0 },
        Instruction::Call { dst: Register::new(2), args_count: 1 },
        Instruction::Halt
    ];

    let mut instructions = [
        // Push function (which doesn't store anything in registers)
        CompiledInstruction {
            handler: push_fn_ptr,
            param1: ExtraParam { reg_pos: 0 },
            param2: ExtraParam { jmp: std::ptr::null() },
            param3: ExtraParam { __: false },
        },
        /* FUNCTION */
        // Load 2
        CompiledInstruction {
            handler: load_two_simple,
            param1: ExtraParam { reg_pos: (1 as usize) | ((1 as usize) << 16) },
            param2: ExtraParam { reg_pos: (2 as usize) | ((2 as usize) << 16) },
            param3: ExtraParam { __: false },
        },
        // Compares n and 2. Return to Ret if true, else the instruction after
        CompiledInstruction {
            handler: jmp_lt,
            param1: ExtraParam { reg_pos: (0 as usize) | ((1 as usize) << 16) },
            param2: ExtraParam { jmp: std::ptr::null() },
            param3: ExtraParam { jmp: std::ptr::null() },
        },
        // Returns n
        CompiledInstruction {
            handler: ret,
            param1: ExtraParam { reg_pos: 0 },
            param2: ExtraParam { __: false },
            param3: ExtraParam { __: false },
        },
        // Sub: Calculates n - 2
        CompiledInstruction {
            handler: sub,
            param1: ExtraParam { reg_pos: 3 },
            param2: ExtraParam { reg_pos: 0 },
            param3: ExtraParam { reg_pos: 1 },
        },
        // Calls fib(n - 2)
        CompiledInstruction {
            handler: call_abs,
            param1: ExtraParam { reg_pos: 4 },
            param2: ExtraParam { jmp: std::ptr::null() },
            param3: ExtraParam { register_offset: 4 - 1 }, // regoffset - args count
        },
        CompiledInstruction {
            handler: sub,
            param1: ExtraParam { reg_pos: 5 },
            param2: ExtraParam { reg_pos: 0 },
            param3: ExtraParam { reg_pos: 2 },
        },
        // Calls fib(n - 1)
        CompiledInstruction {
            handler: call_abs,
            param1: ExtraParam { reg_pos: 6 },
            param2: ExtraParam { jmp: std::ptr::null() },
            param3: ExtraParam { register_offset: 6 - 1 }, // regoffset - args count
        },
        // Add: Calculates fib(n - 2) + fib(n - 1)
        CompiledInstruction {
            handler: add,
            param1: ExtraParam { reg_pos: 7 },
            param2: ExtraParam { reg_pos: 4 },
            param3: ExtraParam { reg_pos: 6 },
        },
        // Returns the result of the above expression
        CompiledInstruction {
            handler: ret,
            param1: ExtraParam { reg_pos: 7 },
            param2: ExtraParam { __: false },
            param3: ExtraParam { __: false },
        },

        // Loads the argument n = 35
        CompiledInstruction {
            handler: load_one_simple,
            param1: ExtraParam { reg_pos: 1 },
            param2: ExtraParam { const_pos: 0 },
            param3: ExtraParam { __: false },
        },
        // Calls the function
        CompiledInstruction {
            handler: call_abs,
            param1: ExtraParam { reg_pos: 2 },
            param2: ExtraParam { jmp: std::ptr::null() },
            param3: ExtraParam { register_offset: 2 - 1 }, // regoffset - args count
        },
        // Terminates the program
        CompiledInstruction {
            handler: halt,
            param1: ExtraParam { __: false },
            param2: ExtraParam { __: false },
            param3: ExtraParam { __: false },
        },
    ];

    instructions[0].param2 = ExtraParam { jmp: &instructions[10] as *const CompiledInstruction };
    instructions[2].param2 = ExtraParam { jmp: &instructions[3] as *const CompiledInstruction };
    instructions[2].param3 = ExtraParam { jmp: &instructions[4] as *const CompiledInstruction };
    instructions[5].param2 = ExtraParam { jmp: &instructions[1] as *const CompiledInstruction };
    instructions[7].param2 = ExtraParam { jmp: &instructions[1] as *const CompiledInstruction };
    instructions[11].param2 = ExtraParam { jmp: &instructions[1] as *const CompiledInstruction };

    let constants_pool = &[35, 2, 1];

    let instructions: &[CompiledInstruction] = &instructions;

    let mut vm = VM2::new(constants_pool, &[]);

    unsafe {
        let now = Instant::now();
        (instructions[0].handler)(instructions.as_ptr(), &mut vm);
        println!("{:?}", now.elapsed());
    }

    // println!("{:?}", vm);
}

enum Param {
    Param0,
    Param1,
    Param2,
}

fn compile_instructions(instructions: &[Instruction]) -> Vec<CompiledInstruction> {
    let mut instruction_id_map: AHashMap<usize, Vec<(Param, usize)>> = AHashMap::new();

    let mut compiled_instructions = vec![];

    for inst in instructions {
        let compiled_inst = match inst {
            Instruction::Halt => {
                CompiledInstruction {
                    handler: halt,
                    param1: ExtraParam { __: false },
                    param2: ExtraParam { __: false },
                    param3: ExtraParam { __: false },
                }
            }
            Instruction::Add { dst, reg1, reg2 } => {
                CompiledInstruction {
                    handler: add,
                    param1: ExtraParam { reg_pos: dst.get_reg() },
                    param2: ExtraParam { reg_pos: reg1.get_reg() },
                    param3: ExtraParam { reg_pos: reg2.get_reg() },
                }
            }
            Instruction::Sub { dst, reg1, reg2 } => {
                CompiledInstruction {
                    handler: sub,
                    param1: ExtraParam { reg_pos: dst.get_reg() },
                    param2: ExtraParam { reg_pos: reg1.get_reg() },
                    param3: ExtraParam { reg_pos: reg2.get_reg() },
                }
            }
            Instruction::LoadConstOne { reg, const_pos } => {
                CompiledInstruction {
                    handler: load_one_simple,
                    param1: ExtraParam { reg_pos: reg.get_reg() },
                    param2: ExtraParam { const_pos: *const_pos },
                    param3: ExtraParam { __: false },
                }
            }
            Instruction::LoadConstTwo { reg1, const_pos1, reg2, const_pos2 } => {
                CompiledInstruction {
                    handler: load_two_simple,
                    param1: ExtraParam { reg_pos: reg1.get_reg() | (*const_pos1 << 16) },
                    param2: ExtraParam { reg_pos: reg2.get_reg() | (*const_pos2 << 16) },
                    param3: ExtraParam { __: false },
                }
            }
        };

        compiled_instructions.push(compiled_inst);
    }

    compiled_instructions
}
