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

    fn get_cmp_values(&self) -> (Value, Value) {
        match self.program.get(self.pc - 1).unwrap() {
            Instruction::Cmp { src1, src2 } => {
                let lhs = match src1 {
                    InstructionSrc::Constant(v) => *v,
                    InstructionSrc::Register(reg) => { *self.get_register(*reg) }
                };
                let rhs = match src2 {
                    InstructionSrc::Constant(v) => *v,
                    InstructionSrc::Register(reg) => { *self.get_register(*reg) }
                };

                (lhs, rhs)
            }
            _ => panic!("sdf"),
        }
    }

    #[profiler::function_tracker("vm-execution")]
    pub fn run(&mut self) {
        while self.pc < self.program.len() {
            let instruction = self.get_instruction();
            match instruction {
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
                Instruction::Function { dest, instructions_count } => panic!("NOT IMPLEMENTED"),
                Instruction::Halt => {
                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "R1:S0 = {:?}",
                            self.get_register(InstructionRegister::new(1, 0, true))
                        )
                    }
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
