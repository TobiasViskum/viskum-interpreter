mod tests;

use std::time::Instant;

// pub const NOP: u8 = 0;
pub const HALT: u8 = 0;

pub const LOAD_SHORT: u8 = 1;

pub const ADD_SHORT: u8 = 2;
pub const SUB_SHORT: u8 = 3;

pub const CMP_NO_STORE: u8 = 4;
pub const CMP_STORE: u8 = 5;
pub const JMP_LT: u8 = 6;

pub const PUSH_FN_PTR: u8 = 7;
pub const CALL_FN: u8 = 8;
pub const RET: u8 = 9;

pub const EMPTY_OPERAND: usize = 0;

type Bytecode = (Instruction, Instruction);
type Instruction = (u8, u16, u16, u16, u8);

fn main() {
    let constants_table = &[35, 2, 1];

    let fn_instructions: &Vec<Bytecode> = &vec![
        LoadShort::new(1, 1).to_bytecode(), // Load r1, [#1]
        CmpNoStore::new(0, 1).to_bytecode(),
        JmpIf::new(JMP_LT, 4, 5).to_bytecode(),
        Ret::new(0).to_bytecode(),
        BinaryShort::new(SUB_SHORT, 2, 0, 1).to_bytecode(),
        CallFn::new(3, 0, 3, 1).to_bytecode(),
        LoadShort::new(4, 2).to_bytecode(),
        BinaryShort::new(SUB_SHORT, 5, 0, 4).to_bytecode(),
        CallFn::new(6, 0, 6, 1).to_bytecode(),
        BinaryShort::new(ADD_SHORT, 7, 3, 6).to_bytecode(),
        Ret::new(7).to_bytecode() // Ret r3
    ];

    let mut instructions: Vec<Bytecode> = vec![
        PushFnPtr::new(0, fn_instructions.len()).to_bytecode() // Push fn
    ];
    instructions.extend(fn_instructions);

    instructions.extend(
        vec![
            LoadShort::new(1, 0).to_bytecode(),
            CallFn::new(2, 0, 2, 1).to_bytecode(), // Call 1 [0] ^1
            (HALT, 0, 0, 0) // Halt
        ]
    );

    let instructions_stack: [Bytecode; 15] = [
        PushFnPtr::new(0, fn_instructions.len()).to_bytecode(),
        LoadShort::new(1, 1).to_bytecode(), // Load r1, [#1]
        CmpNoStore::new(0, 1).to_bytecode(),
        JmpIf::new(JMP_LT, 4, 5).to_bytecode(),
        Ret::new(0).to_bytecode(),
        BinaryShort::new(SUB_SHORT, 2, 0, 1).to_bytecode(),
        CallFn::new(3, 0, 3, 1).to_bytecode(),
        LoadShort::new(4, 2).to_bytecode(),
        BinaryShort::new(SUB_SHORT, 5, 0, 4).to_bytecode(),
        CallFn::new(6, 0, 6, 1).to_bytecode(),
        BinaryShort::new(ADD_SHORT, 7, 3, 6).to_bytecode(),
        Ret::new(7).to_bytecode(),
        LoadShort::new(1, 0).to_bytecode(),
        CallFn::new(2, 0, 2, 1).to_bytecode(), // Call 1 [0] ^1
        (HALT, 0, 0, 0),
    ];

    let mut vm = VM::new(&instructions, constants_table);

    let now = Instant::now();
    vm.run();
    println!("{:?}", now.elapsed())
}

#[derive(Debug)]
pub struct VM<'a> {
    registers: [u64; u16::MAX as usize],
    constants_pool: &'a [u64],
    instructions: &'a [Bytecode],
    stack_frames: Vec<(usize, usize)>,
    outer_stack_frame: (usize, usize),
    ip: usize,
}

impl<'a> VM<'a> {
    pub fn new(instructions: &'a [Bytecode], constants_pool: &'a [u64]) -> Self {
        Self {
            instructions,
            constants_pool,
            stack_frames: Vec::new(),
            outer_stack_frame: (0, 0),
            registers: [0; u16::MAX as usize],
            ip: 0,
        }
    }

    fn push_stack_frame(&mut self, ip: usize, register_offset: usize) {
        let prev_reg_offset = self.outer_stack_frame.1;
        self.stack_frames.push(self.outer_stack_frame);
        self.outer_stack_frame = (ip, prev_reg_offset + register_offset);
    }

    fn pop_stack_frame_ip(&mut self) -> usize {
        let popped_ip = self.outer_stack_frame.0;
        self.outer_stack_frame = self.stack_frames.pop().unwrap();
        popped_ip
    }

    fn rel_reg(&self, reg: usize) -> u64 {
        self.registers[self.outer_stack_frame.1 + reg]
    }

    fn rel_mut_reg(&mut self, reg: usize) -> &mut u64 {
        let reg_offset = self.outer_stack_frame.1;
        self.registers.get_mut(reg_offset + reg).unwrap()
    }

    fn get_instruction(&self, ip: usize) -> Bytecode {
        self.instructions[ip]
    }

    fn fetch_and_execute_instruction(&mut self) {}

