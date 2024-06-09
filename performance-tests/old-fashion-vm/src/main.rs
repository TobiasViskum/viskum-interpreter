mod tests;
mod convert_to_optimized;

use std::{ collections::BinaryHeap, rc::Rc, thread, time::Instant };
use ahash::AHashMap;

mod compiled_instructions;
mod instructions;

use compiled_instructions::do_test;

// pub const NOP: u8 = 0;
pub const HALT: u8 = 0;

pub const LOAD_CONST_ONE: u8 = 1;
pub const LOAD_CONST_TWO: u8 = 2;

// Binary instructions
// R = Relative reg pos, A = Absolute reg pos
// E.g. RRR = dst: Relative, reg1: Relative, reg2: Relative
pub const ADD_NUM_RRR: u8 = 3;
// pub const ADD_NUM_RAR: u8 = 3;
// pub const ADD_NUM_RRA: u8 = 4;
// pub const ADD_NUM_AAA: u8 = 5;
// pub const ADD_NUM_AAR: u8 = 6;
// pub const ADD_NUM_ARA: u8 = 7;

pub const SUB_NUM_RRR: u8 = 4;
// pub const SUB_NUM_RAR: u8 = 9;
// pub const SUB_NUM_RRA: u8 = 10;
// pub const SUB_NUM_AAA: u8 = 11;
// pub const SUB_NUM_AAR: u8 = 12;
// pub const SUB_NUM_ARA: u8 = 13;

pub const CMP_NO_STORE: u8 = 5;
pub const CMP_STORE: u8 = 6;
pub const JMP_LT: u8 = 7;

pub const PUSH_FN_PTR: u8 = 8;
// pub const CALL_FN_REL: u8 = 8;
pub const CALL_FN_ABS: u8 = 9;
pub const RET: u8 = 10;
pub const LOAD_COMPLEX_CONST_ONE: u8 = 11;
pub const LOAD_COMPLEX_CONST_TWO: u8 = 12;
pub const STRING_CONCAT_FORGET: u8 = 13;

pub const EMPTY_OPERAND: usize = 0;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Opcode {
    Halt = 0,
    LoadConstOne = 1,
    LoadConstTwo = 2,
    AddNum = 3,
    SubNum = 4,
    CmpNoStore = 5,
    CmpStore = 6,
    JmpLt = 7,
    PushFnPtr = 8,
    CallFnAbs = 9,
    Ret = 10,
    LoadComplexConstOne = 11,
    LoadComplexConstTwo = 12,
    StringConcatForget = 13,
}

type Bytecode = (Opcode, usize, usize, usize);

fn main() {
    do_test()

    // let constants_table = &[35, 2, 1];
    // let complex_constants_pool = &[HeapValue::new_string("Hello"), HeapValue::new_string(" World")];

    // let fn_instructions = &vec![
    //     LoadTwo::new(LoadType::Simple, (1, 1), (2, 2)).to_bytecode(),
    //     JmpIf::new(Opcode::JmpLt, 0, 1, 3, 4).to_bytecode(),
    //     Ret::new(0).to_bytecode(),
    //     BinaryShort::new(Opcode::SubNum, 3, 0, 1).to_bytecode(),
    //     CallFn::new(Opcode::CallFnAbs, 4, 0, (4, 1)).to_bytecode(),
    //     BinaryShort::new(Opcode::SubNum, 5, 0, 2).to_bytecode(),
    //     CallFn::new(Opcode::CallFnAbs, 6, 0, (6, 1)).to_bytecode(),
    //     BinaryShort::new(Opcode::AddNum, 7, 4, 6).to_bytecode(),
    //     Ret::new(7).to_bytecode()
    // ];

    // let mut instructions = vec![PushFnPtr::new(0, fn_instructions.len()).to_bytecode()];
    // instructions.extend(fn_instructions);

    // instructions.extend(
    //     vec![
    //         LoadOne::new(LoadType::Simple, (1, 0)).to_bytecode(),
    //         CallFn::new(Opcode::CallFnAbs, 2, 0, (2, 1)).to_bytecode(),
    //         // LoadTwo::new(LoadType::Complex, (0, 0), (1, 1)).to_bytecode(),
    //         // (STRING_CONCAT_FORGET, 2, 0, 1),
    //         (Opcode::Halt, 0, 0, 0)
    //     ]
    // );
    // let mut vm = VM::new(&instructions, constants_table, complex_constants_pool);

    // let now = Instant::now();
    // vm.run();
    // println!("{:?}", now.elapsed())
}

#[derive(Debug, Clone)]
pub enum HeapValue {
    String(Vec<u8>),
}

impl HeapValue {
    pub fn new_string(str: &str) -> Self {
        Self::String(str.as_bytes().to_vec())
    }

    pub fn unwrap_as_string_bytes(&self) -> &Vec<u8> {
        match self {
            Self::String(string) => string,
        }
    }
}

#[derive(Debug)]
pub struct HeapReferences {
    references: usize,
}

impl HeapReferences {
    pub fn new() -> Self {
        Self {
            references: 1,
        }
    }

