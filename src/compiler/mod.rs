pub(in crate::compiler) mod ds;
pub(in crate::compiler) mod error_handler;
pub(in crate::compiler) mod ir;
pub(in crate::compiler) mod parser;
pub(in crate::compiler) mod traits;
pub(in crate::compiler) mod llvm_builder;

use std::rc::Rc;

use ahash::AHashMap;
use colored::Colorize;
use ds::{ ssa_ident::{ self, SSAIdent }, value::ValueType, vm_builder::VMBuilder };
use error_handler::{ CompileError, ErrorHandler, ReportedError };
use ir::{
    ast::{ stmt::{ FnArg, VarAssignStmt }, Ast, AstArena, AST_DISSASEMBLE_INDENTATION },
    icfg::ICFG,
};
use parser::{ token::TokenMetadata, Lexeme, Parser };
pub use traits::Dissasemble;
use traits::{ ExprTrait, LoadConstants };

use crate::{
    macros::merge_chars_range,
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

trait SymbolTableActions {
    fn lookup_var<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolVar, String>;

    fn lookup_var_by_name<'a>(
        &'a self,
        name: Rc<str>,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolVar, String>;

    fn lookup_fn<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolFn, String>;

    fn insert_var(&mut self, ssa_ident: SSAIdent, symbol_var: SymbolVar);

    fn insert_fn(&mut self, ssa_ident: SSAIdent, symbol_fn: SymbolFn) -> Result<(), CompileError>;
}

#[derive(Clone, Debug)]
pub struct SymbolVar {
    mut_keyword_metadata: Option<TokenMetadata>,
    value_type: ValueType,
    ident_metadata: TokenMetadata,
}

impl SymbolVar {
    pub fn new(
        value_type: ValueType,
        ident_metadata: TokenMetadata,
        mut_keyword_metadata: Option<TokenMetadata>
    ) -> Self {
        Self { value_type, ident_metadata, mut_keyword_metadata }
    }

    pub fn get_is_mutable(&self) -> bool {
        self.mut_keyword_metadata.is_some()
    }

    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }

    pub fn get_ident_metadata(&self) -> TokenMetadata {
        self.ident_metadata
    }

    pub fn get_mut_keyword_metadata(&self) -> Option<TokenMetadata> {
        self.mut_keyword_metadata
    }
}
#[derive(Clone, Debug)]
pub struct SymbolFn {
    ident_metadata: TokenMetadata,
    fn_args: Vec<Rc<FnArg>>,
    ret_type: ValueType,
}

impl SymbolFn {
    pub fn new(
        ident_metadata: TokenMetadata,
        fn_args: Vec<Rc<FnArg>>,
        ret_type: ValueType
    ) -> Self {
        Self { ident_metadata, fn_args, ret_type }
    }

    pub fn get_ret_type(&self) -> &ValueType {
        &self.ret_type
    }

    pub fn get_ident_metadata(&self) -> TokenMetadata {
        self.ident_metadata
    }

    pub fn get_fn_args(&self) -> &Vec<Rc<FnArg>> {
        &self.fn_args
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum SymbolType {
    Var,
    Fn,
}
#[derive(Debug)]
enum SymbolMetadata {
    Var(SymbolVar),
    Fn(SymbolFn),
}

#[derive(Debug)]
struct Symbol {
    symbol_type: SymbolType,
    symbol_metadata: SymbolMetadata,
}

impl Symbol {
    pub fn new_fn(symbol_fn: SymbolFn) -> Self {
        Self { symbol_type: SymbolType::Fn, symbol_metadata: SymbolMetadata::Fn(symbol_fn) }
    }

    pub fn new_var(symbol_var: SymbolVar) -> Self {
        Self { symbol_type: SymbolType::Var, symbol_metadata: SymbolMetadata::Var(symbol_var) }
    }

    pub fn get_symbol_type(&self) -> SymbolType {
        self.symbol_type
    }

    pub fn get_symbol_metadata(&self) -> &SymbolMetadata {
        &self.symbol_metadata
    }
}

#[derive(Debug)]
struct Symbols {
    symbols: AHashMap<SSAIdent, Symbol>,
}

impl Dissasemble for Symbols {
    fn dissasemble(&self) -> String {
        let dissasembled_keys = self.symbols
            .keys()
            .enumerate()
            .map(|(i, key)|
                format!("{}{}", key.dissasemble(), if i < self.symbols.len() - 1 {
                    ", "
                } else {
                    ""
                })
            )
            .collect::<String>();

        format!("{{ {} }}", dissasembled_keys)
    }
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            symbols: AHashMap::new(),
        }
    }

