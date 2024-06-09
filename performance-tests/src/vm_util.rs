use std::mem::size_of;

use crate::Registers;

#[derive(Debug, Clone, Copy)]
pub enum Dst {
    Register(Register),
    StackPush,
}

impl Dst {
    pub fn dissasemble(&self) -> String {
        match self {
            Self::Register(reg) => reg.dissasemble(),
            Self::StackPush => "+[]".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl Register {
    pub fn dissasemble(&self) -> String {
        (
            match self {
                Self::R0 => "R0",
                Self::R1 => "R1",
                Self::R2 => "R2",
                Self::R3 => "R3",
            }
        ).to_string()
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct ConstPtr(usize);

impl ConstPtr {
    pub fn new(ptr: usize) -> Self {
        Self(ptr)
    }

    pub fn get_idx(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct StackPtr {
    idx: usize,
    is_relative: bool,
}

impl StackPtr {
    pub fn new(idx: usize, is_relative: bool) -> Self {
        Self { idx, is_relative }
    }

    #[inline]
    pub fn get_idx(&self) -> usize {
        self.idx
    }

    #[inline]
    pub fn get_is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn dissasemble(&self, stack_frame: Option<usize>) -> String {
        match self.is_relative {
            true =>
                match stack_frame {
                    Some(sf) => format!("[[ {} ]]", self.idx + sf),
                    None => format!("[{} + n]", self.idx),
                }
            false => format!("[{}]", self.idx),
        }
    }
}

// #[derive(Debug, Clone, Copy)]
// pub struct ConstantPointer(usize);

// impl ConstantPointer {
//     pub fn new(ptr: usize) -> Self {
//         Self(ptr)
//     }
// }

#[derive(Debug, Clone, Copy, Hash)]
pub enum Src {
    Register(Register),
    ConstantPointer(usize),
    StackPointer(StackPtr),
}

impl Src {
    #[inline]
    pub(crate) fn get_val<'b>(
        &'b self,
        constants_pool: &'b [Value],
        stack: &'b Stack,
        registers: &'b Registers
    ) -> &'b Value {
        match self {
            Self::ConstantPointer(idx) => unsafe { constants_pool.get_unchecked(*idx) }
            Self::StackPointer(stack_ptr) => stack.get(stack_ptr).unwrap_val(),
            Self::Register(reg) => registers.get_reg(*reg).get_val(constants_pool),
        }
    }

    pub fn dissasemble(&self, stack_frame: Option<usize>) -> String {
        match self {
            Self::Register(reg) => reg.dissasemble(),
            Self::ConstantPointer(idx) => format!("%{}", idx),
            Self::StackPointer(stack_ptr) => stack_ptr.dissasemble(stack_frame),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int32(i32),
    Int64(i64),
    Bool(bool),
}

impl Value {
    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => Self::Int32(lhs + rhs),
            (Self::Int64(lhs), Self::Int64(rhs)) => Self::Int64(lhs + rhs),
            _ => panic!("Addition not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => Self::Int32(lhs - rhs),
            (Self::Int64(lhs), Self::Int64(rhs)) => Self::Int64(lhs - rhs),
            _ => panic!("Subtraction not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => Self::Int32(lhs * rhs),
            (Self::Int64(lhs), Self::Int64(rhs)) => Self::Int64(lhs * rhs),
            _ => panic!("Mul not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn div(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => Self::Int32(lhs / rhs),
            (Self::Int64(lhs), Self::Int64(rhs)) => Self::Int64(lhs / rhs),
            _ => panic!("Div not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn cmp_e(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => lhs == rhs,
            (Self::Int64(lhs), Self::Int64(rhs)) => lhs == rhs,
            _ => panic!("CmpLt not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn cmp_ne(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => lhs != rhs,
            (Self::Int64(lhs), Self::Int64(rhs)) => lhs != rhs,
            _ => panic!("CmpLt not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn cmp_l(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => lhs < rhs,
            (Self::Int64(lhs), Self::Int64(rhs)) => lhs < rhs,
            _ => panic!("CmpLt not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn cmp_le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => lhs <= rhs,
            (Self::Int64(lhs), Self::Int64(rhs)) => lhs <= rhs,
            _ => panic!("CmpLt not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn cmp_g(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => lhs > rhs,
            (Self::Int64(lhs), Self::Int64(rhs)) => lhs > rhs,
            _ => panic!("CmpLt not defined for {:?} and {:?}", self, other),
        }
    }

    #[inline]
    pub fn cmp_ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => lhs >= rhs,
            (Self::Int64(lhs), Self::Int64(rhs)) => lhs >= rhs,
            _ => panic!("CmpLt not defined for {:?} and {:?}", self, other),
        }
    }
}

#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<StackValue>,
    stack_frames: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new(), stack_frames: Vec::new() }
    }

    #[inline]
    pub fn get_val_from_parts(&self, idx: usize, is_relative: bool) -> &Value {
        match is_relative {
            true => unsafe { self.stack.get_unchecked(idx + self.read_stack_frame()).unwrap_val() }
            false => unsafe { self.stack.get_unchecked(idx).unwrap_val() }
        }
    }

    #[inline]
    pub fn get(&self, stack_ptr: &StackPtr) -> &StackValue {
        match stack_ptr.get_is_relative() {
            true => unsafe {
                self.stack.get_unchecked(stack_ptr.get_idx() + self.read_stack_frame())
            }
            false => unsafe { self.stack.get_unchecked(stack_ptr.get_idx()) }
        }
    }

    #[inline]
    pub fn push(&mut self, stack_value: StackValue) {
        self.stack.push(stack_value);
    }

    #[inline]
    pub fn pop(&mut self) -> StackValue {
        unsafe { self.stack.pop().unwrap_unchecked() }
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.stack.truncate(new_len)
    }

    #[inline]
    pub fn push_stack_frame(&mut self, frame_begin: usize) {
        self.stack_frames.push(frame_begin);
    }

    #[inline]
    pub fn read_stack_frame(&self) -> usize {
        unsafe { *self.stack_frames.get_unchecked(self.stack_frames.len() - 1) }
    }

    #[inline]
    pub fn pop_stack_frame(&mut self) {
        self.stack_frames.pop();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    // Not used in code for performance
    pub fn try_read_stack_frame(&self) -> Option<usize> {
        self.stack_frames.last().copied()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StackValue {
    FnPtr(usize),
    Value(Value),
}

impl StackValue {
    #[inline]
    pub fn unwrap_val(&self) -> &Value {
        assert_eq!(16, size_of::<Self>());
        unsafe { std::mem::transmute::<&Self, &Value>(self) }
    }

    #[inline]
    pub fn unwrap_fn_ptr(&self) -> usize {
        (unsafe { std::mem::transmute::<&Self, &u128>(self) >> 64 }) as usize
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterValue {
    ConstantPointer(usize),
    Value(Value),
}

impl RegisterValue {
    #[inline]
    pub fn get_val<'b>(&'b self, constants_pool: &'b [Value]) -> &'b Value {
        match self {
            Self::Value(v) => v,
            Self::ConstantPointer(idx) => unsafe { constants_pool.get_unchecked(*idx) }
        }
    }
}
