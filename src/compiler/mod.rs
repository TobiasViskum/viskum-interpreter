pub(in crate::compiler) mod ds;
pub(in crate::compiler) mod error_handler;
pub(in crate::compiler) mod ir;
pub(in crate::compiler) mod parser;
pub(in crate::compiler) mod traits;

use std::rc::Rc;

use ahash::AHashMap;
use colored::Colorize;
use ds::{ symbol_table::{ GlobalSymbolTable, SSAKey }, value::ValueType, vm_builder::VMBuilder };
use error_handler::ErrorHandler;
use ir::{ ast::{ stmt::FnArg, Ast, AstArena }, icfg::ICFG };
use parser::{ token::TokenMetadata, Parser };
pub use traits::Dissasemble;
use traits::LoadConstants;

use crate::{
    vm::{
        instructions::Instruction,
        optimized_instructions::{ optimize_instructions, OptimizedInstruction },
    },
    U16_MAX,
    U8_MAX,
};

pub fn print_todo(str: &str) {
    eprintln!("{} {}", "TODO:".red().bold(), str)
}

struct SymbolVar {
    mut_keyword_metadata: Option<TokenMetadata>,
    value_type: ValueType,
    ident_metadata: TokenMetadata,
}

struct SymbolFunc {
    ident_metadata: TokenMetadata,
    fn_args: Vec<FnArg>,
    ret_type: ValueType,
}

enum SymbolType {
    Var(SymbolVar),
    Func(SymbolFunc),
}

struct SymbolMetadata {
    symbol_type: SymbolType,
}

struct Symbols {
    symbols: AHashMap<SSAKey, SymbolMetadata>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            symbols: AHashMap::new(),
        }
    }
}

struct SymbolTable {
    parent_symbol_table_id: Option<usize>,
    scoped_symbols: Symbols,
}

impl SymbolTable {
    pub fn new(parent_symbol_table_id: Option<usize>) -> Self {
        Self {
            parent_symbol_table_id,
            scoped_symbols: Symbols::new(),
        }
    }
}

pub struct ProgramSymbolTable {
    symbol_tables: Vec<SymbolTable>,
    ident_occurences: AHashMap<Rc<str>, usize>,
    global_symbols: Symbols,
}

impl ProgramSymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_tables: vec![],
            ident_occurences: AHashMap::new(),
            global_symbols: Symbols::new(),
        }
    }

    pub fn new_symbol_table(&mut self, parent_symbol_table_id: Option<usize>) -> usize {
        self.symbol_tables.push(SymbolTable::new(parent_symbol_table_id));
        self.symbol_tables.len() - 1
    }
}

pub trait SymbolTableActions {}

pub struct Compiler {
    program_symbol_table: ProgramSymbolTable,
    symbol_table: GlobalSymbolTable,
    // register_allocator: RegisterAllocator
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            program_symbol_table: ProgramSymbolTable::new(),
            symbol_table: GlobalSymbolTable::new(),
        }
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

        let mut ast = {
            Parser::new(
                &src_chars,
                error_handler,
                &ast_arena,
                &mut self.program_symbol_table
            ).parse_ast()
        };

        ast.type_check_and_constant_fold(error_handler);

        ast.print();

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
