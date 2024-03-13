use crate::vm::instructions::{ Instruction, InstructionSrc };

use super::BytecodeGenerator;

impl<'a> BytecodeGenerator<'a> {
    #[profiler::function_tracker]
    pub fn optimize_registers(&mut self) {
        for i in 0..self.instructions.len() {
            let instruction = self.instructions.get(i).unwrap();

            match instruction {
                | Instruction::Add { dest, src1, src2 }
                | Instruction::Mul { dest, src1, src2 }
                | Instruction::Sub { dest, src1, src2 }
                | Instruction::Div { dest, src1, src2 } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(*dest);

                    let src1 = match src1 {
                        InstructionSrc::Register(register) => {
                            let new_register = self.get_physical_register(*register);
                            InstructionSrc::Register(new_register)
                        }
                        _ => src1.clone(),
                    };

                    let src2 = match src2 {
                        InstructionSrc::Register(register) => {
                            let new_register = self.get_physical_register(*register);
                            InstructionSrc::Register(new_register)
                        }
                        _ => src2.clone(),
                    };

                    self.instructions
                        .get_mut(i)
                        .unwrap()
                        .modify_binary_instruction(physical_register, src1, src2);
                }
                Instruction::Neg { dest, src } | Instruction::Truthy { dest, src } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(*dest);

                    let src = match src {
                        InstructionSrc::Register(register) => {
                            let new_register = self.get_physical_register(*register);
                            InstructionSrc::Register(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions
                        .get_mut(i)
                        .unwrap()
                        .modify_unary_instruction(physical_register, src);
                }
                Instruction::Load { reg, value } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(*reg);

                    let value = value.clone();

                    self.instructions
                        .get_mut(i)
                        .unwrap()
                        .modify_load_instruction(physical_register, value);
                }

                Instruction::Halt => {
                    break;
                }
                _ => {
                    panic!("Unsupported instruction for register optimization: {:?}", instruction);
                }
            }
        }
    }

    fn get_next_register(&self) -> usize {
        match self.available_registers.borrow_mut().pop() {
            Some(register) => register,
            None => { panic!("No available registers") }
        }
    }

    fn map_virtual_reg_to_physical_reg(&self, virtual_register: usize) -> usize {
        let physical_register = self.get_next_register();
        self.virtual_registers.borrow_mut().insert(virtual_register, physical_register);
        physical_register
    }

    fn get_physical_register(&self, virtual_register: usize) -> usize {
        let physical_register = *self.virtual_registers.borrow().get(&virtual_register).unwrap();
        self.remove_virtual_register(virtual_register);
        self.release_physical_register(physical_register);
        physical_register
    }

    fn remove_virtual_register(&self, virtual_register: usize) {
        self.virtual_registers.borrow_mut().remove(&virtual_register);
    }

    fn release_physical_register(&self, physical_register: usize) {
        self.available_registers.borrow_mut().push(physical_register);
    }
}
