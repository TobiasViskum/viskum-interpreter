pub(in crate::compiler) mod ds;
pub(in crate::compiler) mod error_handler;
pub(in crate::compiler) mod ir;
pub(in crate::compiler) mod parser;
pub(in crate::compiler) mod traits;
pub(in crate::compiler) mod llvm_builder;

use std::rc::Rc;

use colored::Colorize;
use ds::{
    ssa_ident::SSAIdent,
    symbol_table::{ ProgramSymbolTablePhase1, SymbolFn, SymbolVar },
    vm_builder::VMBuilder,
};
use error_handler::{ CompileError, ErrorHandler };
use ir::{ ast::AstArena, icfg::ICFG };
use parser::Parser;
pub use traits::Dissasemble;
use traits::LoadConstants;

use crate::{
    vm::{
        instructions::Instruction,
        optimized_instructions::{ optimize_instructions, OptimizedInstruction },
    },
    U8_MAX,
};

pub fn print_todo(str: &str) {
    eprintln!("{} {}", "TODO:".red().bold(), str)
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_entry(
        &mut self
    ) -> ([i64; U8_MAX], Vec<OptimizedInstruction>, Vec<Instruction>) {
        let file_content = self.get_entry_file_content();
        let src_chars = file_content.chars().collect::<Vec<_>>();
        let mut error_handler = ErrorHandler::new(file_content);

        let icfg = self.make_icfg(&src_chars, &mut error_handler);

        icfg.print();

        let llvm_builder = icfg.build_llvm();
        llvm_builder.output();
        "dot -Tsvg cfg.dot -o cfg.svg && open cfg.svg";

        panic!("Exiting: VM is behind");

        let mut vm_builder = VMBuilder::new();
        icfg.load_constants(&mut vm_builder);

        let mut register_allocator = vm_builder.get_register_allocator();

        let instructions = icfg.generate_instructions(&mut register_allocator);

        instructions
            .iter()
            .enumerate()
            .for_each(|(i, instr)| {
                let idx_string = format!("#{}: ", i);
                println!(
                    "{}{}{}",
                    idx_string,
                    " ".repeat(6 - idx_string.len()),
                    instr.dissasemble()
                )
            });

        (vm_builder.take_registers(), optimize_instructions(&instructions), instructions)
    }

    pub fn make_icfg(&mut self, src_chars: &Vec<char>, error_handler: &mut ErrorHandler) -> ICFG {
        let ast_arena = AstArena::new();

        let mut program_symbol_table = ProgramSymbolTablePhase1::new();

        let mut ast = {
            Parser::new(&src_chars, error_handler).parse_ast(&mut program_symbol_table, &ast_arena)
        };

        ast.type_check_and_constant_fold(&mut program_symbol_table, error_handler);

        ast.ast_print(&mut program_symbol_table);

        if error_handler.has_error() {
            Self::log_errors(error_handler);
        }

        ast.construct_icfg()
    }

    pub fn log_errors(error_handler: &ErrorHandler) -> ! {
        error_handler.print_errors();

        std::process::exit(1);
    }

    fn get_entry_file_content(&self) -> String {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: {} <source file>", args[0]);
            std::process::exit(1);
        }

        match std::fs::read_to_string(&args[1]) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