    pub fn inc(&mut self) {
        self.references += 1;
    }

    pub fn dec(&mut self) {
        self.references -= 1;
    }
}

#[derive(Debug)]
pub struct Heap {
    heap: AHashMap<usize, (HeapValue, HeapReferences)>,
}
impl Heap {
    pub fn new() -> Self {
        Self {
            heap: AHashMap::new(),
        }
    }

    pub fn get(&self, idx: &usize) -> &HeapValue {
        &self.heap.get(idx).unwrap().0
    }

    pub fn clean_up(&mut self, idx1: &usize, idx2: &usize) {
        let v1 = self.heap.get_mut(idx1).unwrap();
        v1.1.dec();
        if v1.1.references == 0 {
            self.heap.remove(idx1);
        }

        let v2 = self.heap.get_mut(idx2).unwrap();
        v2.1.dec();
        if v2.1.references == 0 {
            self.heap.remove(idx2);
        }
    }

    pub fn insert(&mut self, idx: usize, heap_value: HeapValue) {
        self.heap.insert(idx, (heap_value, HeapReferences::new()));
    }

    pub fn insert_string(&mut self, idx: usize, heap_value: Vec<u8>) {
        self.heap.insert(idx, (HeapValue::String(heap_value), HeapReferences::new()));
    }
}

pub struct VM<'a> {
    registers: [u64; u16::MAX as usize],
    constants_pool: &'a [u64],
    complex_constants_pool: &'a [HeapValue],
    instructions: &'a [Bytecode],
    stack_frames: Vec<(usize, usize)>, // Replace (usize, usize) with struct
    heap: Heap,
    outer_stack_frame: (usize, usize), // Make this its own struct
    ip: usize,
}

