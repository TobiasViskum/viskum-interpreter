use std::rc::Rc;

use ahash::AHashMap;
use symbols::Symbols;
use util_structs::SymbolType;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::{ CompileError, ReportedError },
    ir::ast::{ stmt::VarAssignStmt, AST_DISSASEMBLE_INDENTATION },
    parser::Lexeme,
    traits::{ ExprTrait, SymbolTableActionsPhase1 },
    Dissasemble,
};

use super::{ FnType, SymbolFn, SymbolTable, SymbolVar };

pub mod symbol_table;
pub mod symbols;
pub mod util_structs;

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
// Helper dissasemble methods
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
