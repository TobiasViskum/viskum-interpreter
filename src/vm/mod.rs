use crate::{ constants::REGISTERS, value::Value };

pub mod instructions;
mod helper_methods;

use self::instructions::{ InstructionRegister, Instruction, InstructionSrc };

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

pub struct VMFunction {
    registers: Registers,
    instructions: Vec<Instruction>,
    ip: usize,
}

pub struct VM {
    registers: Registers,
    program: Vec<Instruction>,
    pc: usize,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> VM {
        VM {
            registers: Registers::new(),
            program,
            pc: 0,
        }
    }

    fn get_register(&self, instruction_register: InstructionRegister) -> &Value {
        self.registers.get(instruction_register.register, instruction_register.scope)
    }

    fn get_register_mut(&mut self, instruction_register: InstructionRegister) -> &mut Value {
        self.registers.get_mut(instruction_register.register, instruction_register.scope)
    }

    fn start_scope(&mut self) {
        self.registers.start_scope();
    }

    fn end_scope(&mut self) {
        self.registers.end_scope();
    }

    #[profiler::function_tracker("vm-execution")]
    pub fn run(&mut self) {
        let mut cmp_operands: [Option<InstructionSrc>; 2] = [None, None];

        while self.pc < self.program.len() {
            let instruction = self.get_instruction();
            match instruction {
                Instruction::Cmp { src1, src2 } => {
                    cmp_operands = [Some(src1.clone()), Some(src2.clone())];
                }

                Instruction::JE { true_pos, false_pos } => {
                    let left_operand = match cmp_operands[0].take().unwrap() {
                        InstructionSrc::Constant(v) => v,
                        InstructionSrc::Register(reg) => { *self.get_register(reg) }
                    };
                    let right_operand = match cmp_operands[1].take().unwrap() {
                        InstructionSrc::Constant(v) => v,
                        InstructionSrc::Register(reg) => { *self.get_register(reg) }
                    };

                    if left_operand.cmp_g(&right_operand).unwrap() {
                        self.pc = *true_pos;
                    } else {
                        self.pc = *false_pos;
                    }
                    continue;
                }
                Instruction::Jmp { pos } => {
                    self.pc = *pos - 1;
                    continue;
                }
                Instruction::Function { dest, instructions_count } => panic!("NOT IMPLEMENTED"),
                Instruction::Halt => {
                    // #[cfg(debug_assertions)]
                    // {
                    //     for scope in &self.registers.registers {
                    //         for register in scope {
                    //             if let Value::Int32(value) = register {
                    //                 if value != &0 {
                    //                     println!("{}", value);
                    //                 }
                    //             } else {
                    //                 println!("{}", register.to_string());
                    //             }
                    //         }
                    //     }
                    // }
                    break;
                }
                Instruction::StartScope => {
                    self.start_scope();
                }
                Instruction::EndScope => {
                    self.end_scope();
                }
                Instruction::Load { reg, src } => {
                    let src = match src {
                        InstructionSrc::Register(_) => self.get_register(*reg),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*reg) = src.clone();
                }
                Instruction::Add { dest, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.add(&src2).unwrap();
                }
                Instruction::Define { dest, src } => {
                    let mut src = Some(match src {
                        InstructionSrc::Register(register) => *self.get_register(*register),
                        InstructionSrc::Constant(value) => *value,
                    });
                    *self.get_register_mut(*dest) = src.take().unwrap();
                }
                Instruction::Assign { dest, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };
                    *self.get_register_mut(*dest) = src.clone();
                }
                Instruction::Sub { dest, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.sub(&src2).unwrap();
                }
                Instruction::Mul { dest, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.mul(&src2).unwrap();
                }
                Instruction::Div { dest, src1, src2 } => {
                    let src1 = match src1 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.div(&src2).unwrap();
                }
                Instruction::Neg { dest, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src.neg().unwrap();
                }
                Instruction::Truthy { dest, src } => {
                    let src = match src {
                        InstructionSrc::Register(register) => self.get_register(*register),
                        InstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src.not();
                }
            }
            self.pc += 1;
        }
    }
}
