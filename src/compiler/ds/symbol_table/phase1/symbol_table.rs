use std::rc::Rc;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::CompileError,
    traits::SymbolTableActionsPhase1,
};

use super::{
    symbols::Symbols,
    util_structs::{ FnType, SymbolFn, SymbolVar },
    ProgramSymbolTablePhase1,
};

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

    pub(super) fn get_symbols(&self) -> &Symbols {
        &self.scoped_symbols
    }

    pub(super) fn get_mut_symbols(&mut self) -> &mut Symbols {
        &mut self.scoped_symbols
    }
}

impl SymbolTableActionsPhase1 for SymbolTable {
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
