use std::{ rc::Rc, time::{ Instant, SystemTime, UNIX_EPOCH } };

use colored::Colorize;

use crate::{ constants::REGISTERS, value::{ Value, ValueType } };

use super::helper_structs::{ VMRegisters, VMStack };
pub mod fast_instr;
pub mod big_instr;

#[derive(Debug, Copy, Clone)]
pub struct StackLocation {
    pub stack_pos: usize,
    pub is_relative: bool,
}

impl StackLocation {
    pub fn new(stack_pos: usize, is_relative: bool) -> Self {
        Self { stack_pos, is_relative }
    }

    pub fn from_tuple((stack_pos, is_relative): (usize, bool)) -> Self {
        Self { stack_pos, is_relative }
    }

    pub fn dissassemble(&self, stack_offset: Option<usize>) -> String {
        if self.is_relative {
            if let Some(stack_offset) = stack_offset {
                format!("[{}]", self.stack_pos + stack_offset)
            } else {
                format!("[{} + x]", self.stack_pos)
            }
        } else {
            format!("[{}]", self.stack_pos)
        }
    }

    #[inline]
    pub fn get_stack_pos(&self, stack_offset: usize) -> usize {
        if self.is_relative { self.stack_pos + stack_offset } else { self.stack_pos }
    }
}

#[derive(Debug, Clone)]
pub enum InstructionSrc {
    Register {
        reg: usize,
    },
    Constant {
        val: Value,
    },
    Stack(StackLocation),
}

impl InstructionSrc {
    pub fn dissassemble(&self, stack_offset: Option<usize>) -> String {
        match self {
            Self::Register { reg } => { format!("R{}", reg) }
            Self::Constant { val } => { format!("C({})", val.to_string()) }
            Self::Stack(stack_loc) => { stack_loc.dissassemble(stack_offset) }
        }
    }

    #[inline]
    pub fn get_val(
        &self,
        registers: &mut VMRegisters,
        stack: &mut VMStack,
        stack_offset: usize
    ) -> Value {
        match self {
            InstructionSrc::Register { reg } => registers.get(*reg),
            InstructionSrc::Constant { val } => val.clone(),
            InstructionSrc::Stack(stack_loc) => {
                stack.get_as_val(stack_loc.get_stack_pos(stack_offset))
            }
        }
    }

