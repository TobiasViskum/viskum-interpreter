use std::{ borrow::Borrow, cell::RefCell, collections::HashMap };

use crate::vm::instructions::{
    helper_structs::InstructionRegister,
    IRInstruction,
    IRInstructionSrc,
    VMInstruction,
};

use super::{ BytecodeGenerator, BytecodeScopeRegisters };

impl<'a> BytecodeGenerator<'a> {
    #[profiler::function_tracker]
    pub fn get_optimized_registers(&mut self) -> Vec<VMInstruction> {
        let mut instructions: Vec<VMInstruction> = Vec::new();

        for instruction in &self.instructions {
            println!("{}", instruction.dissassemble());
        }

        for i in 0..self.instructions.len() {
            let instruction = self.instructions.get(i).unwrap();

            match instruction {
                | IRInstruction::Add { dest, src1, src2 }
                | IRInstruction::Mul { dest, src1, src2 }
                | IRInstruction::Sub { dest, src1, src2 }
                | IRInstruction::Div { dest, src1, src2 } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(dest);

                    let src1 = match src1 {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }

                        _ => src1.clone(),
                    };

                    let src2 = match src2 {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(register);
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
                IRInstruction::StartScope => {
                    self.start_scope();
                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::EndScope => {
                    self.end_scope();
                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Neg { dest, src } | IRInstruction::Truthy { dest, src } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(dest);

                    let src = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(register);
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
                    let dest = self.map_virtual_reg_to_physical_reg(dest);

                    let src = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions.get_mut(i).unwrap().modify_definement_instruction(dest, src);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Assign { dest, src } => {
                    let dest = self.map_virtual_reg_to_physical_reg(dest);

                    let src = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(register);
                            IRInstructionSrc::VariableRegister(new_register)
                        }
                        _ => src.clone(),
                    };

                    self.instructions.get_mut(i).unwrap().modify_assignment_instruction(dest, src);

                    let instruction = self.instructions.get(i).unwrap().to_vm_instruction();
                    instructions.push(instruction);
                }
                IRInstruction::Load { reg, src } => {
                    let physical_register = self.map_virtual_reg_to_physical_reg(reg);

                    let value = match src {
                        IRInstructionSrc::Register(register) => {
                            let new_register = self.get_and_release_physical_register(register);
                            IRInstructionSrc::Register(new_register)
                        }
                        IRInstructionSrc::VariableRegister(register) => {
                            let new_register = self.get_physcal_register(register);
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

    fn get_available_registers(&self, scope_depth: usize) -> &RefCell<Vec<usize>> {
        &self.bytecode_scope_registers.get(scope_depth).unwrap().available_registers
    }

    fn get_virtual_registers(&self, scope_depth: usize) -> &RefCell<HashMap<usize, usize>> {
        &self.bytecode_scope_registers.get(scope_depth).unwrap().virtual_registers
    }

    fn get_next_register(&self, scope_depth: usize) -> usize {
        match self.get_available_registers(scope_depth).borrow_mut().pop() {
            Some(register) => register,
            None => { panic!("No available registers") }
        }
    }

    fn map_virtual_reg_to_physical_reg(
        &self,
        instruction_register: &InstructionRegister
    ) -> InstructionRegister {
        let (virtual_register, scope_depth) = (
            instruction_register.register,
            instruction_register.scope,
        );

        let physical_register = self.get_next_register(scope_depth);
        self.get_virtual_registers(scope_depth)
            .borrow_mut()
            .insert(virtual_register, physical_register);
        InstructionRegister::new(physical_register, scope_depth)
    }

    fn get_and_release_physical_register(
        &self,
        instruction_register: &InstructionRegister
    ) -> InstructionRegister {
        let (virtual_register, scope_depth) = (
            instruction_register.register,
            instruction_register.scope,
        );

        let physical_register = self.get_physcal_register(instruction_register);

        self.remove_virtual_register(virtual_register, scope_depth);
        self.release_physical_register(&physical_register);
        physical_register
    }

    fn get_physcal_register(
        &self,
        instruction_register: &InstructionRegister
    ) -> InstructionRegister {
        let (virtual_register, scope_depth) = (
            instruction_register.register,
            instruction_register.scope,
        );

        let physical_register = *self
            .get_virtual_registers(scope_depth)
            .borrow()
            .get(&virtual_register)
            .unwrap();

        InstructionRegister::new(physical_register, scope_depth)
    }

    fn remove_virtual_register(&self, virtual_register: usize, scope_depth: usize) {
        self.get_virtual_registers(scope_depth).borrow_mut().remove(&virtual_register);
    }

    fn release_physical_register(&self, instruction_register: &InstructionRegister) {
        self.get_available_registers(instruction_register.scope)
            .borrow_mut()
            .push(instruction_register.register);
    }

    /*
     fn get_physcal_register(&self, virtual_register: usize) -> usize {
        *self.virtual_registers.borrow().get(&virtual_register).unwrap()
    }

    fn remove_virtual_register(&self, virtual_register: usize) {
        self.virtual_registers.borrow_mut().remove(&virtual_register);
    }

    fn release_physical_register(&self, physical_register: usize) {
        self.available_registers.borrow_mut().push(physical_register);
    }
    */

    fn start_scope(&mut self) {
        self.scope_depth += 1;
        self.bytecode_scope_registers.push(BytecodeScopeRegisters::new());
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        self.bytecode_scope_registers.pop();
    }
}

/*
DEFINE S0:R0 1
STARTSCOPE
    DEFINE S1:R2 2
    ADD S1:R4 S0:R2 1
    STARTSCOPE
        DEFINE S2:R6 3
        ADD S2:R8 S0:R6 1
    ENDSCOPE
    LOAD S1:R10 true
ENDSCOPE
LOAD S0:R11 false
LOAD S0:R12 true
STARTSCOPE
    ADD S1:R13 S0:R0 2
ENDSCOPE
HALT
*/
