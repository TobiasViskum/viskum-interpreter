use crate::vm::instructions::{ IRInstruction, IRInstructionSrc, VMInstruction };

use super::BytecodeGenerator;

impl<'a> BytecodeGenerator<'a> {
    #[profiler::function_tracker]
    pub fn get_optimized_registers(&mut self) -> Vec<VMInstruction> {
        let mut instructions: Vec<VMInstruction> = Vec::new();

        for i in 0..self.instructions.len() {
            let instruction = self.instructions.get(i).unwrap();

            match instruction {
                | IRInstruction::Add { dest, src1, src2 }
                | IRInstruction::Mul { dest, src1, src2 }
                | IRInstruction::Sub { dest, src1, src2 }
                | IRInstruction::Div { dest, src1, src2 } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(*dest);

                    let src1 = match src1 {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(*register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(*register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }

                        _ => src1.clone(),
                    };

                    let src2 = match src2 {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(*register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(*register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }

                        _ => src2.clone(),
                    };

                    self.instructions
                        .get_mut(i)
                        .unwrap()
                        .modify_binary_instruction(physical_register, src1, src2);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Neg { dest, src } | IRInstruction::Truthy { dest, src } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(*dest);

                    let src = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(*register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(*register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions
                        .get_mut(i)
                        .unwrap()
                        .modify_unary_instruction(physical_register, src);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Define { dest, src } => {
                    let dest = self.map_virtual_reg_to_physical_reg(*dest);

                    let src = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(*register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(*register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions.get_mut(i).unwrap().modify_definement_instruction(dest, src);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Assign { dest, src } => {
                    let dest = self.map_virtual_reg_to_physical_reg(*dest);

                    let src = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(*register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(*register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions.get_mut(i).unwrap().modify_assignment_instruction(dest, src);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Load { reg, src } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(*reg);

                    let value = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(*register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(*register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions
                        .get_mut(i)
                        .unwrap()
                        .modify_load_instruction(physical_register, value);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }

                IRInstruction::Halt => {
                    instructions.push(VMInstruction::Halt);
                    break;
                }
            }
        }

        instructions
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

    fn get_and_release_physical_register(&self, virtual_register: usize) -> usize {
        let physical_register = *self.virtual_registers.borrow().get(&virtual_register).unwrap();
        self.remove_virtual_register(virtual_register);
        self.release_physical_register(physical_register);
        physical_register
    }

    fn get_physcal_register(&self, virtual_register: usize) -> usize {
        *self.virtual_registers.borrow().get(&virtual_register).unwrap()
    }

    fn remove_virtual_register(&self, virtual_register: usize) {
        self.virtual_registers.borrow_mut().remove(&virtual_register);
    }

    fn release_physical_register(&self, physical_register: usize) {
        self.available_registers.borrow_mut().push(physical_register);
    }
}
