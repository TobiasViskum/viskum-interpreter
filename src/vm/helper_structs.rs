use std::mem;

use colored::Colorize;

use crate::{ constants::REGISTERS, value::{ Function, Value } };

#[derive(Debug)]
pub struct VMRegisters {
    registers: Vec<Option<Value>>,
}

impl VMRegisters {
    pub fn new() -> Self {
        let registers = vec![None; REGISTERS];

        Self { registers }
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
        mem::take(&mut self.registers[reg]).unwrap()
    }

    #[inline(always)]
    pub fn get_ref(&self, reg: usize) -> &Value {
        &self.registers[reg].as_ref().unwrap()
    }

    #[inline(always)]
    pub fn get_mut_unwrap(&mut self, reg: usize) -> &mut Value {
        self.registers[reg].as_mut().unwrap()
    }

    #[inline(always)]
    pub fn get_mut(&mut self, reg: usize) -> &mut Option<Value> {
        &mut self.registers[reg]
    }
}

#[derive(Debug)]
pub struct VMStack {
    stack: Vec<Value>,
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
                val.to_string().as_str().bold()
            );
        }
        println!(" ]");
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    #[inline(always)]
    pub fn get(&self, stack_pos: usize) -> Value {
        self.stack[stack_pos].clone()
    }

    #[inline(always)]
    pub fn get_ref(&self, stack_pos: usize) -> &Value {
        &self.stack[stack_pos]
    }

    #[inline(always)]
    pub fn get_mut_ref(&mut self, stack_pos: usize) -> &mut Value {
        &mut self.stack[stack_pos]
    }

    #[inline(always)]
    pub fn get_ptr_func(&mut self, stack_pos: usize) -> *mut Function {
        match &mut self.stack[stack_pos] {
            Value::Function(func) => func as *mut Function,
            _ => panic!("sdf"),
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, stack_pos: usize) -> &mut Value {
        &mut self.stack[stack_pos]
    }

    pub fn push(&mut self, value: Value, expected_pos: usize) {
        self.stack.push(value);
        if self.stack.len() - 1 != expected_pos {
            panic!("Expected stack_heigh: {} but got: {}", expected_pos, self.stack.len() - 1)
        }
    }

    pub fn decrement_stack_height(&mut self, decrement_val: usize) {
        self.stack.truncate(self.stack.len() - decrement_val);
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
        let result = match stack.get(self.function_pos) {
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

    #[inline(always)]
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
