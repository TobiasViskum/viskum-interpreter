use std::{ mem, ops::Index, time::Instant };
use std::marker::PhantomData;
use instructions::{
    AddInstruction,
    CallInstruction,
    CmpLtJmpInstruction,
    HaltInstruction,
    Instruction,
    Instruction2,
    Opcode,
    Operand,
    PushFunctionInstruction,
    PushInstrPtrInstruction,
    PushStackInstruction,
    RetInstruction,
    Src1,
    SubInstruction,
};

use rules_approach_3::RULES_3;
use rules_approach_4::RULES_4;
use vm_util::{ ConstPtr, Dst, Register, RegisterValue, Src, Stack, StackPtr, StackValue, Value };
mod instructions;
mod vm_util;
mod rules_approach_3;
mod rules_approach_4;

fn main() {
    const FIB: i32 = 35;

    let constant_pool = &vec![Value::Int32(FIB), Value::Int32(2), Value::Int32(1)];

    let instructions1 = &vec![
        Instruction::PushFunction { instructions_amount: 8 },
        Instruction::CmpLJmp {
            src1: Src::StackPointer(StackPtr::new(0, true)),
            src2: Src::ConstantPointer(1),
            true_pos: 2,
            false_pos: 3,
        },
        // Instruction::CmpLJmp {
        //     src1: StackPtr::new(0, true),
        //     src2: ConstPtr::new(1),
        //     true_pos: 2,
        //     false_pos: 3,
        // },
        Instruction::RetStack { stack_ptr: StackPtr::new(0, true) }, // { src: Src::StackPointer(StackPtr::new(0, true)) },
        Instruction::Sub {
            dst: Dst::StackPush,
            src1: StackPtr::new(0, true),
            src2: ConstPtr::new(1),
        },
        // Instruction::Sub {
        //     dst: Dst::StackPush,
        //     src1: Src::StackPointer(StackPtr::new(0, true)),
        //     src2: Src::ConstantPointer(1),
        // },
        Instruction::Call { stack_ptr: StackPtr::new(0, false), args_count: 1, tco: false },
        Instruction::Sub {
            dst: Dst::StackPush,
            src1: StackPtr::new(0, true),
            src2: ConstPtr::new(2),
        },
        // Instruction::Sub {
        //     dst: Dst::StackPush,
        //     src1: Src::StackPointer(StackPtr::new(0, true)),
        //     src2: Src::ConstantPointer(2),
        // },
        Instruction::Call { stack_ptr: StackPtr::new(0, false), args_count: 1, tco: false },
        Instruction::Add {
            dst: Register::R0,
            src1: StackPtr::new(1, true),
            src2: StackPtr::new(2, true),
        },
        // Instruction::Add {
        //     dst: Dst::Register(Register::R0),
        //     src1: Src::StackPointer(StackPtr::new(1, true)),
        //     src2: Src::StackPointer(StackPtr::new(2, true)),
        // },
        Instruction::RetRegister { reg: Register::R0 }, // { src: Src::Register(Register::R0) },
        Instruction::PushStackConst { const_ptr: ConstPtr::new(0) }, // { src: Src::ConstantPointer(0) },
        Instruction::Call { stack_ptr: StackPtr::new(0, false), args_count: 1, tco: false },
        Instruction::Halt
    ];

    const EMPTY_OPERAND: Operand = Operand::Empty;

    let instructions3: &[(usize, [Operand; 4])] = &vec![
        (
            Opcode::PushFunction as usize,
            [Operand::InstrsAmount(12), EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND],
        ),
        (
            Opcode::CmpLtJmp as usize,
            [
                Operand::Src(Src::StackPointer(StackPtr::new(0, true))),
                Operand::Src(Src::ConstantPointer(1)),
                Operand::JmpPos(2),
                Operand::JmpPos(3),
            ],
        ),
        (
            Opcode::Ret as usize,
            [
                Operand::Src(Src::StackPointer(StackPtr::new(0, true))),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::Sub as usize,
            [
                Operand::Dst(Register::R0),
                Operand::Src(Src::StackPointer(StackPtr::new(0, true))),
                Operand::Src(Src::ConstantPointer(1)),
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::PushInstrPtr as usize,
            [Operand::InstrPtr(7), EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND],
        ),
        (
            Opcode::PushStack as usize,
            [
                Operand::Src(Src::Register(Register::R0)),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::Call as usize,
            [
                Operand::StackPtr(StackPtr::new(0, false)),
                Operand::ArgsCount(1),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::Sub as usize,
            [
                Operand::Dst(Register::R1),
                Operand::Src(Src::StackPointer(StackPtr::new(0, true))),
                Operand::Src(Src::ConstantPointer(2)),
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::PushInstrPtr as usize,
            [Operand::InstrPtr(11), EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND],
        ),
        (
            Opcode::PushStack as usize,
            [
                Operand::Src(Src::Register(Register::R1)),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::Call as usize,
            [
                Operand::StackPtr(StackPtr::new(0, false)),
                Operand::ArgsCount(1),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::Add as usize,
            [
                Operand::Dst(Register::R0),
                Operand::Src(Src::StackPointer(StackPtr::new(1, true))),
                Operand::Src(Src::StackPointer(StackPtr::new(2, true))),
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::Ret as usize,
            [
                Operand::Src(Src::Register(Register::R0)),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (
            Opcode::PushInstrPtr as usize,
            [Operand::InstrPtr(16), EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND],
        ),
        (
            Opcode::PushStack as usize,
            [Operand::Src(Src::ConstantPointer(0)), EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND],
        ),
        (
            Opcode::Call as usize,
            [
                Operand::StackPtr(StackPtr::new(0, false)),
                Operand::ArgsCount(1),
                EMPTY_OPERAND,
                EMPTY_OPERAND,
            ],
        ),
        (Opcode::Halt as usize, [EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND, EMPTY_OPERAND])
    ];

    let instructions2 = &vec![
        Instruction2::PushFunction(PushFunctionInstruction::new(12)),
        Instruction2::CmpLtJmp(
            CmpLtJmpInstruction::new(
                Src::StackPointer(StackPtr::new(0, true)),
                Src::ConstantPointer(1),
                2,
                3
            )
        ),
        Instruction2::Ret(RetInstruction::new(Src::StackPointer(StackPtr::new(0, true)))),
        Instruction2::Sub(
            SubInstruction::new(
                Register::R0,
                Src::StackPointer(StackPtr::new(0, true)),
                Src::ConstantPointer(1)
            )
        ),
        Instruction2::PushInstrPtr(PushInstrPtrInstruction::new(7)),
        Instruction2::PushStack(PushStackInstruction::new(Src::Register(Register::R0))),
        Instruction2::Call(CallInstruction::new(StackPtr::new(0, false), 1)),
        Instruction2::Sub(
            SubInstruction::new(
                Register::R1,
                Src::StackPointer(StackPtr::new(0, true)),
                Src::ConstantPointer(2)
            )
        ),
        Instruction2::PushInstrPtr(PushInstrPtrInstruction::new(11)),
        Instruction2::PushStack(PushStackInstruction::new(Src::Register(Register::R1))),
        Instruction2::Call(CallInstruction::new(StackPtr::new(0, false), 1)),
        Instruction2::Add(
            AddInstruction::new(
                Register::R0,
                Src::StackPointer(StackPtr::new(1, true)),
                Src::StackPointer(StackPtr::new(2, true))
            )
        ),
        Instruction2::Ret(RetInstruction::new(Src::Register(Register::R0))),
        Instruction2::PushInstrPtr(PushInstrPtrInstruction::new(16)),
        Instruction2::PushStack(PushStackInstruction::new(Src::ConstantPointer(0))),
        Instruction2::Call(CallInstruction::new(StackPtr::new(0, false), 1)),
        Instruction2::Halt(HaltInstruction::new())
    ];

    let instructions4 = &vec![
        (
            Opcode::PushFunction as usize,
            Instruction2::PushFunction(PushFunctionInstruction::new(12)),
        ),
        (
            Opcode::CmpLtJmp as usize,
            Instruction2::CmpLtJmp(
                CmpLtJmpInstruction::new(
                    Src::StackPointer(StackPtr::new(0, true)),
                    Src::ConstantPointer(1),
                    2,
                    3
                )
            ),
        ),
        (
            Opcode::Ret as usize,
            Instruction2::Ret(RetInstruction::new(Src::StackPointer(StackPtr::new(0, true)))),
        ),
        (
            Opcode::Sub as usize,
            Instruction2::Sub(
                SubInstruction::new(
                    Register::R0,
                    Src::StackPointer(StackPtr::new(0, true)),
                    Src::ConstantPointer(1)
                )
            ),
        ),
        (
            Opcode::PushInstrPtr as usize,
            Instruction2::PushInstrPtr(PushInstrPtrInstruction::new(7)),
        ),
        (
            Opcode::PushStack as usize,
            Instruction2::PushStack(PushStackInstruction::new(Src::Register(Register::R0))),
        ),
        (
            Opcode::Call as usize,
            Instruction2::Call(CallInstruction::new(StackPtr::new(0, false), 1)),
        ),
        (
            Opcode::Sub as usize,
            Instruction2::Sub(
                SubInstruction::new(
                    Register::R1,
                    Src::StackPointer(StackPtr::new(0, true)),
                    Src::ConstantPointer(2)
                )
            ),
        ),
        (
            Opcode::PushInstrPtr as usize,
            Instruction2::PushInstrPtr(PushInstrPtrInstruction::new(11)),
        ),
        (
            Opcode::PushStack as usize,
            Instruction2::PushStack(PushStackInstruction::new(Src::Register(Register::R1))),
        ),
        (
            Opcode::Call as usize,
            Instruction2::Call(CallInstruction::new(StackPtr::new(0, false), 1)),
        ),
        (
            Opcode::Add as usize,
            Instruction2::Add(
                AddInstruction::new(
                    Register::R0,
                    Src::StackPointer(StackPtr::new(1, true)),
                    Src::StackPointer(StackPtr::new(2, true))
                )
            ),
        ),
        (Opcode::Ret as usize, Instruction2::Ret(RetInstruction::new(Src::Register(Register::R0)))),
        (
            Opcode::PushInstrPtr as usize,
            Instruction2::PushInstrPtr(PushInstrPtrInstruction::new(16)),
        ),
        (
            Opcode::PushStack as usize,
            Instruction2::PushStack(PushStackInstruction::new(Src::ConstantPointer(0))),
        ),
        (
            Opcode::Call as usize,
            Instruction2::Call(CallInstruction::new(StackPtr::new(0, false), 1)),
        ),
        (Opcode::Halt as usize, Instruction2::Halt(HaltInstruction::new()))
    ];

    let mut vm = VM::new(instructions1, constant_pool);
    let now = Instant::now();
    vm.run_1();

    println!("{:?}", now.elapsed());
    println!("{:?}", vm.stack)

    // vm.run_all(instructions1, instructions2, instructions3, instructions4);
}

struct VM<'a> {
    instructions: &'a [Instruction],
    constant_pool: &'a [Value],
    stack: Stack,
    registers: Registers,
    instr_ptrs: Vec<usize>,
}

impl<'a> Iterator for VM<'a> {
    type Item = &'a Instruction;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.instructions.len()))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let instr = unsafe { self.instructions.get_unchecked(self.registers.get_ip()) };

        match instr {
            Instruction::Call { stack_ptr, args_count, tco } => {
                self.stack.push_stack_frame(self.stack.len() - *args_count);

                self.instr_ptrs.push(self.registers.get_ip() + 1);

                let fn_ptr = self.stack.get(stack_ptr).unwrap_fn_ptr();

                self.set_ip(|_| fn_ptr + 1);
            }
            Instruction::RetStack { stack_ptr } => {
                let v1 = *self.stack.get(stack_ptr);

                self.stack.truncate(self.stack.read_stack_frame());
                self.stack.pop_stack_frame();

                let instr_ptr = self.instr_ptrs.pop().unwrap();

                self.stack.push(v1);

                self.set_ip(|_| instr_ptr);
            }
            Instruction::RetRegister { reg } => {
                let v1 = *self.registers.get_reg(*reg).get_val(self.constant_pool);

                self.stack.truncate(self.stack.read_stack_frame());
                self.stack.pop_stack_frame();

                let instr_ptr = self.instr_ptrs.pop().unwrap();

                self.stack.push(StackValue::Value(v1));

                self.set_ip(|_| instr_ptr);
            }
            Instruction::RetConst { const_ptr } => {
                let v1 = unsafe { *self.constant_pool.get_unchecked(const_ptr.get_idx()) };

                self.stack.truncate(self.stack.read_stack_frame());
                self.stack.pop_stack_frame();

                let instr_ptr = self.instr_ptrs.pop().unwrap();

                self.stack.push(StackValue::Value(v1));

                self.set_ip(|_| instr_ptr);
            }
            Instruction::PushFunction { instructions_amount } => {
                // THREAD
                self.stack.push(StackValue::FnPtr(self.get_ip()));

                self.set_ip(|prev| prev + instructions_amount + 1);
            }
            Instruction::PushStackConst { const_ptr } => {
                let v1 = unsafe { *self.constant_pool.get_unchecked(const_ptr.get_idx()) };
                // let v1 = *src.get_val(self.constant_pool, &self.stack, &self.registers);

                self.stack.push(StackValue::Value(v1));

                self.inc_ip();
            }
            Instruction::PushStackRegister { reg } => {
                let v1 = *self.registers.get_reg(*reg).get_val(&self.constant_pool);
                // let v1 = *src.get_val(self.constant_pool, &self.stack, &self.registers);

                self.stack.push(StackValue::Value(v1));

                self.inc_ip();
            }
            Instruction::Add { dst, src1, src2 } => {
                let (v1, v2) = (
                    self.stack.get(src1).unwrap_val(),
                    self.stack.get(src2).unwrap_val(),
                );
                // let v2 = self.stack.get(src2).unwrap_val();

                // let vals = [src1, src2]
                //     .iter()
                //     .map(|src| self.stack.get(*src).unwrap_val())
                //     .collect::<Vec<_>>();

                // let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                // let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = v1.add(v2);
                // let result = unsafe { vals.get_unchecked(0).add(vals.get_unchecked(1)) };

                // match dst {
                //     Dst::Register(reg) =>
                self.registers.set_reg(*dst, RegisterValue::Value(result));
                //     Dst::StackPush => self.stack.push(StackValue::Value(result)),
                // }

                self.inc_ip();
            }
            Instruction::Sub { dst, src1, src2 } => {
                let (v1, v2) = (
                    self.stack.get(src1).unwrap_val(),
                    unsafe { self.constant_pool.get_unchecked(src2.get_idx()) },
                );

                // let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                // let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = v1.sub(v2);
                // match dst {
                //     Dst::Register(reg) =>
                //         self.registers.set_reg(*reg, RegisterValue::Value(result)),
                /* Dst::StackPush => */ self.stack.push(StackValue::Value(result));
                // }

                self.inc_ip();
            }

            Instruction::Mul { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = v1.mul(v2);
                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }

            Instruction::Div { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = v1.div(v2);
                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::CmpEJmp { src1, src2, true_pos, false_pos } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                match v1.cmp_e(v2) {
                    true => self.set_ip(|_| *true_pos),
                    false => self.set_ip(|_| *false_pos),
                }
            }
            Instruction::CmpNeJmp { src1, src2, true_pos, false_pos } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                match v1.cmp_ne(v2) {
                    true => self.set_ip(|_| *true_pos),
                    false => self.set_ip(|_| *false_pos),
                }
            }
            Instruction::CmpLJmp { src1, src2, true_pos, false_pos } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                match v1.cmp_l(v2) {
                    true => self.set_ip(|_| *true_pos),
                    false => self.set_ip(|_| *false_pos),
                }
            }
            Instruction::CmpLeJmp { src1, src2, true_pos, false_pos } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                match v1.cmp_l(v2) {
                    true => self.set_ip(|_| *true_pos),
                    false => self.set_ip(|_| *false_pos),
                }
            }
            Instruction::CmpGJmp { src1, src2, true_pos, false_pos } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                match v1.cmp_g(v2) {
                    true => self.set_ip(|_| *true_pos),
                    false => self.set_ip(|_| *false_pos),
                }
            }
            Instruction::CmpGeJmp { src1, src2, true_pos, false_pos } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                match v1.cmp_ge(v2) {
                    true => self.set_ip(|_| *true_pos),
                    false => self.set_ip(|_| *false_pos),
                }
            }
            Instruction::CmpE { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = Value::Bool(v1.cmp_e(v2));

                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::CmpNe { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = Value::Bool(v1.cmp_ne(v2));

                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::CmpL { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = Value::Bool(v1.cmp_l(v2));

                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::CmpLe { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = Value::Bool(v1.cmp_l(v2));

                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::CmpG { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = Value::Bool(v1.cmp_g(v2));

                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::CmpGe { dst, src1, src2 } => {
                let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
                let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

                let result = Value::Bool(v1.cmp_ge(v2));

                match dst {
                    Dst::Register(reg) =>
                        self.registers.set_reg(*reg, RegisterValue::Value(result)),
                    Dst::StackPush => self.stack.push(StackValue::Value(result)),
                }

                self.inc_ip();
            }
            Instruction::Halt => {
                self.inc_ip();
                return None;
            }
        }

        Some(instr)
    }
}

impl<'a> VM<'a> {
    pub fn new(instructions: &'a [Instruction], constant_pool: &'a [Value]) -> Self {
        Self {
            instructions,
            stack: Stack::new(),
            constant_pool,
            registers: Registers::new(),
            instr_ptrs: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.stack = Stack::new();
        self.registers = Registers::new();
    }

    pub fn run_1(&mut self) {
        self.into_iter().for_each(|_| {})

        // while self.registers.get_ip() < self.instructions.len() {
        //     let instr = unsafe { self.instructions.get_unchecked(self.registers.get_ip()) };

        //     match instr {
        //         Instruction::Call { stack_ptr, args_count, tco } => {
        //             self.stack.push_stack_frame(self.stack.len() - *args_count);

        //             self.instr_ptrs.push(self.registers.get_ip() + 1);

        //             let fn_ptr = self.stack.get(stack_ptr).unwrap_fn_ptr();

        //             self.set_ip(|_| fn_ptr + 1);
        //         }
        //         Instruction::RetStack { stack_ptr } => {
        //             let v1 = *self.stack.get(stack_ptr);

        //             self.stack.truncate(self.stack.read_stack_frame());
        //             self.stack.pop_stack_frame();

        //             let instr_ptr = self.instr_ptrs.pop().unwrap();

        //             self.stack.push(v1);

        //             self.set_ip(|_| instr_ptr);
        //         }
        //         Instruction::RetRegister { reg } => {
        //             let v1 = *self.registers.get_reg(*reg).get_val(self.constant_pool);

        //             self.stack.truncate(self.stack.read_stack_frame());
        //             self.stack.pop_stack_frame();

        //             let instr_ptr = self.instr_ptrs.pop().unwrap();

        //             self.stack.push(StackValue::Value(v1));

        //             self.set_ip(|_| instr_ptr);
        //         }
        //         Instruction::RetConst { const_ptr } => {
        //             let v1 = unsafe { *self.constant_pool.get_unchecked(const_ptr.get_idx()) };

        //             self.stack.truncate(self.stack.read_stack_frame());
        //             self.stack.pop_stack_frame();

        //             let instr_ptr = self.instr_ptrs.pop().unwrap();

        //             self.stack.push(StackValue::Value(v1));

        //             self.set_ip(|_| instr_ptr);
        //         }
        //         Instruction::PushFunction { instructions_amount } => {
        //             // THREAD
        //             self.stack.push(StackValue::FnPtr(self.get_ip()));

        //             self.set_ip(|prev| prev + instructions_amount + 1);
        //         }
        //         Instruction::PushStackConst { const_ptr } => {
        //             let v1 = unsafe { *self.constant_pool.get_unchecked(const_ptr.get_idx()) };
        //             // let v1 = *src.get_val(self.constant_pool, &self.stack, &self.registers);

        //             self.stack.push(StackValue::Value(v1));

        //             self.inc_ip();
        //         }
        //         Instruction::PushStackRegister { reg } => {
        //             let v1 = *self.registers.get_reg(*reg).get_val(&self.constant_pool);
        //             // let v1 = *src.get_val(self.constant_pool, &self.stack, &self.registers);

        //             self.stack.push(StackValue::Value(v1));

        //             self.inc_ip();
        //         }
        //         Instruction::Add { dst, src1, src2 } => {
        //             let (v1, v2) = (
        //                 self.stack.get(src1).unwrap_val(),
        //                 self.stack.get(src2).unwrap_val(),
        //             );
        //             // let v2 = self.stack.get(src2).unwrap_val();

        //             // let vals = [src1, src2]
        //             //     .iter()
        //             //     .map(|src| self.stack.get(*src).unwrap_val())
        //             //     .collect::<Vec<_>>();

        //             // let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             // let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = v1.add(v2);
        //             // let result = unsafe { vals.get_unchecked(0).add(vals.get_unchecked(1)) };

        //             // match dst {
        //             //     Dst::Register(reg) =>
        //             self.registers.set_reg(*dst, RegisterValue::Value(result));
        //             //     Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             // }

        //             self.inc_ip();
        //         }
        //         Instruction::Sub { dst, src1, src2 } => {
        //             let (v1, v2) = (
        //                 self.stack.get(src1).unwrap_val(),
        //                 unsafe { self.constant_pool.get_unchecked(src2.get_idx()) },
        //             );

        //             // let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             // let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = v1.sub(v2);
        //             // match dst {
        //             //     Dst::Register(reg) =>
        //             //         self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //             /* Dst::StackPush => */ self.stack.push(StackValue::Value(result));
        //             // }

        //             self.inc_ip();
        //         }

        //         Instruction::Mul { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = v1.mul(v2);
        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }

        //         Instruction::Div { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = v1.div(v2);
        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::CmpEJmp { src1, src2, true_pos, false_pos } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             match v1.cmp_e(v2) {
        //                 true => self.set_ip(|_| *true_pos),
        //                 false => self.set_ip(|_| *false_pos),
        //             }
        //         }
        //         Instruction::CmpNeJmp { src1, src2, true_pos, false_pos } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             match v1.cmp_ne(v2) {
        //                 true => self.set_ip(|_| *true_pos),
        //                 false => self.set_ip(|_| *false_pos),
        //             }
        //         }
        //         Instruction::CmpLJmp { src1, src2, true_pos, false_pos } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             match v1.cmp_l(v2) {
        //                 true => self.set_ip(|_| *true_pos),
        //                 false => self.set_ip(|_| *false_pos),
        //             }
        //         }
        //         Instruction::CmpLeJmp { src1, src2, true_pos, false_pos } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             match v1.cmp_l(v2) {
        //                 true => self.set_ip(|_| *true_pos),
        //                 false => self.set_ip(|_| *false_pos),
        //             }
        //         }
        //         Instruction::CmpGJmp { src1, src2, true_pos, false_pos } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             match v1.cmp_g(v2) {
        //                 true => self.set_ip(|_| *true_pos),
        //                 false => self.set_ip(|_| *false_pos),
        //             }
        //         }
        //         Instruction::CmpGeJmp { src1, src2, true_pos, false_pos } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             match v1.cmp_ge(v2) {
        //                 true => self.set_ip(|_| *true_pos),
        //                 false => self.set_ip(|_| *false_pos),
        //             }
        //         }
        //         Instruction::CmpE { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = Value::Bool(v1.cmp_e(v2));

        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::CmpNe { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = Value::Bool(v1.cmp_ne(v2));

        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::CmpL { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = Value::Bool(v1.cmp_l(v2));

        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::CmpLe { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = Value::Bool(v1.cmp_l(v2));

        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::CmpG { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = Value::Bool(v1.cmp_g(v2));

        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::CmpGe { dst, src1, src2 } => {
        //             let v1 = src1.get_val(self.constant_pool, &self.stack, &self.registers);
        //             let v2 = src2.get_val(self.constant_pool, &self.stack, &self.registers);

        //             let result = Value::Bool(v1.cmp_ge(v2));

        //             match dst {
        //                 Dst::Register(reg) =>
        //                     self.registers.set_reg(*reg, RegisterValue::Value(result)),
        //                 Dst::StackPush => self.stack.push(StackValue::Value(result)),
        //             }

        //             self.inc_ip();
        //         }
        //         Instruction::Halt => {
        //             break;
        //         }
        //     }
        // }
    }

    pub fn run_all(
        &'a mut self,
        instructions1: &'a [Instruction],
        instructions2: &'a [Instruction2],
        instructions3: &'a [(usize, [Operand; 4])],
        instructions4: &'a [(usize, Instruction2)]
    ) {
        // let now = Instant::now();
        // self.run_1(instructions1);
        // let elapsed = now.elapsed();

        // println!("Approach 1 took: {:?}", elapsed);
        // println!("Result: {:?}\n\n", self.stack);

        // self.reset();

        // let now = Instant::now();
        // self.run_2(instructions2);
        // let elapsed = now.elapsed();

        // println!("Approach 2 took: {:?}", elapsed);
        // println!("Result: {:?}\n\n", self.stack);

        // self.reset();

        // let now = Instant::now();
        // self.run_3(instructions3);
        // let elapsed = now.elapsed();

        // println!("Approach 3 took: {:?}", elapsed);
        // println!("Result: {:?}\n\n", self.stack);

        // self.reset();

        // let now = Instant::now();
        // self.run_4(instructions4);
        // let elapsed = now.elapsed();

        // println!("Approach 4 took: {:?}", elapsed);
        // println!("Result: {:?}\n\n", self.stack);

        // self.reset();
    }

    fn run_2(&mut self, instructions: &[Instruction2]) {
        // let mut hash_table: AHashMap<Instruction2, > = AHashMap::new();

        while self.registers.get_ip() < instructions.len() {
            let instr = unsafe { instructions.get_unchecked(self.registers.get_ip()) };

            match instr {
                Instruction2::Halt(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::Add(instr) => {
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers);
                }
                Instruction2::Sub(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::Mul(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::Div(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::PushFunction(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::CmpLtJmp(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::Call(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::Ret(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::PushStack(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
                Instruction2::PushInstrPtr(instr) =>
                    instr.execute(self.constant_pool, &mut self.stack, &mut self.registers),
            }
        }
    }

    fn run_3(&mut self, instructions: &[(usize, [Operand; 4])]) {
        while self.registers.get_ip() < instructions.len() {
            let (op_idx, operands) = unsafe { instructions.get_unchecked(self.get_ip()) };

            unsafe {
                RULES_3.get_unchecked(*op_idx)(
                    operands,
                    self.constant_pool,
                    &mut self.stack,
                    &mut self.registers
                );
            }
        }
    }

    fn run_4(&mut self, instructions4: &[(usize, Instruction2)]) {
        while self.registers.get_ip() < instructions4.len() {
            let (opcode, instr) = instructions4.get(self.registers.get_ip()).unwrap();

            unsafe {
                RULES_4.get_unchecked(*opcode)(
                    instr,
                    self.constant_pool,
                    &mut self.stack,
                    &mut self.registers
                );
            }
        }
    }

    #[inline]
    fn get_ip(&self) -> usize {
        self.registers.get_ip()
    }

    #[inline]
    fn inc_ip(&mut self) {
        self.registers.set_ip(self.registers.get_ip() + 1)
    }

    #[inline]
    fn set_ip(&mut self, mut cb: impl FnMut(usize) -> usize) {
        self.registers.set_ip(cb(self.get_ip()));
    }
}

struct Registers {
    r0: RegisterValue,
    r1: RegisterValue,
    r2: RegisterValue,
    r3: RegisterValue,
    ip: usize,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r0: RegisterValue::ConstantPointer(0),
            r1: RegisterValue::ConstantPointer(0),
            r2: RegisterValue::ConstantPointer(0),
            r3: RegisterValue::ConstantPointer(0),
            ip: 0,
        }
    }

    #[inline]
    pub fn get_reg(&self, reg: Register) -> &RegisterValue {
        match reg {
            Register::R0 => &self.r0,
            Register::R1 => &self.r1,
            Register::R2 => &self.r2,
            Register::R3 => &self.r3,
        }
    }

    #[inline]
    pub fn set_reg(&mut self, reg: Register, value: RegisterValue) {
        match reg {
            Register::R0 => self.set_r0(value),
            Register::R1 => self.set_r1(value),
            Register::R2 => self.set_r2(value),
            Register::R3 => self.set_r3(value),
        }
    }

    #[inline]
    fn set_r0(&mut self, value: RegisterValue) {
        self.r0 = value;
    }

    #[inline]
    fn set_r1(&mut self, value: RegisterValue) {
        self.r1 = value;
    }

    #[inline]
    fn set_r2(&mut self, value: RegisterValue) {
        self.r2 = value;
    }

    #[inline]
    fn set_r3(&mut self, value: RegisterValue) {
        self.r3 = value;
    }

    #[inline]
    pub fn get_ip(&self) -> usize {
        self.ip
    }

    #[inline]
    pub fn set_ip(&mut self, new_ip: usize) {
        self.ip = new_ip;
    }

    #[inline]
    pub fn add_ip(&mut self, add_ip: usize) {
        self.ip += add_ip;
    }

    #[inline]
    pub fn inc_ip(&mut self) {
        self.ip += 1;
    }
}
