use std::time::Instant;

use ahash::{ AHashMap, AHashSet };
use instructions::{ Instruction, Reg };
use optimized_instructions::OptimizedInstruction;

use crate::{ U16_MAX, U8_MAX };

pub mod instructions;
pub mod optimized_instructions;

pub struct VM {
    registers: [i64; U8_MAX],
    register_offset: usize,
    stack_frames: [StackFrame; U8_MAX],
    stack_frame_idx: usize,
    heap: Heap,
}

impl VM {
    pub fn new(registers: [i64; U8_MAX]) -> Self {
        Self {
            registers,
            register_offset: 0,
            heap: Heap::new(),
            stack_frames: [StackFrame::empty(); U8_MAX],
            stack_frame_idx: 0,
        }
    }

    pub fn run(&mut self, instructions: Vec<OptimizedInstruction>) {
        if instructions.len() > 0 {
            let now = Instant::now();
            unsafe {
                (instructions[0].handler)(instructions.as_ptr(), self);
            }
            println!("Program took: {:?}", now.elapsed())
        }
    }

    pub fn debug_run(&mut self, instructions: Vec<Instruction>) {
        #[cfg(debug_assertions)]
        println!("------- RUNTIME INFORMATION -------\n");

        if instructions.len() > 0 {
            let now = Instant::now();

            let mut i: usize = 0;

            loop {
                let instr = &instructions[i];

                macro_rules! set_reg {
                    ($reg:ident) => {
                        *(&mut self.registers[$reg + self.register_offset])
                    };
                }
                macro_rules! get_reg {
                    ($reg:ident) => {
                        {
                            let reg = match $reg {
                                Reg::Rel(reg) => *reg + self.register_offset,
                                Reg::Abs(reg) => *reg,
                            };
                            self.registers[reg]
                        }
                    };
                }
                macro_rules! next_op {
                    () => {
                        i += 1;
                    };
                    ($pos:literal) => {
                        i = $pos
                    };
                    ($pos:ident) => {
                        i = *$pos
                    };
                }

                let debug_str = match instr {
                    Instruction::Halt => {
                        break;
                    }
                    Instruction::AddInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = get_reg!(src1_reg) + get_reg!(src2_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} + {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::SubInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = get_reg!(src1_reg) - get_reg!(src2_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} - {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::DivInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = get_reg!(src1_reg) / get_reg!(src2_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} / {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::MulInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = get_reg!(src1_reg) * get_reg!(src2_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} * {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::NegInt { dst_reg, src_reg } => {
                        set_reg!(dst_reg) = -get_reg!(src_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("-{}", get_reg!(src_reg))
                    }
                    Instruction::NotInt { dst_reg, src_reg } => {
                        set_reg!(dst_reg) = !get_reg!(src_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("!{}", get_reg!(src_reg))
                    }
                    Instruction::CmpEqInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = (get_reg!(src1_reg) == get_reg!(src2_reg)) as i64;
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} == {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::CmpNeInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = (get_reg!(src1_reg) != get_reg!(src2_reg)) as i64;
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} != {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::CmpGeInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = (get_reg!(src1_reg) >= get_reg!(src2_reg)) as i64;
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} >= {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::CmpGtInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = (get_reg!(src1_reg) > get_reg!(src2_reg)) as i64;
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} > {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::CmpLeInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = (get_reg!(src1_reg) <= get_reg!(src2_reg)) as i64;
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} <= {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::CmpLtInt { dst_reg, src1_reg, src2_reg } => {
                        set_reg!(dst_reg) = (get_reg!(src1_reg) < get_reg!(src2_reg)) as i64;
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("{} < {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::Goto { jmp_pos } => {
                        next_op!(jmp_pos);
                        #[cfg(debug_assertions)]
                        format!("")
                    }
                    Instruction::JmpCmpEqInt {
                        true_jmp_pos,
                        false_jmp_pos,
                        src1_reg,
                        src2_reg,
                    } => {
                        match get_reg!(src1_reg) == get_reg!(src2_reg) {
                            true => next_op!(true_jmp_pos),
                            false => next_op!(false_jmp_pos),
                        }
                        #[cfg(debug_assertions)]
                        format!("{} == {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::JmpCmpNeInt {
                        true_jmp_pos,
                        false_jmp_pos,
                        src1_reg,
                        src2_reg,
                    } => {
                        match get_reg!(src1_reg) != get_reg!(src2_reg) {
                            true => next_op!(true_jmp_pos),
                            false => next_op!(false_jmp_pos),
                        }
                        #[cfg(debug_assertions)]
                        format!("{} != {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::JmpCmpGeInt {
                        true_jmp_pos,
                        false_jmp_pos,
                        src1_reg,
                        src2_reg,
                    } => {
                        match get_reg!(src1_reg) >= get_reg!(src2_reg) {
                            true => next_op!(true_jmp_pos),
                            false => next_op!(false_jmp_pos),
                        }
                        #[cfg(debug_assertions)]
                        format!("{} >= {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::JmpCmpGtInt {
                        true_jmp_pos,
                        false_jmp_pos,
                        src1_reg,
                        src2_reg,
                    } => {
                        match get_reg!(src1_reg) > get_reg!(src2_reg) {
                            true => next_op!(true_jmp_pos),
                            false => next_op!(false_jmp_pos),
                        }
                        #[cfg(debug_assertions)]
                        format!("{} > {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::JmpCmpLeInt {
                        true_jmp_pos,
                        false_jmp_pos,
                        src1_reg,
                        src2_reg,
                    } => {
                        match get_reg!(src1_reg) <= get_reg!(src2_reg) {
                            true => next_op!(true_jmp_pos),
                            false => next_op!(false_jmp_pos),
                        }
                        #[cfg(debug_assertions)]
                        format!("{} <= {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::JmpCmpLtInt {
                        true_jmp_pos,
                        false_jmp_pos,
                        src1_reg,
                        src2_reg,
                    } => {
                        match get_reg!(src1_reg) < get_reg!(src2_reg) {
                            true => next_op!(true_jmp_pos),
                            false => next_op!(false_jmp_pos),
                        }
                        #[cfg(debug_assertions)]
                        format!("{} < {}", get_reg!(src1_reg), get_reg!(src2_reg))
                    }
                    Instruction::Copy { dst_reg, src_reg } => {
                        set_reg!(dst_reg) = get_reg!(src_reg);
                        next_op!();
                        #[cfg(debug_assertions)]
                        format!("copy {}", get_reg!(src_reg))
                    }
                };
                #[cfg(debug_assertions)]
                {
                    println!(
                        "{}{}@ At #{} in {}",
                        debug_str,
                        " ".repeat(15 - debug_str.len()),
                        i,
                        instr.to_op_name()
                    );
                }
            }

            let elapsed = now.elapsed();
            #[cfg(debug_assertions)]
            println!("\n-----------------------------------\n");

            println!("Program took: {:?}", elapsed);
        }
    }

    pub fn read_register_offset(&self) -> usize {
        self.register_offset
    }

    pub fn print_regs(&self) {
        for (i, reg_value) in self.registers.iter().enumerate() {
            if *reg_value != -2937 {
                println!("R{}: {}", i, *reg_value);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct StackFrame {
    call_instr_ptr: *const OptimizedInstruction,
    next_instr_ptr: *const OptimizedInstruction,
    register_offset: usize,
}

impl StackFrame {
    pub fn empty() -> Self {
        Self {
            call_instr_ptr: std::ptr::null(),
            next_instr_ptr: std::ptr::null(),
            register_offset: 0,
        }
    }

    pub fn new(
        call_instr_ptr: *const OptimizedInstruction,
        next_instr_ptr: *const OptimizedInstruction,
        register_offset: usize
    ) -> Self {
        Self {
            call_instr_ptr,
            next_instr_ptr,
            register_offset,
        }
    }
}

#[derive(Debug)]
pub struct Heap {
    heap: AHashMap<usize, (HeapValue, HeapReferences)>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            heap: AHashMap::new(),
        }
    }

    pub fn get(&self, idx: &usize) -> &HeapValue {
        &self.heap.get(idx).unwrap().0
    }

    pub fn clean_up(&mut self, idx1: &usize, idx2: &usize) {
        let v1 = self.heap.get_mut(idx1).unwrap();
        v1.1.dec();
        if v1.1.references == 0 {
            self.heap.remove(idx1);
        }

        let v2 = self.heap.get_mut(idx2).unwrap();
        v2.1.dec();
        if v2.1.references == 0 {
            self.heap.remove(idx2);
        }
    }

    pub fn insert(&mut self, idx: usize, heap_value: HeapValue) {
        self.heap.insert(idx, (heap_value, HeapReferences::new()));
    }

    pub fn insert_string(&mut self, idx: usize, heap_value: Vec<u8>) {
        self.heap.insert(idx, (HeapValue::String(heap_value), HeapReferences::new()));
    }
}

#[derive(Debug)]
pub enum HeapValue {
    String(Vec<u8>),
}

impl HeapValue {
    pub fn new_string(str: &str) -> Self {
        Self::String(str.as_bytes().to_vec())
    }

    pub fn unwrap_as_string_bytes(&self) -> &Vec<u8> {
        match self {
            Self::String(string) => string,
        }
    }
}

#[derive(Debug)]
pub struct HeapReferences {
    references: usize,
}

impl HeapReferences {
    pub fn new() -> Self {
        Self {
            references: 1,
        }
    }

    pub fn inc(&mut self) {
        self.references += 1;
    }

    pub fn dec(&mut self) {
        self.references -= 1;
    }
}