    pub fn lookup_main(&mut self) -> Result<(), CompileError> {
        let main_fn_symbol = self.symbols
            .iter()
            .filter_map(|(ssa_ident, symbol)| {
                match (ssa_ident.get_ident().as_ref(), symbol.get_symbol_metadata()) {
                    ("main", SymbolMetadata::Fn(fn_symbol)) => Some((ssa_ident, fn_symbol)),
                    _ => None,
                }
            })
            .next();

        if let Some((ssa_ident, main_fn_symbol)) = main_fn_symbol {
            if main_fn_symbol.get_fn_args().len() == 0 {
                let (mut ssa_ident, main_fn_symbol) = (ssa_ident.clone(), main_fn_symbol.clone());
                self.symbols.remove(&ssa_ident);
                ssa_ident.set_subscript_to_zero();
                self.symbols.insert(ssa_ident, Symbol::new_fn(main_fn_symbol));
                Ok(())
            } else {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "'main' function expected 0 arguments, but got {}",
                                main_fn_symbol.get_fn_args().len()
                            ),
                            merge_chars_range!(
                                main_fn_symbol
                                    .get_fn_args()
                                    .iter()
                                    .map(|arg| arg.get_src_chars_range())
                                    .collect::<Vec<_>>()
                            )
                        )
                    )
                )
            }
        } else {
            Err(
                CompileError::new(
                    ReportedError::new(
                        "'main' function not found".to_string(),
                        TokenMetadata::new(1, 1, 1).into()
                    )
                )
            )
        }

        //  find(|&(ssa_ident, symbol)| {
        //     &ssa_ident.get_ident() == "main" && symbol
        // })
    }
}

impl SymbolTableActions for Symbols {
    fn lookup_fn(
        &self,
        ssa_ident: &SSAIdent,
        _: &ProgramSymbolTablePhase1
    ) -> Result<&SymbolFn, String> {
        if let Some(symbol) = self.symbols.get(ssa_ident) {
            match symbol.get_symbol_metadata() {
                SymbolMetadata::Fn(fn_symbol) => Ok(fn_symbol),
                SymbolMetadata::Var(_) => {
                    Err(
                        format!(
                            "Undefined function '{}'. A variable with the same name exists",
                            ssa_ident.get_ident()
                        )
                    )
                }
            }
        } else {
            Err(format!("Undefined function '{}'", ssa_ident.get_ident()))
        }
    }

    fn lookup_var(
        &self,
        ssa_ident: &SSAIdent,
        _: &ProgramSymbolTablePhase1
    ) -> Result<&SymbolVar, String> {
        if let Some(symbol) = self.symbols.get(ssa_ident) {
            match symbol.get_symbol_metadata() {
                SymbolMetadata::Var(var_symbol) => Ok(var_symbol),
                SymbolMetadata::Fn(_) => {
                    Err(
                        format!(
                            "Undefined variable '{}'. A function with the same name exists",
                            ssa_ident.get_ident()
                        )
                    )
                }
            }
        } else {
            Err(format!("Undefined variable '{}'", ssa_ident.get_ident()))
        }
    }

