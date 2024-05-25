use std::alloc::dealloc;
use std::alloc::Layout;
use std::ptr;
use std::rc::Rc;
use std::time::Instant;

use compiler::Compiler;
use error_handler::ErrorHandler;
use parser::Parser;
use vm::helper_structs::ConstantsTable;
use vm::VM;

use crate::vm::instructions::Instruction;
use crate::value::Value;
use crate::vm::instructions::InstructionSrc;
use crate::vm::instructions::StackLocation;
use crate::value::Function;
use crate::vm::vm2::VM2;
use crate::vm::instructions::fast_instr::FastInstr;
use crate::vm::instructions::fast_instr::Opcode;

#[cfg(test)]
mod tests;

mod ast;
mod compiler;
mod error_handler;
mod parser;
mod vm;
// mod value;
mod constants;
mod operations;
mod util;
mod value;

/*
MUL R0 2 3
ADD R1 1 R0
MUL R0 4 5
ADD R2 R1 R0
ADD R0 3 6
MUL R1 6 R0
ADD R0 R2 R1
ADD R1 1 1
DIV R2 4 R1
ADD R1 R0 R2
*/

/*
MUL R9 2 3
ADD R8 1 R9
MUL R7 4 5
ADD R6 R8 R7
ADD R5 3 6
MUL R4 6 R5
ADD R3 R6 R4
ADD R2 1 1
DIV R1 4 R2
ADD R0 R3 R1
*/

#[profiler::start(true)]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }

    let file_content = match std::fs::read_to_string(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    let file_str = file_content.as_str();

    let src = file_str;

    let error_handler = &mut ErrorHandler::new();

    let ast = {
        let src_chars = src.chars().collect::<Vec<_>>();

        let mut parser = Parser::new(&src_chars, error_handler);
        let mut ast = parser.parse_to_ast();

        ast.type_check(error_handler);
        ast
    };

    if error_handler.has_error() {
        error_handler.print_errors(src);
        std::process::exit(1);
    }

    let instructions = {
        let mut compiler = Compiler::new(error_handler);
        compiler.compile(ast)
    };

    if let Some(instructions) = instructions {
        let fn_instructions = vec![
            Instruction::Define {
                stack_loc: StackLocation::new(0, true),
                src: InstructionSrc::Register { reg: 0 },
            },
            Instruction::Cmp {
                src1: InstructionSrc::Stack(StackLocation::new(0, true)),
                src2: InstructionSrc::Constant { val: Value::Int32(2) },
            },
            Instruction::JL { true_pos: 3, false_pos: 4 },
            Instruction::ReturnPop {
                src: InstructionSrc::Stack(StackLocation::new(0, true)),
                amount: 1,
            },
            Instruction::Sub {
                reg: 0,
                src1: InstructionSrc::Stack(StackLocation::new(0, true)),
                src2: InstructionSrc::Constant { val: Value::Int32(1) },
            },

            Instruction::Call {
                stack_loc_dest: StackLocation::new(1, true),
                stack_loc_call: StackLocation::new(0, false),
            },

            Instruction::Sub {
                reg: 0,
                src1: InstructionSrc::Stack(StackLocation::new(0, true)),
                src2: InstructionSrc::Constant { val: Value::Int32(2) },
            },

            Instruction::Call {
                stack_loc_dest: StackLocation::new(2, true),
                stack_loc_call: StackLocation::new(0, false),
            },
            Instruction::Add {
                reg: 0,
                src1: InstructionSrc::Stack(StackLocation::new(2, true)),
                src2: InstructionSrc::Stack(StackLocation::new(1, true)),
            },

            Instruction::ReturnPop {
                src: InstructionSrc::Register { reg: 0 },
                amount: 3,
            }
        ];
        let main_instructions = vec![
            Instruction::Add {
                reg: 0,
                src1: InstructionSrc::Constant { val: Value::Int32(2) },
                src2: InstructionSrc::Constant { val: Value::Int32(5) },
            },
            Instruction::Sub {
                reg: 1,
                src1: InstructionSrc::Constant { val: Value::Int32(8) },
                src2: InstructionSrc::Constant { val: Value::Int32(7) },
            },
            Instruction::Add {
                reg: 2,
                src1: InstructionSrc::Register { reg: 0 },
                src2: InstructionSrc::Register { reg: 1 },
            },

            Instruction::Halt
        ];

        let mut vm = VM::new(instructions);

        vm.run();

        let instrs = vec![
            FastInstr::new_push_function(17),

            // Define
            FastInstr::new_push_stack(0),
            // First if-branch
            FastInstr::new_load_i32_v2(1, 1),
            FastInstr::new_cmp_jmp(Opcode::CmpLtJmp, 0, 1, 4, 6),
            FastInstr::new_load_stack(0, true, 0),
            FastInstr::new_return_pop(0, 1),
            // If not
            // fib (n - 2)
            FastInstr::new_load_i32_v2(1, 1),
            FastInstr::new_load_stack(2, true, 0),
            FastInstr::new_binary(Opcode::Sub, 0, 2, 1),
            FastInstr::new_call_function(0),
            // fib (n - 1)
            FastInstr::new_load_i32_v2(1, 2),
            FastInstr::new_load_stack(2, true, 0),
            FastInstr::new_binary(Opcode::Sub, 0, 2, 1),
            FastInstr::new_call_function(0),
            // fib(n - 2) + fib(n - 1)
            FastInstr::new_load_stack(1, true, 1),
            FastInstr::new_load_stack(2, true, 2),
            FastInstr::new_binary(Opcode::Add, 0, 2, 1),
            FastInstr::new_return_pop(0, 3),

            // Main program
            FastInstr::new_load_i32_v2(0, 0),
            FastInstr::new_call_function(0),
            FastInstr::new_halt()
        ];

        let constants = vec![Value::Int32(35), Value::Int32(2), Value::Int32(1)];
        let constants_table = ConstantsTable::new(&constants);

        let mut vm2 = VM2::new(&instrs, &constants_table);
        vm2.run();
    } else {
        error_handler.print_errors(src);
    }
}
