use std::fmt::Debug;

use ahash::AHashMap;

use crate::compiled_instructions::CompiledInstruction;

const MAX_CALL_DEPTH: usize = 65536;

pub struct VM2<'a> {
    pub registers: [u64; u16::MAX as usize],
    pub constants_pool: &'a [u64],
    pub complex_constants_pool: &'a [HeapValue],
    pub stack_frames: [StackFrame; MAX_CALL_DEPTH],
    pub outer_stack_frame: *const StackFrame,
    pub stack_frame_depth: usize,
    pub heap: Heap,
}

impl<'a> Debug for VM2<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let regs = self.registers
            .into_iter()
            .filter(|r| *r != 0)
            .collect::<Vec<_>>();

        write!(f, "{:?}", regs)
    }
}

impl<'a> VM2<'a> {
    pub fn new(constants_pool: &'a [u64], complex_constants_pool: &'a [HeapValue]) -> Self {
        let mut stack_frames = [StackFrame::empty(); MAX_CALL_DEPTH];
        stack_frames[0] = StackFrame::empty();

        Self {
            constants_pool,
            outer_stack_frame: &stack_frames[0] as *const StackFrame,
            complex_constants_pool,
            stack_frames,
            stack_frame_depth: 0,
            registers: [0; u16::MAX as usize],
            heap: Heap::new(),
        }
    }

    pub fn push_stack_frame(
        &mut self,
        call_instruction_ptr: *const CompiledInstruction,
        next_instruction_ptr: *const CompiledInstruction,
        register_offset: usize
    ) {
        let prev_reg_offset = self.stack_frames[self.stack_frame_depth].register_offset;
        self.stack_frame_depth += 1;
        self.stack_frames[self.stack_frame_depth] = StackFrame::new(
            call_instruction_ptr,
            next_instruction_ptr,
            prev_reg_offset + register_offset
        );
        self.outer_stack_frame = &self.stack_frames[self.stack_frame_depth] as *const StackFrame;
    }

    pub fn pop_stack_frame_as_ptr(
        &mut self
    ) -> (*const CompiledInstruction, *const CompiledInstruction) {
        let stack_frame = self.stack_frames[self.stack_frame_depth];
        self.stack_frame_depth -= 1;
        self.outer_stack_frame = &self.stack_frames[self.stack_frame_depth] as *const StackFrame;
        (stack_frame.call_instruction_ptr, stack_frame.next_instruction_ptr)
    }

    pub fn read_register_offset(&self) -> usize {
        unsafe { (*self.outer_stack_frame).register_offset }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StackFrame {
    call_instruction_ptr: *const CompiledInstruction,
    next_instruction_ptr: *const CompiledInstruction,
    register_offset: usize,
}

impl StackFrame {
    pub fn empty() -> Self {
        Self {
            call_instruction_ptr: std::ptr::null(),
            next_instruction_ptr: std::ptr::null(),
            register_offset: 0,
        }
    }

    pub fn new(
        call_instruction_ptr: *const CompiledInstruction,
        next_instruction_ptr: *const CompiledInstruction,
        register_offset: usize
    ) -> Self {
        Self {
            call_instruction_ptr,
            next_instruction_ptr,
            register_offset,
        }
    }
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