    fn lookup_var_by_name<'a>(
        &'a self,
        name: Rc<str>,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolVar, String> {
        let found_var = self.symbols
            .iter()
            .filter_map(|(ssa_ident, symbol)| {
                if ssa_ident.get_ident() == name {
                    if let SymbolMetadata::Var(var_symbol) = symbol.get_symbol_metadata() {
                        return Some((ssa_ident, var_symbol));
                    }
                }
                None
            })
            .max_by(|(ssa_ident1, _), (ssa_ident2, _)| {
                ssa_ident1.get_subscript().cmp(&ssa_ident2.get_subscript())
            });

        match found_var {
            Some((_, var_symbol)) => Ok(var_symbol),
            None => Err(format!("Could not find variable '{}' in scope", name)),
        }
    }

    fn insert_fn(&mut self, ssa_ident: SSAIdent, symbol_fn: SymbolFn) -> Result<(), CompileError> {
        let functions_with_same_name = self.symbols
            .iter()
            .filter_map(|(ident, symbol)| {
                match symbol.get_symbol_metadata() {
                    SymbolMetadata::Fn(fn_metadata) => Some((ident, fn_metadata)),
                    _ => None,
                }
            })
            .skip_while(|&(ident, _)| ident.get_ident() != ssa_ident.get_ident());

        let reported_errors = functions_with_same_name
            .map(|(_, symbol_fn)| {
                ReportedError::new(
                    format!("A function named '{}' already exists", ssa_ident.get_ident()),
                    symbol_fn.get_ident_metadata().into()
                )
            })
            .collect::<Vec<_>>();

        if reported_errors.len() > 0 {
            Err(CompileError::new_multiple(reported_errors))
        } else {
            self.symbols.insert(ssa_ident, Symbol::new_fn(symbol_fn));
            Ok(())
        }
    }

    fn insert_var(&mut self, ssa_ident: SSAIdent, symbol_var: SymbolVar) {
        self.symbols.insert(ssa_ident, Symbol::new_var(symbol_var));
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    parent_symbol_table_id: Option<usize>,
    scoped_symbols: Symbols,
    fn_ret_type: Option<ValueType>,
}

impl SymbolTable {
    pub fn new(parent_symbol_table_id: Option<usize>, fn_ret_type: Option<ValueType>) -> Self {
        Self {
            parent_symbol_table_id,
            scoped_symbols: Symbols::new(),
            fn_ret_type,
        }
    }

    pub fn get_parent_symbol_table_id(&self) -> Option<usize> {
        self.parent_symbol_table_id
    }

    pub fn get_ret_type(&self) -> Option<&ValueType> {
        self.fn_ret_type.as_ref()
    }

    fn get_symbols(&self) -> &Symbols {
        &self.scoped_symbols
    }

    fn get_mut_symbols(&mut self) -> &mut Symbols {
        &mut self.scoped_symbols
    }
}

impl SymbolTableActions for SymbolTable {
    fn lookup_fn<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolFn, String> {
        match self.scoped_symbols.lookup_fn(ssa_ident, program_symbol_table) {
            Ok(symbol_fn) => Ok(symbol_fn),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    println!("Parent id: {}", parent_id);
                    let parent_table = program_symbol_table.get_table(parent_id);
                    parent_table.lookup_fn(ssa_ident, program_symbol_table)
                } else {
                    program_symbol_table.lookup_global_fn(ssa_ident)
                }
            }
        }
    }

    fn lookup_var<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolVar, String> {
        match self.scoped_symbols.lookup_var(ssa_ident, program_symbol_table) {
            Ok(symbol_fn) => Ok(symbol_fn),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    let parent_table = program_symbol_table.get_table(parent_id);

                    parent_table.lookup_var(ssa_ident, program_symbol_table)
                } else {
                    program_symbol_table.lookup_global_var(ssa_ident)
                }
            }
        }
    }

    fn lookup_var_by_name<'a>(
        &'a self,
        name: Rc<str>,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&'a SymbolVar, String> {
        match self.scoped_symbols.lookup_var_by_name(Rc::clone(&name), program_symbol_table) {
            Ok(symbol_var) => Ok(symbol_var),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    let parent_table = program_symbol_table.get_table(parent_id);

                    parent_table.lookup_var_by_name(name, program_symbol_table)
                } else {
                    program_symbol_table.lookup_global_var_by_name(name)
                }
            }
        }
    }

    fn insert_fn(&mut self, ssa_ident: SSAIdent, symbol_fn: SymbolFn) -> Result<(), CompileError> {
        self.scoped_symbols.insert_fn(ssa_ident, symbol_fn)
    }

    fn insert_var(&mut self, ssa_ident: SSAIdent, symbol_var: SymbolVar) {
        self.scoped_symbols.insert_var(ssa_ident, symbol_var)
    }
}

