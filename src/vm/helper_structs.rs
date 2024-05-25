use std::{ mem, ptr };

use colored::Colorize;

use crate::{ constants::REGISTERS, value::{ Function, Value } };

#[derive(Debug)]
pub enum VMValue {
    Value(Value),
    InstructionPointer(usize),
}

impl VMValue {
    pub fn new_val(value: Value) -> Self {
        Self::Value(value)
    }

    pub fn new_instr_pointer(ip: usize) -> Self {
        Self::InstructionPointer(ip)
    }

    pub fn as_val(&self) -> &Value {
        match self {
            Self::Value(v) => v,
            _ => panic!("Expected value in as_val"),
        }
    }

    pub fn as_mut_val(&mut self) -> &mut Value {
        match self {
            Self::Value(v) => v,
            _ => panic!("Expected value in as_mut_val"),
        }
    }

    pub fn as_ip(&self) -> usize {
        match self {
            Self::InstructionPointer(ip) => *ip,
            _ => panic!("Expected ip in as_ip"),
        }
    }
}

#[derive(Debug)]
pub struct VMRegisters2 {
    registers: [*const Value; REGISTERS],
}

impl VMRegisters2 {
    pub fn new() -> Self {
        // let registers = vec![None; REGISTERS];

        const ARRAY_REPEAT_VALUE: *const Value = ptr::null();
        Self { registers: [ARRAY_REPEAT_VALUE; REGISTERS] }
    }

    #[inline(always)]
    pub fn get(&mut self, reg: usize) -> *const Value {
        unsafe { *self.registers.get_unchecked(reg) }
    }

    #[inline(always)]
    pub fn get_ref(&self, reg: usize) -> *const Value {
        unsafe { *self.registers.get_unchecked(reg) }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, reg: usize) -> &mut *const Value {
        unsafe { self.registers.get_unchecked_mut(reg) }
    }
}

#[derive(Debug)]
pub struct VMRegisters {
    registers: [Option<Value>; REGISTERS],
}

impl VMRegisters {
    pub fn new() -> Self {
        // let registers = vec![None; REGISTERS];

        const ARRAY_REPEAT_VALUE: Option<Value> = None;
        Self { registers: [ARRAY_REPEAT_VALUE; REGISTERS] }
    }

    pub fn trace(&self) {
        let mut found = false;
        for i in 0..self.registers.len() {
            if self.registers[i].is_some() {
                found = true;
                break;
            }
        }
        if !found {
            return;
        }

        print!("{{ ");
        for i in 0..self.registers.len() {
            if let Some(reg) = &self.registers[i] {
                print!(
                    " {}{} ",
                    format!("R{}: ", i).as_str().dimmed(),
                    reg.to_string().as_str().bold()
                );
            }
        }
        println!("  }}")
    }

    #[inline(always)]
    pub fn get(&mut self, reg: usize) -> Value {
        mem::take(self.get_mut(reg)).unwrap()
    }

    #[inline(always)]
    pub fn get_ref(&self, reg: usize) -> &Value {
        unsafe { self.registers.get_unchecked(reg).as_ref().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, reg: usize) -> &mut Option<Value> {
        unsafe { self.registers.get_unchecked_mut(reg) }
    }
}

#[derive(Debug)]
pub struct VMStack {
    stack: Vec<VMValue>,
}

impl VMStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn trace(&self) {
        print!("[ ");

        for (i, val) in self.stack.iter().enumerate() {
            print!(
                " {}{} ",
                format!("{}: ", i).as_str().dimmed().italic(),
                val.as_val().to_string().as_str().bold()
            );
        }
        println!(" ]");
    }

    // #[inline(always)]
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    #[inline(always)]
    pub fn get_as_val(&self, stack_pos: usize) -> Value {
        unsafe { self.stack.get_unchecked(stack_pos).as_val().clone() }
    }

    #[inline(always)]
    pub fn get_as_ip(&self, stack_pos: usize) -> usize {
        unsafe { self.stack.get_unchecked(stack_pos).as_ip() }
    }

