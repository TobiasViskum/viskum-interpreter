use std::rc::Rc;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, symbol_table::{ SymbolFn, SymbolVar }, value::ValueType },
    error_handler::CompileError,
};

use super::{
    symbols::Symbols,
    util_structs::{ UserSymbol, UserSymbolFn, UserSymbolVar },
    ProgramSymbolTablePhase1,
};

#[derive(Debug)]
pub struct SymbolTable {
    parent_symbol_table_id: Option<usize>,
    scoped_symbols: Symbols<UserSymbol>,
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

    pub fn lookup_var_ident_by_name<'a>(
        &'a self,
        ssa_ident: &Rc<str>,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<&SSAIdent, String> {
        match self.scoped_symbols.lookup_var_ident_by_name(ssa_ident) {
            Ok(symbol_var) => Ok(symbol_var),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    let parent_table = program_symbol_table.get_table(parent_id);

                    parent_table.lookup_var_ident_by_name(ssa_ident, program_symbol_table)
                } else {
                    Err(format!("Undefined variable: {}", ssa_ident))
                }
            }
        }
    }

    pub fn get_parent_symbol_table_id(&self) -> Option<usize> {
        self.parent_symbol_table_id
    }

    pub fn get_ret_type(&self) -> Option<&ValueType> {
        self.fn_ret_type.as_ref()
    }

    pub(super) fn get_symbols(&self) -> &Symbols<UserSymbol> {
        &self.scoped_symbols
    }

    pub(super) fn get_mut_symbols(&mut self) -> &mut Symbols<UserSymbol> {
        &mut self.scoped_symbols
    }

    pub fn lookup_fn<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<SymbolFn<'a>, String> {
        match self.scoped_symbols.lookup_fn(ssa_ident, program_symbol_table) {
            Ok(symbol_fn) => Ok(SymbolFn::UserSymbolFn(symbol_fn)),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    let parent_table = program_symbol_table.get_table(parent_id);
                    parent_table.lookup_fn(ssa_ident, program_symbol_table)
                } else {
                    Ok(
                        SymbolFn::NativeSymbolFn(
                            program_symbol_table.lookup_global_fn(ssa_ident.borrow_ident())?
                        )
                    )
                }
            }
        }
    }

    pub fn lookup_var<'a>(
        &'a self,
        ssa_ident: &'a SSAIdent,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<SymbolVar<'a>, String> {
        match self.scoped_symbols.lookup_var(ssa_ident, program_symbol_table) {
            Ok(symbol_var) => Ok(SymbolVar::UserSymbolVar(symbol_var)),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    let parent_table = program_symbol_table.get_table(parent_id);

                    parent_table.lookup_var(ssa_ident, program_symbol_table)
                } else {
                    Ok(
                        SymbolVar::NativeSymbolVar(
                            program_symbol_table.lookup_global_var(ssa_ident.borrow_ident())?
                        )
                    )
                }
            }
        }
    }

    pub fn lookup_var_by_name<'a>(
        &'a self,
        name: &Rc<str>,
        program_symbol_table: &'a ProgramSymbolTablePhase1
    ) -> Result<SymbolVar<'a>, String> {
        match self.scoped_symbols.lookup_var_by_name(name, program_symbol_table) {
            Ok(symbol_var) => Ok(SymbolVar::UserSymbolVar(symbol_var)),
            Err(_) => {
                if let Some(parent_id) = self.parent_symbol_table_id {
                    let parent_table = program_symbol_table.get_table(parent_id);

                    parent_table.lookup_var_by_name(name, program_symbol_table)
                } else {
                    Ok(SymbolVar::NativeSymbolVar(program_symbol_table.lookup_global_var(name)?))
                }
            }
        }
    }

    pub fn insert_fn(
        &mut self,
        ssa_ident: SSAIdent,
        symbol_fn: UserSymbolFn
    ) -> Result<(), CompileError> {
        self.scoped_symbols.insert_fn(ssa_ident, symbol_fn)
    }

    pub fn insert_var(&mut self, ssa_ident: SSAIdent, symbol_var: UserSymbolVar) {
        self.scoped_symbols.insert_var(ssa_ident, symbol_var)
    }
}