pub struct ProgramSymbolTablePhase1 {
    symbol_tables: Vec<SymbolTable>,
    ident_occurences: AHashMap<Rc<str>, usize>,
    ssa_declarations: Vec<(SSAIdent, SymbolType)>,
    global_symbols: Symbols,
    current_symbol_table_id: usize,
}

impl Dissasemble for ProgramSymbolTablePhase1 {
    fn dissasemble(&self) -> String {
        self.dissasemble_table(0, 0)
    }
}

impl ProgramSymbolTablePhase1 {
    fn dissasemble_table(&self, table_id: usize, scope_depth: usize) -> String {
        let table = self.get_table(table_id);
        let child_table_ids = self.get_child_table_ids(table_id);

        let mut string_builder = format!(
            "{}[{}]: {}\n",
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            table_id,
            table.get_symbols().dissasemble()
        );
        for child_table_id in child_table_ids.iter() {
            string_builder += self.dissasemble_table(*child_table_id, scope_depth + 1).as_str();
            string_builder += "\n";
        }

        string_builder
    }

    fn get_child_table_ids(&self, table_id: usize) -> Vec<usize> {
        self.symbol_tables
            .iter()
            .enumerate()
            .filter_map(|(i, symbol_table)| {
                if let Some(parent_symbol_table_id) = symbol_table.get_parent_symbol_table_id() {
                    if parent_symbol_table_id == table_id {
                        return Some(i);
                    }
                }
                None
            })
            .collect::<Vec<_>>()
    }
}

impl ProgramSymbolTablePhase1 {
    pub fn new() -> Self {
        Self {
            symbol_tables: vec![],
            ident_occurences: AHashMap::new(),
            ssa_declarations: Vec::new(),
            global_symbols: Symbols::new(),
            current_symbol_table_id: 0,
        }
    }

    pub fn expect_main_function(&mut self) -> Result<(), CompileError> {
        self.get_mut_table(0).get_mut_symbols().lookup_main()
    }

    pub(super) fn get_table(&self, table_id: usize) -> &SymbolTable {
        self.symbol_tables.get(table_id).expect("Expected symbol table")
    }

    pub(super) fn get_mut_table(&mut self, table_id: usize) -> &mut SymbolTable {
        self.symbol_tables.get_mut(table_id).expect("Expected symbol table")
    }

    pub fn set_current_symbol_table_id(&mut self, new_symbol_table_id: usize) {
        self.current_symbol_table_id = new_symbol_table_id;
    }

    pub fn get_current_symbol_table_id(&self) -> usize {
        self.current_symbol_table_id
    }
    pub fn get_next_symbol_table_id(&self) -> usize {
        self.symbol_tables.len()
    }

    pub fn get_fn_ret_type(&self) -> Option<&ValueType> {
        self.get_table(self.current_symbol_table_id).get_ret_type()
    }

    // fn declare_before_assignment(
    //     &mut self,
    //     var_assign_stmt: &VarAssignStmt
    // ) -> Result<SymbolVar, CompileError> {
    //     let mut ssa_ident = var_assign_stmt.get_target_expr().get_ssa_ident().clone();
    //     ssa_ident.dec_subscript();