    fn execute(&mut self) {
        let (op, dst, reg1, reg2) = self.get_instruction(self.ip);

        match op {
            LOAD_SHORT => {
                *self.rel_mut_reg(dst) = self.constants_pool[reg1];
                self.inc_ip();
            }
            ADD_SHORT => {
                *self.rel_mut_reg(dst) = self.rel_reg(reg1) + self.rel_reg(reg2);
                self.inc_ip();
            }
            SUB_SHORT => {
                *self.rel_mut_reg(dst) = self.rel_reg(reg1) - self.rel_reg(reg2);
                self.inc_ip();
            }
            PUSH_FN_PTR => {
                *self.rel_mut_reg(dst) = (self.ip + 1) as u64;

                self.inc_ip_by(reg1 + 1);
            }
            CMP_NO_STORE => {
                let (jmp_op, _, true_pos, false_pos) = self.get_instruction(self.ip + 1);
                match jmp_op {
                    JMP_LT => {
                        if self.rel_reg(reg1) < self.rel_reg(reg2) {
                            self.set_ip(true_pos);
                        } else {
                            self.set_ip(false_pos);
                        }
                    }
                    _ => panic!("Expected JMP_IF"),
                }
            }
            RET => {
                let ret_val = self.rel_reg(dst);
                let new_ip = self.pop_stack_frame_ip();
                *self.rel_mut_reg(self.instructions[new_ip].1) = ret_val;
                self.set_ip(new_ip + 1);
            }
            CALL_FN => {
                self.push_stack_frame(self.ip, reg2);
                self.set_ip(self.registers[reg1] as usize);
            }
            HALT => { self.set_ip(self.instructions.len()) }
            c => {
                panic!("Undefined instruction: {}", c);
            }
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.execute();
        }
    }

    fn inc_ip(&mut self) {
        self.ip += 1;
    }

    fn inc_ip_by(&mut self, inc: usize) {
        self.ip += inc;
    }

    fn set_ip(&mut self, ip: usize) {
        self.ip = ip;
    }
}

struct PushFnPtr {
    dst: usize,
    instrs_amount: usize,
}
impl PushFnPtr {
    pub fn new(dst: usize, instrs_amount: usize) -> Self {
        Self { dst, instrs_amount }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (PUSH_FN_PTR, self.dst, self.instrs_amount, EMPTY_OPERAND)
    }
}

struct LoadShort {
    dst: usize,
    const_pos: usize,
}
impl LoadShort {
    pub fn new(dst: usize, const_pos: usize) -> Self {
        Self { dst, const_pos }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (LOAD_SHORT, self.dst, self.const_pos, EMPTY_OPERAND)
    }
}

struct BinaryShort {
    op: u8,
    dst: usize,
    reg1: usize,
    reg2: usize,
}
impl BinaryShort {
    pub fn new(op: u8, dst: usize, reg1: usize, reg2: usize) -> Self {
        if !matches!(op, ADD_SHORT | SUB_SHORT) {
            panic!("Invalid binary_short opcode");
        }

        Self { op, dst, reg1, reg2 }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (self.op, self.dst, self.reg1, self.reg2)
    }
}

struct Ret {
    return_reg: usize,
}
impl Ret {
    pub fn new(return_reg: usize) -> Self {
        Self { return_reg }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (RET, self.return_reg, EMPTY_OPERAND, EMPTY_OPERAND)
    }
}

struct CallFn {
    dst: usize,
    fn_pos: usize,
    reg_offset: usize,
    args_count: usize,
}
impl CallFn {
    pub fn new(dst: usize, fn_pos: usize, reg_offset: usize, args_count: usize) -> Self {
        Self { dst, fn_pos, reg_offset, args_count }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (CALL_FN, self.dst, self.fn_pos, self.reg_offset - self.args_count)
    }
}

struct CmpNoStore {
    reg1: usize,
    reg2: usize,
}
impl CmpNoStore {
    pub fn new(reg1: usize, reg2: usize) -> Self {
        Self { reg1, reg2 }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (CMP_NO_STORE, EMPTY_OPERAND, self.reg1, self.reg2)
    }
}

struct JmpIf {
    op: u8,
    true_pos: usize,
    false_pos: usize,
}
impl JmpIf {
    pub fn new(op: u8, true_pos: usize, false_pos: usize) -> Self {
        if !matches!(op, JMP_LT) {
            panic!("Invalid jmp_if opcode");
        }

        Self { op, true_pos, false_pos }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (self.op, EMPTY_OPERAND, self.true_pos, self.false_pos)
    }
}

fn pack_bools(bool1: bool, bool2: bool, bool3: bool) -> u8 {
    let mut packed = 0;
    if bool1 {
        packed |= 0b0000_0001;
    }
    if bool2 {
        packed |= 0b0000_0010;
    }
    if bool3 {
        packed |= 0b0000_0100;
    }
    packed
}

// Function to unpack three boolean values from a single u8
fn unpack_bools(byte: u8) -> (bool, bool, bool) {
    let bool1 = (byte & 0b0000_0001) != 0;
    let bool2 = (byte & 0b0000_0010) != 0;
    let bool3 = (byte & 0b0000_0100) != 0;
    (bool1, bool2, bool3)
}
