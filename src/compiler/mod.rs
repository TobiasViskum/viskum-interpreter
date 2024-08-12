pub(in crate::compiler) mod ds;
pub(in crate::compiler) mod error_handler;
pub(in crate::compiler) mod ir;
pub(in crate::compiler) mod parser;
pub(in crate::compiler) mod traits;

use colored::Colorize;
use ds::{ symbol_table::GlobalSymbolTable, vm_builder::VMBuilder };
use error_handler::ErrorHandler;
use ir::{ ast::{ Ast, AstArena }, icfg::ICFG };
use parser::Parser;
pub use traits::Dissasemble;
use traits::LoadConstants;

use crate::{
    vm::{
        instructions::Instruction,
        optimized_instructions::{ optimize_instructions, OptimizedInstruction },
    },
    U16_MAX,
};

pub fn print_todo(str: &str) {
    eprintln!("{} {}", "TODO:".red().bold(), str)
}

pub struct Compiler {
    symbol_table: GlobalSymbolTable,
    // register_allocator: RegisterAllocator
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            symbol_table: GlobalSymbolTable::new(),
        }
    }

    pub fn compile_entry(
        &mut self
    ) -> ([i64; U16_MAX], Vec<OptimizedInstruction>, Vec<Instruction>) {
        let file_content = self.get_entry_file_content();
        let src_chars = file_content.chars().collect::<Vec<_>>();
        let mut error_handler = ErrorHandler::new(file_content);

        let icfg = self.make_icfg(&src_chars, &mut error_handler);

        icfg.print();

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
        println!();

        (vm_builder.take_registers(), optimize_instructions(&instructions), instructions)
    }

    pub fn make_icfg(&mut self, src_chars: &Vec<char>, error_handler: &mut ErrorHandler) -> ICFG {
        let mut parser = Parser::new(&src_chars, error_handler);
        let arena = AstArena::new();
        println!("1");
        let mut ast = parser.parse_ast(&mut self.symbol_table, &arena);

        println!("2");
        ast.type_check_and_constant_fold(error_handler);

        println!("3");

        ast.print();

        if error_handler.has_error() {
            Self::log_errors(error_handler);
        }

        Ast::construct_icfg(ast)
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
