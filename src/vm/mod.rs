use crate::{ constants::REGISTERS, value::Value };

use self::instructions::{ helper_structs::InstructionRegister, VMInstruction, VMInstructionSrc };

pub mod instructions;
mod helper_methods;

pub struct Registers {
    registers: Vec<[Value; REGISTERS]>,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: vec![[Value::Int32(0); REGISTERS]],
        }
    }

    pub fn get(&self, index: usize, scope_depth: usize) -> &Value {
        self.registers[scope_depth].get(index).unwrap()
    }

    pub fn get_mut(&mut self, index: usize, scope_depth: usize) -> &mut Value {
        self.registers[scope_depth].get_mut(index).unwrap()
    }

    pub fn start_scope(&mut self) {
        self.registers.push([Value::Int32(0); REGISTERS]);
    }

    pub fn end_scope(&mut self) {
        for i in 0..REGISTERS {
            let register = self.get(i, self.registers.len() - 1);
            if let Value::Int32(value) = *register {
                if value != 0 {
                    println!("R{}: {}", i, value);
                }
            } else {
                println!("R{}: {}", i, register.to_string());
            }
        }
        self.registers.pop();
    }
}

pub struct VM {
    registers: Registers,
    program: Vec<VMInstruction>,
    pc: usize,
}

impl VM {
    pub fn new(program: Vec<VMInstruction>) -> VM {
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
        while self.pc < self.program.len() {
            let instruction = self.get_instruction();
            match instruction {
                VMInstruction::Halt => {
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
                VMInstruction::StartScope => {
                    self.start_scope();
                }
                VMInstruction::EndScope => {
                    self.end_scope();
                }
                VMInstruction::Load { reg, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.get_register(*reg),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*reg) = *src;
                }
                VMInstruction::Add { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.add(&src2).unwrap();
                }
                VMInstruction::Define { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };
                    *self.get_register_mut(*dest) = *src;
                }
                VMInstruction::Assign { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };
                    *self.get_register_mut(*dest) = *src;
                }
                VMInstruction::Sub { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.sub(&src2).unwrap();
                }
                VMInstruction::Mul { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.mul(&src2).unwrap();
                }
                VMInstruction::Div { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src1.div(&src2).unwrap();
                }
                VMInstruction::Neg { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src.neg().unwrap();
                }
                VMInstruction::Truthy { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.get_register(*register),
                        VMInstructionSrc::Constant(value) => value,
                    };

                    *self.get_register_mut(*dest) = src.not();
                }
            }
            self.pc += 1;
        }
    }
}
