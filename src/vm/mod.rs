use crate::{ constants::REGISTERS, value::Value };

use self::{ instructions::InstructionSrc, instructions::Instruction };

pub mod instructions;
mod helper_methods;
pub struct VM {
    registers: [Value; REGISTERS],
    program: Vec<Instruction>,
    pc: usize,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> VM {
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
                Instruction::Halt => {
                    #[cfg(debug_assertions)]
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
                    println!("Result: {}", self.registers[1].to_string());
                    break;
                }
                Instruction::Load { reg, value } => {
                    self.registers[*reg] = *value;
                }
                Instruction::Add { dest, src1, src2 } => {
                    let src1 = match src1 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.add(&src2).unwrap();
                }
                Instruction::Define { dest, src } => {
                    let src = match src {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };
                    self.registers[*dest] = src;
                }
                Instruction::Sub { dest, src1, src2 } => {
                    let src1 = match src1 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.sub(&src2).unwrap();
                }
                Instruction::Mul { dest, src1, src2 } => {
                    let src1 = match src1 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.mul(&src2).unwrap();
                }
                Instruction::Div { dest, src1, src2 } => {
                    let src1 = match src1 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    let src2 = match src2 {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src1.div(&src2).unwrap();
                }
                Instruction::Neg { dest, src } => {
                    let src = match src {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src.neg().unwrap();
                }
                Instruction::Truthy { dest, src } => {
                    let src = match src {
                        | InstructionSrc::Register(register)
                        | InstructionSrc::VariableRegister(register) => self.registers[*register],
                        InstructionSrc::Constant(value) => *value,
                    };

                    self.registers[*dest] = src.not();
                }
            }
            self.pc += 1;
        }
    }
}
