use colored::Colorize;
use ds::symbol_table::GlobalSymbolTable;
use error_handler::ErrorHandler;
use ir::ast::{ Ast, AstArena };
use parser::Parser;

pub(in crate::compiler) mod parser;
pub(in crate::compiler) mod ds;
pub(in crate::compiler) mod ir;
pub(in crate::compiler) mod error_handler;

pub trait Dissasemble {
    fn dissasemble(&self) -> String;
}

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

    pub fn compile_entry(&mut self) -> Option<()> {
        let file_content = self.get_entry_file_content();
        let src_chars = file_content.chars().collect::<Vec<_>>();
        let mut error_handler = ErrorHandler::new(file_content);

        let icfg = {
            let mut parser = Parser::new(&src_chars, &mut error_handler);
            let arena = AstArena::new();
            let mut ast = parser.parse_ast(&mut self.symbol_table, &arena);
            ast.type_check_and_constant_fold(&mut error_handler);

            ast.print();

            Ast::construct_icfg(ast)
        };

        Some(())
    }

    pub fn log_errors(&self) -> ! {
        // Log errors
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