    pub fn get_val_mut_ref<'a>(
        &'a mut self,
        registers: &'a mut VMRegisters,
        stack: &'a mut VMStack,
        stack_offset: usize
    ) -> &'a mut Value {
        match self {
            InstructionSrc::Register { reg } => registers.get_mut(*reg).as_mut().unwrap(),
            InstructionSrc::Constant { val } => val,
            InstructionSrc::Stack(stack_loc) => {
                stack.get_mut_ref(stack_loc.get_stack_pos(stack_offset))
            }
        }
    }

    pub fn get_val_ref<'a>(
        &'a self,
        registers: &'a VMRegisters,
        stack: &'a VMStack,
        stack_offset: usize
    ) -> &'a Value {
        match self {
            InstructionSrc::Register { reg } => registers.get_ref(*reg),
            InstructionSrc::Constant { val } => val,
            InstructionSrc::Stack(stack_loc) => {
                stack.get_ref(stack_loc.get_stack_pos(stack_offset))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NativeCall {
    Print,
    Now,
}

impl NativeCall {
    pub fn get_native(lexeme: &Rc<str>) -> Option<Self> {
        match lexeme.as_ref() {
            "print" => Some(NativeCall::Print),
            "now" => Some(NativeCall::Now),
            _ => None,
        }
    }

    pub fn call(&self, values: &Vec<&Value>) -> Value {
        match self {
            Self::Print => {
                let mut print_string = String::new();
                for val in values {
                    print_string += val.to_string().as_str();
                }
                println!("{}", print_string);
                Value::Void
            }
            Self::Now => {
                // let now = SystemTime::now();
                // let int = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as i32

                Value::Int32(0)
            }
        }
    }

    pub fn dissassemble(&self) -> String {
        let str = match self {
            Self::Print => "PRINT",
            Self::Now => "NOW",
        };
        str.to_string()
    }

    pub fn get_lexeme(&self) -> String {
        let str = match self {
            Self::Print => "print",
            Self::Now => "now",
        };
        str.to_string()
    }

    pub fn get_args_count(&self) -> (usize, usize) {
        match self {
            Self::Print => (0, REGISTERS),
            Self::Now => (0, 0),
        }
    }

    pub fn get_return_type(&self) -> ValueType {
        match self {
            Self::Print => ValueType::Void,
            Self::Now => ValueType::Int32,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Halt,
    JmpPop {
        pos: usize,
        amount: usize,
    },
    NativeCall {
        stack_loc_dest: StackLocation,
        native_call: NativeCall,
        args_regs: Vec<usize>,
    },
    Call {
        stack_loc_dest: StackLocation,
        stack_loc_call: StackLocation,
    },
    Return {
        src: InstructionSrc,
    },
    ReturnPop {
        src: InstructionSrc,
        amount: usize,
    },
    Load {
        reg: usize,
        src: InstructionSrc,
    },
    Pop {
        amount: usize,
    },
    Add {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Sub {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Mul {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Div {
        reg: usize,
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Neg {
        reg: usize,
        src: InstructionSrc,
    },
    Truthy {
        reg: usize,
        src: InstructionSrc,
    },
    Define {
        stack_loc: StackLocation,
        src: InstructionSrc,
    },
    Assign {
        stack_loc: StackLocation,
        src: InstructionSrc,
    },
    Cmp {
        src1: InstructionSrc,
        src2: InstructionSrc,
    },
    Jmp {
        pos: usize,
    },
    JE {
        true_pos: usize,
        false_pos: usize,
    },
    JNE {
        true_pos: usize,
        false_pos: usize,
    },
    JG {
        true_pos: usize,
        false_pos: usize,
    },
    JGE {
        true_pos: usize,
        false_pos: usize,
    },
    JL {
        true_pos: usize,
        false_pos: usize,
    },
    JLE {
        true_pos: usize,
        false_pos: usize,
    },
}

impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        match value {
            Instruction::Halt => 0,
            Instruction::Add { .. } => 1,
            Instruction::Sub { .. } => 2,
            Instruction::Mul { .. } => 3,
            Instruction::Div { .. } => 4,
            Instruction::Jmp { .. } => 5,
            Instruction::JmpPop { .. } => 6,
            Instruction::Return { .. } => 7,
            Instruction::ReturnPop { .. } => 8,
            Instruction::Assign { .. } => 9,
            Instruction::Call { .. } => 10,
            Instruction::Cmp { .. } => 11,
            Instruction::Define { .. } => 12,
            Instruction::JE { .. } => 13,
            Instruction::JG { .. } => 14,
            Instruction::JGE { .. } => 15,
            Instruction::JL { .. } => 16,
            Instruction::JLE { .. } => 17,
            Instruction::JNE { .. } => 18,
            Instruction::Load { .. } => 19,
            Instruction::NativeCall { .. } => 20,
            Instruction::Neg { .. } => 21,
            Instruction::Truthy { .. } => 22,
            Instruction::Pop { .. } => 23,
        }
    }
}

impl Instruction {
    pub fn trace(&self, stack_offset: usize) {
        println!("{}", self.debug(Some(stack_offset)).as_str().blue())
    }

    pub fn dissassemble(&self) -> String {
        self.debug(None)
    }

    fn debug(&self, stack_offset: Option<usize>) -> String {
        match self {
            Self::NativeCall { native_call, .. } => {
                format!("NATIVE_CALL {} [...]", native_call.dissassemble())
            }
            Self::Call { stack_loc_dest, stack_loc_call } => {
                format!(
                    "CALL {} {}",
                    stack_loc_dest.dissassemble(stack_offset),
                    stack_loc_call.dissassemble(stack_offset)
                )
            }
            Self::Cmp { src1, src2 } => {
                format!(
                    "CMP {} {}",
                    src1.dissassemble(stack_offset),
                    src2.dissassemble(stack_offset)
                )
            }
            Self::Return { src } => { format!("RETURN {}", src.dissassemble(stack_offset)) }
            Self::ReturnPop { src, amount } => {
                format!("RETURN_POP {} {}", src.dissassemble(stack_offset), amount)
            }
            Self::Load { reg, src } => {
                format!("LOAD R{} {}", reg, src.dissassemble(stack_offset))
            }
            Self::Pop { amount } => { format!("POP {}", amount) }
            Self::Jmp { pos } => { format!("JMP {}", pos) }
            Self::JE { true_pos, false_pos } => { format!("JE {} {}", true_pos, false_pos) }
            Self::JNE { true_pos, false_pos } => { format!("JNE {} {}", true_pos, false_pos) }
            Self::JG { true_pos, false_pos } => { format!("JG {} {}", true_pos, false_pos) }
            Self::JGE { true_pos, false_pos } => { format!("JGE {} {}", true_pos, false_pos) }
            Self::JL { true_pos, false_pos } => { format!("JL {} {}", true_pos, false_pos) }
            Self::JLE { true_pos, false_pos } => { format!("JLE {} {}", true_pos, false_pos) }
            Self::Halt => { "HALT".to_string() }
            Self::JmpPop { pos, amount } => { format!("JMP_POP {} {}", pos, amount) }
            Self::Add { reg, src1, src2 } => {
                format!(
                    "ADD R{} {} {}",
                    reg,
                    src1.dissassemble(stack_offset),
                    src2.dissassemble(stack_offset)
                )
            }
            Self::Sub { reg, src1, src2 } => {
                format!(
                    "SUB R{} {} {}",
                    reg,
                    src1.dissassemble(stack_offset),
                    src2.dissassemble(stack_offset)
                )
            }
            Self::Mul { reg, src1, src2 } => {
                format!(
                    "MUL R{} {} {}",
                    reg,
                    src1.dissassemble(stack_offset),
                    src2.dissassemble(stack_offset)
                )
            }
            Self::Div { reg, src1, src2 } => {
                format!(
                    "DIV R{} {} {}",
                    reg,
                    src1.dissassemble(stack_offset),
                    src2.dissassemble(stack_offset)
                )
            }
            Self::Neg { reg, src } => { format!("NEG R{} {}", reg, src.dissassemble(stack_offset)) }
            Self::Truthy { reg, src } => {
                format!("TRUTHY R{} {}", reg, src.dissassemble(stack_offset))
            }
            Self::Define { stack_loc, src } => {
                format!(
                    "DEFINE {} {}",
                    stack_loc.dissassemble(stack_offset),
                    src.dissassemble(stack_offset)
                )
            }
            Self::Assign { stack_loc, src } => {
                format!(
                    "ASSIGN {} {}",
                    stack_loc.dissassemble(stack_offset),
                    src.dissassemble(stack_offset)
                )
            }
        }
    }
}