    #[inline(always)]
    pub fn pop_as_val(&mut self) -> Value {
        match self.stack.pop().unwrap() {
            VMValue::Value(v) => v,
            _ => panic!("Expected value in pop_as_val"),
        }
    }

    #[inline(always)]
    pub fn pop_as_ip(&mut self) -> usize {
        self.stack.pop().unwrap().as_ip()
    }

    #[inline(always)]
    pub fn read_last_as_ip(&mut self) -> usize {
        self.stack.last().unwrap().as_ip()
    }

    #[inline(always)]
    pub fn get_ref(&self, stack_pos: usize) -> &Value {
        unsafe { self.stack.get_unchecked(stack_pos).as_val() }
    }

    #[inline(always)]
    pub fn get_mut_ref(&mut self, stack_pos: usize) -> &mut Value {
        self.stack[stack_pos].as_mut_val()
    }

    // #[inline(always)]
    pub fn get_ptr_func(&mut self, stack_pos: usize) -> *mut Function {
        match self.stack[stack_pos].as_mut_val() {
            Value::Function(func) => func as *mut Function,
            _ => panic!("sdf"),
        }
    }

    #[inline(always)]
    pub fn push_val(&mut self, value: Value) {
        self.stack.push(VMValue::Value(value));
    }

    #[inline(always)]
    pub fn push_ip(&mut self, ip: usize) {
        self.stack.push(VMValue::InstructionPointer(ip));
    }

    // #[inline(always)]
    pub fn push(&mut self, value: Value, expected_pos: usize) {
        self.stack.push(VMValue::Value(value));
        if self.stack.len() - 1 != expected_pos {
            panic!("Expected stack_heigh: {} but got: {}", expected_pos, self.stack.len() - 1)
        }
    }

    #[inline(always)]
    pub fn decrement_stack_height(&mut self, decrement_val: usize) {
        self.stack.truncate(self.stack.len() - decrement_val);
    }
}

#[derive(Debug)]
pub struct ConstantsTable<'a> {
    constants_table: &'a [Value],
}

impl<'a> ConstantsTable<'a> {
    pub fn new(constants_table: &'a [Value]) -> Self {
        Self { constants_table }
    }
}

#[derive(Debug)]
pub struct RuntimeInformation {
    stack_heights: Vec<usize>,
}

impl RuntimeInformation {
    pub fn new() -> Self {
        Self {
            stack_heights: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn pop_stack_height(&mut self) {
        self.stack_heights.pop();
    }

    #[inline(always)]
    pub fn push_stack_height(&mut self, height: usize) {
        self.stack_heights.push(height);
    }

    #[inline(always)]
    pub fn get_stack_offset(&self) -> usize {
        unsafe { *self.stack_heights.get_unchecked(self.stack_heights.len() - 1) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CallFrame {
    ip: usize,
    function_pos: usize,
    stack_height: usize,
}

impl CallFrame {
    pub fn new(function_pos: usize, stack_height: usize) -> Self {
        Self {
            ip: 0,
            function_pos,
            stack_height,
        }
    }

    pub fn execute(&mut self, stack: &mut VMStack, registers: &mut VMRegisters) -> Value {
        let result = match stack.get_as_val(self.function_pos) {
            Value::Function(mut func) =>
                func.instructions.execute(&mut self.ip, stack, registers, self.stack_height),
            _ => panic!("Expected func!"),
        };

        result
    }
}

#[derive(Debug)]
pub struct CallFrames {
    call_frames: Vec<CallFrame>,
}

impl CallFrames {
    pub fn new() -> Self {
        Self {
            call_frames: Vec::new(),
        }
    }

    #[inline]
    pub fn get(&self, call_frame_index: usize) -> &CallFrame {
        &self.call_frames[call_frame_index]
    }

    pub fn pop(&mut self) {
        self.call_frames.pop();
    }

    // pub fn len(&self) -> usize {
    //     self.call_frames.len()
    // }

    pub fn push(&mut self, call_frame: CallFrame) -> usize {
        self.call_frames.push(call_frame);
        self.call_frames.len() - 1
    }
}