    //     let symbol_var = match
    //         self.get_table(self.current_symbol_table_id).lookup_var(&ssa_ident, self)
    //     {
    //         Ok(symbol_var) => symbol_var.clone(),
    //         Err(msg) => {
    //             return Err(
    //                 CompileError::new(
    //                     ReportedError::new(
    //                         msg,
    //                         var_assign_stmt.get_target_expr().collect_metadata()
    //                     )
    //                 )
    //             );
    //         }
    //     };

    //     let ssa_ident = var_assign_stmt.get_target_expr().get_ssa_ident();
    //     let new_symbol_var = SymbolVar::new(
    //         symbol_var.get_value_type().clone(),
    //         var_assign_stmt.get_target_expr().get_metadata(),
    //         symbol_var.get_mut_keyword_metadata()
    //     );
    //     self.insert_var(ssa_ident.clone(), new_symbol_var.clone());

    //     Ok(new_symbol_var)
    // }

    fn get_prev_symbol_var(
        &self,
        var_assign_stmt: &VarAssignStmt
    ) -> Result<&SymbolVar, CompileError> {
        let prev_symbol_var = self
            .get_table(self.current_symbol_table_id)
            .lookup_var_by_name(
                var_assign_stmt.get_target_expr().get_ssa_ident().get_ident(),
                self
            );

        let prev_symbol_var = match prev_symbol_var {
            Ok(v) => v,
            Err(msg) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            msg,
                            var_assign_stmt.get_target_expr().collect_metadata()
                        )
                    )
                );
            }
        };

        Ok(prev_symbol_var)
    }

    pub fn assign_var(
        &mut self,
        var_assign_stmt: &mut VarAssignStmt
    ) -> Result<ValueType, CompileError> {
        let prev_symbol_var = self.get_prev_symbol_var(var_assign_stmt)?.clone();

        let ssa_ident = var_assign_stmt.get_target_expr().get_ssa_ident().clone();

        self.insert_var(
            ssa_ident.clone(),
            SymbolVar::new(
                prev_symbol_var.get_value_type().clone(),
                var_assign_stmt.get_target_expr().get_metadata(),
                prev_symbol_var.get_mut_keyword_metadata()
            )
        );

        var_assign_stmt.get_mut_target_expr().type_check(self)?;
        let value_type = var_assign_stmt.get_mut_value_expr().type_check(self)?;

        if prev_symbol_var.get_is_mutable() {
            if prev_symbol_var.get_value_type().is(&value_type) {
                Ok(value_type)
            } else {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Variable '{}' is of type '{}' but it is assigned to type '{}'",
                                ssa_ident.get_ident(),
                                prev_symbol_var.get_value_type().dissasemble(),
                                value_type.dissasemble()
                            ),
                            {
                                let mut src_chars_range = var_assign_stmt
                                    .get_target_expr()
                                    .collect_metadata();
                                src_chars_range.merge(
                                    &var_assign_stmt.get_value_expr().collect_metadata()
                                );
                                src_chars_range
                            }
                        )
                    )
                )
            }
        } else {
            Err(
                CompileError::new_multiple(
                    vec![
                        ReportedError::new(
                            format!(
                                "Cannot assign a value to immutable variable '{}'",
                                ssa_ident.get_ident()
                            ),
                            {
                                let mut src_chars_range = var_assign_stmt
                                    .get_target_expr()
                                    .collect_metadata();
                                src_chars_range.merge(
                                    &var_assign_stmt.get_value_expr().collect_metadata()
                                );
                                src_chars_range
                            }
                        ),
                        ReportedError::new(
                            format!("Consider changing this to `mut {}`", ssa_ident.get_ident()),
                            prev_symbol_var.get_ident_metadata().into()
                        )
                    ]
                )
            )
        }
    }

    pub fn insert_var(&mut self, ssa_ident: SSAIdent, symbol_var: SymbolVar) {
        self.get_mut_table(self.current_symbol_table_id).insert_var(ssa_ident, symbol_var);
    }

    pub fn insert_fn(
        &mut self,
        ssa_ident: SSAIdent,
        symbol_fn: SymbolFn
    ) -> Result<(), CompileError> {
        self.get_mut_table(self.current_symbol_table_id).insert_fn(ssa_ident, symbol_fn)
    }

    pub fn lookup_var<'a>(&'a self, ssa_ident: &'a SSAIdent) -> Result<&'a SymbolVar, String> {
        let table = self.get_table(self.current_symbol_table_id);
        table.lookup_var(ssa_ident, self)
    }

    pub fn lookup_fn<'a>(&'a self, ssa_ident: &'a SSAIdent) -> Result<&'a SymbolFn, String> {
        let table = self.get_table(self.current_symbol_table_id);
        table.lookup_fn(ssa_ident, self)
    }

    pub(super) fn lookup_global_var<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent
    ) -> Result<&'a SymbolVar, String> {
        self.global_symbols.lookup_var(ssa_ident, self)
    }

    pub(super) fn lookup_global_var_by_name<'a>(
        &'a self,
        name: Rc<str>
    ) -> Result<&'a SymbolVar, String> {
        self.global_symbols.lookup_var_by_name(name, self)
    }

    pub(super) fn lookup_global_fn<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent
    ) -> Result<&'a SymbolFn, String> {
        self.global_symbols.lookup_fn(ssa_ident, self)
    }

    pub fn search_fn_ssa_ident(&self, lexeme: &Lexeme) -> SSAIdent {
        for (ssa_ident, symbol_type) in self.ssa_declarations.iter().rev() {
            if *symbol_type == SymbolType::Fn && ssa_ident.get_ident() == *lexeme.borrow_ident() {
                return ssa_ident.clone();
            }
        }

        SSAIdent::new(lexeme.get_ident(), 1)
    }

    pub fn search_var_ssa_ident(&self, lexeme: &Lexeme) -> SSAIdent {
        for (ssa_ident, symbol_type) in self.ssa_declarations.iter().rev() {
            if *symbol_type == SymbolType::Var && ssa_ident.get_ident() == *lexeme.borrow_ident() {
                return ssa_ident.clone();
            }
        }

        SSAIdent::new(lexeme.get_ident(), 1)
    }

    pub fn declare_fn_ssa_ident(&mut self, lexeme: &Lexeme) -> SSAIdent {
        let new_subscript = self.ident_occurences
            .get(lexeme.borrow_ident())
            .cloned()
            .map(|subscript| subscript + 1)
            .unwrap_or(1);
        self.ident_occurences.insert(lexeme.get_ident(), new_subscript);
        let ssa_ident = SSAIdent::new(lexeme.get_ident(), new_subscript);
        self.ssa_declarations.push((ssa_ident.clone(), SymbolType::Fn));
        ssa_ident
    }

    pub fn declare_var_ssa_ident(&mut self, lexeme: &Lexeme) -> SSAIdent {
        let new_subscript = self.ident_occurences
            .get(lexeme.borrow_ident())
            .cloned()
            .map(|subscript| subscript + 1)
            .unwrap_or(1);
        self.ident_occurences.insert(lexeme.get_ident(), new_subscript);
        let ssa_ident = SSAIdent::new(lexeme.get_ident(), new_subscript);
        self.ssa_declarations.push((ssa_ident.clone(), SymbolType::Var));
        ssa_ident
    }

    pub fn make_stage_2(self) {}

    pub fn new_symbol_table(
        &mut self,
        parent_symbol_table_id: Option<usize>,
        fn_return_type: Option<ValueType>
    ) -> usize {
        self.symbol_tables.push(SymbolTable::new(parent_symbol_table_id, fn_return_type));
        let new_id = self.symbol_tables.len() - 1;
        self.current_symbol_table_id = new_id;
        new_id
    }
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
