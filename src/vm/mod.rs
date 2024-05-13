use crate::{ constants::REGISTERS, value::Value };

pub mod instructions;
mod helper_methods;

use self::instructions::{ Instruction, InstructionSrc };

pub struct Registers {
    registers: Vec<Vec<Value>>,
}

impl Registers {
    pub fn new() -> Registers {
        let mut registers = Vec::with_capacity(REGISTERS);
        for _ in 0..REGISTERS {
            registers.push(Value::Empty);
        }

        Registers {
            registers: vec![registers],
        }
    }

    pub fn get(&self, index: usize, scope_depth: usize) -> &Value {
        self.registers[scope_depth].get(index).unwrap()
    }

    pub fn get_mut(&mut self, index: usize, scope_depth: usize) -> &mut Value {
        self.registers[scope_depth].get_mut(index).unwrap()
    }

    pub fn start_scope(&mut self) {
        let mut registers = Vec::with_capacity(REGISTERS);
        for _ in 0..REGISTERS {
            registers.push(Value::Empty);
        }

        type usize = i32;
        let d: usize = 2;

        self.registers.push(registers);
    }

    pub fn end_scope(&mut self) {
        // for i in 0..REGISTERS {
        //     let scope = self.registers.len() - 1;
        //     let register = self.get(i, scope);
        //     if let Value::Int32(value) = *register {
        //         if value != 0 {
        //             println!("S{}:R{}: {}", scope, i, value);
        //         }
        //     } else {
        //         println!("S{}:R{}: {}", scope, i, register.to_string());
        //     }
        // }
        self.registers.pop();
    }
}

pub struct StackFrame {
    ip: usize,
    instructions: Vec<Instruction>,
    slots: usize,
}

pub struct VM {
    stack_frames: Vec<StackFrame>,
    registers: [Option<Value>; REGISTERS],
    stack: Vec<Value>,
    program: Vec<Instruction>,
    pc: usize,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> VM {
        VM {
            stack_frames: Vec::with_capacity(64),
            registers: [None; REGISTERS],
            stack: Vec::with_capacity(64),
            program,
            pc: 0,
        }
    }

    fn get_from_stack(&self, stack_pos: usize) -> &Value {
        self.stack.get(stack_pos).unwrap()
    }

    fn push_to_stack(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn get_from_stack_mut(&mut self, stack_pos: usize) -> &mut Value {
        self.stack.get_mut(stack_pos).unwrap()
    }

    fn get_register(&self, instruction_register: usize) -> &Value {
        self.registers.get(instruction_register).unwrap().as_ref().unwrap()
    }

    fn get_register_mut(&mut self, instruction_register: usize) -> &mut Option<Value> {
        self.registers.get_mut(instruction_register).unwrap()
    }

    fn get_cmp_values(&self) -> (Value, Value) {
        match self.program.get(self.pc - 1).unwrap() {
            Instruction::Cmp { src1, src2 } => {
                let lhs = match src1 {
                    InstructionSrc::Constant(v) => *v,
                    InstructionSrc::Register(reg) => { *self.get_register(*reg) }
                    InstructionSrc::Stack(stack_pos) => { *self.get_from_stack(*stack_pos) }
                };
                let rhs = match src2 {
                    InstructionSrc::Constant(v) => *v,
                    InstructionSrc::Register(reg) => { *self.get_register(*reg) }
                    InstructionSrc::Stack(stack_pos) => { *self.get_from_stack(*stack_pos) }
                };

                (lhs, rhs)
            }
            _ => panic!("Expected CMP instruction"),
        }
    }

    #[profiler::function_tracker("vm-execution")]
    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            let instruction = self.get_instruction();

            match instruction {
                Instruction::JmpPop { pos, amount } => {
                    let pos = *pos;
                    self.stack.truncate(self.stack.len() - amount);
                    self.pc = pos;
                    continue;
                }
                Instruction::Pop { amount } => {
                    self.stack.truncate(self.stack.len() - amount);
                }
                Instruction::Cmp { .. } => {
                    self.pc += 1;
                    continue;
                }
                Instruction::JE { true_pos, false_pos } => {
                    let (lhs, rhs) = self.get_cmp_values();

                    if lhs.cmp_e(&rhs).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::JNE { true_pos, false_pos } => {
                    let (lhs, rhs) = self.get_cmp_values();

                    if lhs.cmp_ne(&rhs).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::JG { true_pos, false_pos } => {
                    let (lhs, rhs) = self.get_cmp_values();

                    if lhs.cmp_g(&rhs).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::JGE { true_pos, false_pos } => {
                    let (lhs, rhs) = self.get_cmp_values();

                    if lhs.cmp_ge(&rhs).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::JL { true_pos, false_pos } => {
                    let (lhs, rhs) = self.get_cmp_values();

                    if lhs.cmp_l(&rhs).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::JLE { true_pos, false_pos } => {
                    let (lhs, rhs) = self.get_cmp_values();

                    if lhs.cmp_le(&rhs).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::Jmp { pos } => {
                    self.pc = *pos;
                    continue;
                }
                Instruction::Halt => {
                    #[cfg(debug_assertions)]
                    {
                        println!("Stack: {:?}", self.stack)
                    }
                    break;
                }

                Instruction::Define { stack_pos, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => *self.get_register(*register),
                        InstructionSrc::Constant(value) => *value,
                        InstructionSrc::Stack(pos) => *self.get_from_stack(*pos),
                    };

                    self.push_to_stack(src);
                }
                Instruction::Assign { stack_pos, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };
                    *self.get_from_stack_mut(*stack_pos) = src.clone();
                }
                Instruction::Add { reg, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(pos) => self.get_from_stack(*pos),
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(pos) => self.get_from_stack(*pos),
                    };

                    *self.get_register_mut(*reg) = Some(src1.add(&src2).unwrap());
                }
                Instruction::Sub { reg, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    *self.get_register_mut(*reg) = Some(src1.sub(&src2).unwrap());
                }
                Instruction::Mul { reg, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    *self.get_register_mut(*reg) = Some(src1.mul(&src2).unwrap());
                }
                Instruction::Div { reg, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    *self.get_register_mut(*reg) = Some(src1.div(&src2).unwrap());
                }
                Instruction::Neg { reg, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    *self.get_register_mut(*reg) = Some(src.neg().unwrap());
                }
                Instruction::Truthy { reg, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                        InstructionSrc::Stack(stack_pos) => self.get_from_stack(*stack_pos),
                    };

                    *self.get_register_mut(*reg) = Some(src.not());
                }
            }
            self.pc += 1;
        }
    }
}
