use crate::{ constants::REGISTERS, value::Value };

use self::instructions::{ VMInstruction, VMInstructionSrc };

pub mod instructions;
mod helper_methods;
pub struct VM {
    registers: [Value; REGISTERS],
    program: Vec<VMInstruction>,
    pc: usize,
}

impl VM {
    pub fn new(program: Vec<VMInstruction>) -> VM {
        VM {
            registers: [Value::Int32(0); REGISTERS],
            program,
            pc: 0,
        }
    }

    #[profiler::function_tracker("vm-execution")]
    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            let instruction = self.get_instruction();
            match instruction {
                VMInstruction::Halt => {
                    #[cfg(debug_assertions)]
                    {
                        for i in 0..REGISTERS {
                            let register = self.registers[i];
                            if let Value::Int32(value) = register {
                                if value != 0 {
                                    println!("R{}: {}", i, value);
                                }
                            } else {
                                println!("R{}: {}", i, register.to_string());
                            }
                        }
                    }
                    break;
                }
                VMInstruction::Load { reg, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*reg] = src;
                }
                VMInstruction::Add { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.add(&src2).unwrap();
                }
                VMInstruction::Define { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };
                    self.registers[*dest] = src;
                }
                VMInstruction::Assign { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };
                    self.registers[*dest] = src;
                }
                VMInstruction::Sub { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.sub(&src2).unwrap();
                }
                VMInstruction::Mul { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.mul(&src2).unwrap();
                }
                VMInstruction::Div { dest, src1, src2 } => {
                    let src1 = match src1 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.div(&src2).unwrap();
                }
                VMInstruction::Neg { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src.neg().unwrap();
                }
                VMInstruction::Truthy { dest, src } => {
                    let src = match src {
                        VMInstructionSrc::Register(register) => self.registers[*register],
                        VMInstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src.not();
                }
            }
            self.pc += 1;
        }
    }
}