impl<'a> VM<'a> {
    pub fn new(
        instructions: &'a [Bytecode],
        constants_pool: &'a [u64],
        complex_constants_pool: &'a [HeapValue]
    ) -> Self {
        Self {
            instructions,
            constants_pool,
            complex_constants_pool,
            stack_frames: Vec::new(),
            outer_stack_frame: (0, 0),
            registers: [0; u16::MAX as usize],
            heap: Heap::new(),
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

    fn execute(&mut self) {
        let (op, dst, reg1, reg2) = self.instructions[self.ip];

        match op {
            Opcode::LoadComplexConstOne => {
                let (dst_pos, dst_const_pos) = (dst & 0xffff, (dst >> 16) & 0xffff);
                let heap_value = self.complex_constants_pool[dst_const_pos].clone();
                self.heap.insert(dst_pos, heap_value);

                self.inc_ip();
            }

            Opcode::LoadComplexConstTwo => {
                let (dst_pos, dst_const_pos) = (dst & 0xffff, (dst >> 16) & 0xffff);
                let heap_value = self.complex_constants_pool[dst_const_pos].clone();
                self.heap.insert(dst_pos, heap_value);

                let (reg1_pos, reg1_const_pos) = (reg1 & 0xffff, (reg1 >> 16) & 0xffff);
                let heap_value = self.complex_constants_pool[reg1_const_pos].clone();
                self.heap.insert(reg1_pos, heap_value);

                self.inc_ip();
            }
            Opcode::StringConcatForget => {
                let (lhs, rhs) = (self.heap.get(&reg1), self.heap.get(&reg2));
                let (lhs, rhs) = (lhs.unwrap_as_string_bytes(), rhs.unwrap_as_string_bytes());

                let new_str = vec![]
                    .into_iter()
                    .chain(lhs.into_iter().cloned())
                    .chain(rhs.into_iter().cloned())
                    .collect::<Vec<_>>();

                self.heap.insert_string(dst, new_str);

                self.heap.clean_up(&reg1, &reg2);

                self.inc_ip();
            }
            Opcode::LoadConstOne => {
                let (dst_pos, dst_const_pos) = (dst & 0xffff, (dst >> 16) & 0xffff);
                *self.rel_mut_reg(dst_pos) = self.constants_pool[dst_const_pos];
                self.inc_ip();
            }
            Opcode::LoadConstTwo => {
                let (dst_pos, dst_const_pos) = (dst & 0xffff, (dst >> 16) & 0xffff);
                *self.rel_mut_reg(dst_pos) = self.constants_pool[dst_const_pos];

                let (reg1_pos, reg1_const_pos) = (reg1 & 0xffff, (reg1 >> 16) & 0xffff);
                *self.rel_mut_reg(reg1_pos) = self.constants_pool[reg1_const_pos];

                self.inc_ip();
            }
            Opcode::AddNum => {
                *self.rel_mut_reg(dst) = self.rel_reg(reg1) + self.rel_reg(reg2);
                self.inc_ip();
            }
            Opcode::SubNum => {
                *self.rel_mut_reg(dst) = self.rel_reg(reg1) - self.rel_reg(reg2);
                self.inc_ip();
            }
            Opcode::PushFnPtr => {
                *self.rel_mut_reg(dst) = (self.ip + 1) as u64;

                self.inc_ip_by(reg1 + 1);
            }
            Opcode::JmpLt => {
                let (cmp_reg1, cmp_reg2) = (dst & 0xffff, (dst >> 16) & 0xffff);
                if self.rel_reg(cmp_reg1) < self.rel_reg(cmp_reg2) {
                    self.set_ip(reg1);
                } else {
                    self.set_ip(reg2);
                }
            }
            Opcode::Ret => {
                let ret_val = self.rel_reg(dst);
                let new_ip = self.pop_stack_frame_ip();
                *self.rel_mut_reg(self.instructions[new_ip].1) = ret_val;
                self.set_ip(new_ip + 1);
            }
            Opcode::CallFnAbs => {
                self.push_stack_frame(self.ip, reg2);
                self.set_ip(self.registers[reg1] as usize);
            }
            Opcode::Halt => {
                // println!("{:#?}", self.heap);
                self.set_ip(self.instructions.len());
            }
            c => {
                panic!("Undefined instruction: {:?}", c);
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
        (Opcode::PushFnPtr, self.dst, self.instrs_amount, EMPTY_OPERAND)
    }
}

enum LoadType {
    Simple,
    Complex,
}

struct LoadOne {
    load_type: LoadType,
    dst: u16,
    const_pos: u16,
}
impl LoadOne {
    pub fn new(load_type: LoadType, (dst, const_pos): (u16, u16)) -> Self {
        Self { load_type, dst, const_pos }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        let packed_1 = (self.dst as usize) | ((self.const_pos as usize) << 16);

        let op = match self.load_type {
            LoadType::Simple => Opcode::LoadConstOne,
            LoadType::Complex => Opcode::LoadComplexConstOne,
        };

        (op, packed_1, EMPTY_OPERAND, EMPTY_OPERAND)
    }
}

struct LoadTwo {
    load_type: LoadType,
    dst: u16,
    dst_const_pos: u16,
    reg1: u16,
    reg1_const_pos: u16,
}
impl LoadTwo {
    pub fn new(
        load_type: LoadType,
        (dst, dst_const_pos): (u16, u16),
        (reg1, reg1_const_pos): (u16, u16)
    ) -> Self {
        Self { load_type, dst, dst_const_pos, reg1, reg1_const_pos }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        let packed_1 = (self.dst as usize) | ((self.dst_const_pos as usize) << 16);
        let packed_2 = (self.reg1 as usize) | ((self.reg1_const_pos as usize) << 16);

        let op = match self.load_type {
            LoadType::Simple => Opcode::LoadConstTwo,
            LoadType::Complex => Opcode::LoadComplexConstTwo,
        };

        (op, packed_1, packed_2, EMPTY_OPERAND)
    }
}

struct BinaryShort {
    op: Opcode,
    dst: usize,
    reg1: usize,
    reg2: usize,
}
impl BinaryShort {
    pub fn new(op: Opcode, dst: usize, reg1: usize, reg2: usize) -> Self {
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
        (Opcode::Ret, self.return_reg, EMPTY_OPERAND, EMPTY_OPERAND)
    }
}

struct CallFn {
    op: Opcode,
    dst: usize,
    fn_pos: usize,
    reg_offset: usize,
    args_count: usize,
}
impl CallFn {
    pub fn new(
        op: Opcode,
        dst: usize,
        fn_pos: usize,
        (reg_offset, args_count): (usize, usize)
    ) -> Self {
        Self { op, dst, fn_pos, reg_offset, args_count }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        (self.op, self.dst, self.fn_pos, self.reg_offset - self.args_count)
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
        (Opcode::CmpNoStore, EMPTY_OPERAND, self.reg1, self.reg2)
    }
}

struct JmpIf {
    op: Opcode,
    reg1: u16,
    reg2: u16,
    true_pos: usize,
    false_pos: usize,
}
impl JmpIf {
    pub fn new(op: Opcode, reg1: u16, reg2: u16, true_pos: usize, false_pos: usize) -> Self {
        Self { op, true_pos, false_pos, reg1, reg2 }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        let packed_1 = (self.reg1 as usize) | ((self.reg2 as usize) << 16);

        (self.op, packed_1, self.true_pos, self.false_pos)
    }
}

fn pack_3_bools(bool1: bool, bool2: bool, bool3: bool) -> u8 {
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
fn unpack_3_bools(byte: u8) -> (bool, bool, bool) {
    let bool1 = (byte & 0b0000_0001) != 0;
    let bool2 = (byte & 0b0000_0010) != 0;
    let bool3 = (byte & 0b0000_0100) != 0;
    (bool1, bool2, bool3)
}

fn pack_2_bools(bool1: bool, bool2: bool) -> u8 {
    let mut packed = 0;
    if bool1 {
        packed |= 0b0000_0001;
    }
    if bool2 {
        packed |= 0b0000_0010;
    }
    packed
}

// Function to unpack three boolean values from a single u8
fn unpack_2_bools(byte: u8) -> (bool, bool) {
    let bool1 = (byte & 0b0000_0001) != 0;
    let bool2 = (byte & 0b0000_0010) != 0;

    (bool1, bool2)
}
