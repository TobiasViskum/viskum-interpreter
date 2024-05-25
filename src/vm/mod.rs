use std::{ rc::Rc, time::Instant };

use crate::{ value::{ Value, ValueType }, vm::helper_structs::CallFrame };

pub mod instructions;
pub mod helper_structs;
mod benchmarking;
pub mod vm2;

use self::{
    helper_structs::{ CallFrames, VMRegisters, VMStack },
    instructions::{ Instruction, InstructionSrc },
};

pub struct VM {
    registers: VMRegisters,
    stack: VMStack,
    program: Instructions,
    pc: usize,
}
impl VM {
    pub fn new(instructions: Vec<Instruction>) -> VM {
        let program = Instructions::from(instructions);

        #[cfg(debug_assertions)]
        {
            program.dissassemble();
        }

        VM {
            registers: VMRegisters::new(),
            stack: VMStack::new(),
            program,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        let now = Instant::now();
        // for _ in 0..1000000000 {
        self.program.execute(&mut self.pc, &mut self.stack, &mut self.registers, 0);
        // }
        println!("elapsed: {:?}", now.elapsed())
    }
}

#[derive(Debug, Clone)]
pub struct Instructions {
    pub instructions: Rc<[Instruction]>,
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            instructions: [].into(),
        }
    }

    pub fn dissassemble(&self) {
        let indentation_level = 0;
        let indentation_size = 4;

        // println!("Optimized instructions:");
        println!("----------------------\n");

        for i in 0..self.instructions.len() {
            let instruction = self.get_instruction(i);
            let print_string = format!(
                "{}{}",
                " ".repeat(indentation_level * indentation_size),
                instruction.dissassemble()
            );
            println!("{}", print_string);
        }

        println!("\n----------------------");
    }

    pub fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions: instructions.into(),
        }
    }

    // #[inline]
    // fn get_cmp_values(
    //     &self,
    //     ip: usize,
    //     registers: &mut VMRegisters,
    //     stack: &mut VMStack,
    //     stack_offset: usize
    // ) -> (Value, Value) {
    //     match self.get_instruction(ip - 1) {
    //         Instruction::Cmp { src1, src2 } => {
    //             let lhs = src1.get_val(registers, stack, stack_offset);
    //             let rhs = src2.get_val(registers, stack, stack_offset);

    //             (lhs, rhs)
    //         }
    //         _ => panic!("Expected CMP instruction"),
    //     }
    // }

    // #[inline(always)]
    fn get_instruction(&self, ip: usize) -> &Instruction {
        &self.instructions[ip]
    }

    fn trace(
        &self,
        stack: &mut VMStack,
        registers: &mut VMRegisters,
        instruction: &Instruction,
        stack_offset: usize
    ) {
        #[cfg(trace_execution)]
        {
            registers.trace();
            stack.trace();
            instruction.trace(stack_offset);
            print!("\n")
        }
    }

    #[profiler::function_tracker("vm-execution")]
    pub fn execute(
        &mut self,
        ip: &mut usize,
        stack: &mut VMStack,
        registers: &mut VMRegisters,
        stack_offset: usize
    ) -> Value {
        let len = self.instructions.len();
        let instructions = self.instructions.as_ref();

        while *ip < len {
            let instruction = &instructions[*ip];

            // let instruction_id = u8::from(instruction.clone());

            match instruction {
                Instruction::NativeCall { stack_loc_dest, args_regs, native_call } => {
                    let args = args_regs
                        .iter()
                        .map(|reg| registers.get_ref(*reg))
                        .collect::<Vec<_>>();

                    let result = native_call.call(&args);

                    stack.push(result, stack_loc_dest.get_stack_pos(stack_offset));
                }
                Instruction::ReturnPop { src, amount } => {
                    let val = src.get_val(registers, stack, stack_offset);

                    stack.decrement_stack_height(*amount);

                    // self.trace(stack, registers, instruction, stack_offset);

                    return val;
                }
                Instruction::Return { src } => {
                    let val = src.get_val(registers, stack, stack_offset);

                    // self.trace(stack, registers, instruction, stack_offset);

                    return val;
                }
                Instruction::Load { reg, src } => {
                    let val = src.get_val(registers, stack, stack_offset);

                    *registers.get_mut(*reg) = Some(val);
                }
                Instruction::Call { stack_loc_dest, stack_loc_call } => {
                    // self.trace(stack, registers, instruction, stack_offset);

                    // let stack_pos = stack_loc_call.get_stack_pos(stack_offset);
                    // let func: crate::value::Function = match stack.get(stack_pos) {
                    //     Value::Function(func) => func,
                    //     _ => panic!("sdf"),
                    // };
                    // let mut new_ip = 0;
                    // let val = func.instructions.execute(&mut new_ip, stack, registers, stack.len());

                    // let mut call_frame = CallFrame::new(stack_pos, stack.len());
                    // let val = call_frame.execute(stack, registers);

                    let func = stack.get_ptr_func(stack_loc_call.get_stack_pos(stack_offset));
                    let val = unsafe {
                        (*func).instructions.execute(&mut 0, stack, registers, stack.len())
                    };

                    stack.push(val, stack_loc_dest.get_stack_pos(stack_offset));
                }
                Instruction::JmpPop { pos, amount } => {
                    stack.decrement_stack_height(*amount);
                    *ip = *pos;

                    // self.trace(stack, registers, instruction, stack_offset);

                    continue;
                }
                Instruction::Pop { amount } => {
                    stack.decrement_stack_height(*amount);
                }
                Instruction::Cmp { .. } => {
                    *ip += 1;

                    // self.trace(stack, registers, instruction, stack_offset);

                    continue;
                }

                Instruction::Jmp { pos } => {
                    *ip = *pos;

                    // self.trace(stack, registers, instruction, stack_offset);

                    continue;
                }
                Instruction::Halt => {
                    #[cfg(debug_assertions)]
                    {
                        // println!("stack_len: {}", stack.len());
                        // println!("a = {:#?}", stack.get(0));
                    }
                    break;
                }

                Instruction::Define { stack_loc, src } => {
                    let stack_pos = stack_loc.get_stack_pos(stack_offset);
                    let src = src.get_val(registers, stack, stack_offset);

                    #[cfg(debug_assertions)]
                    {
                        if let Value::Function(func) = &src {
                            func.instructions.dissassemble();
                        }
                    }

                    stack.push(src, stack_pos);
                }
                Instruction::Assign { stack_loc, src } => {
                    let stack_pos = stack_loc.get_stack_pos(stack_offset);
                    let src = src.get_val(registers, stack, stack_offset);
                    *stack.get_mut_ref(stack_pos) = src;
                }
                | Instruction::JE { true_pos, false_pos }
                | Instruction::JNE { true_pos, false_pos }
                | Instruction::JG { true_pos, false_pos }
                | Instruction::JGE { true_pos, false_pos }
                | Instruction::JL { true_pos, false_pos }
                | Instruction::JLE { true_pos, false_pos } => {
                    let (lhs, rhs) = match self.get_instruction(*ip - 1) {
                        Instruction::Cmp { src1, src2 } => {
                            let lhs = src1.get_val_ref(registers, stack, stack_offset);
                            let rhs = src2.get_val_ref(registers, stack, stack_offset);

                            (lhs, rhs)
                        }
                        _ => panic!("Expected CMP instruction"),
                    };

                    let condition = (
                        match instruction {
                            Instruction::JE { .. } => lhs.cmp_e(&rhs),
                            Instruction::JNE { .. } => lhs.cmp_ne(&rhs),
                            Instruction::JG { .. } => lhs.cmp_g(&rhs),
                            Instruction::JGE { .. } => lhs.cmp_ge(&rhs),
                            Instruction::JL { .. } => lhs.cmp_l(&rhs),
                            Instruction::JLE { .. } => lhs.cmp_le(&rhs),
                            _ => unreachable!(),
                        }
                    ).unwrap();

                    *ip = if condition { *true_pos } else { *false_pos };

                    // self.trace(stack, registers, instruction, stack_offset);

                    continue;
                }
                | Instruction::Add { reg, src1, src2 }
                | Instruction::Sub { reg, src1, src2 }
                | Instruction::Div { reg, src1, src2 }
                | Instruction::Mul { reg, src1, src2 } => {
                    let src1 = src1.get_val_ref(registers, stack, stack_offset);
                    let src2 = src2.get_val_ref(registers, stack, stack_offset);

                    let value = match instruction {
                        Instruction::Add { .. } => src1.add(&src2).unwrap(),
                        Instruction::Sub { .. } => src1.sub(&src2).unwrap(),
                        Instruction::Mul { .. } => src1.mul(&src2).unwrap(),
                        Instruction::Div { .. } => src1.div(&src2).unwrap(),
                        _ => unreachable!(),
                    };

                    *registers.get_mut(*reg) = Some(value);
                }
                Instruction::Neg { reg, src } | Instruction::Truthy { reg, src } => {
                    let src = src.get_val_ref(registers, stack, stack_offset);

                    let val = match instruction {
                        Instruction::Neg { .. } => src.neg().unwrap(),
                        Instruction::Truthy { .. } => src.not(),
                        _ => unreachable!(),
                    };

                    *registers.get_mut(*reg) = Some(val);
                }
            }

            // self.trace(stack, registers, instruction, stack_offset);

            *ip += 1;
        }

        Value::Void
    }
}
