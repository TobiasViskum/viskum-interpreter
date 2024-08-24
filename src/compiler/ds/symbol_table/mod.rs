mod local_symbol_table;
mod helper_structs;

use ahash::AHashMap;
pub use helper_structs::*;
use typed_arena::Arena;

use std::{ fmt::Debug, rc::Rc };

use local_symbol_table::LocalSymbolTable;

use crate::compiler::{
    parser::token::TokenMetadata,
    traits::{ Dissasemble, SymbolTableActions, SymbolTableAlloc },
};

use super::value::ValueType;

pub struct GlobalSymbolTable {
    allocated_symbol_tables: Arena<LocalSymbolTable>,
    ident_occurences: AHashMap<Rc<str>, usize>,
    symbols: Symbols,
}

impl Debug for GlobalSymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ident_occurences: {:?}\nsymbols:{:?}", self.ident_occurences, self.symbols)
    }
}

macro_rules! clone_table_ref {
    ($name:ident) => {
        unsafe { &mut *($name as *mut LocalSymbolTable) }
    };
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self {
            allocated_symbol_tables: Arena::new(),
            ident_occurences: AHashMap::new(),
            symbols: Symbols::new(),
        }
    }

    fn alloc_empty(
        &mut self,
        parent: Option<*mut LocalSymbolTable>,
        return_type: Option<ValueType>
    ) -> *mut LocalSymbolTable {
        let global_ptr = self as *mut GlobalSymbolTable;

        self.allocated_symbol_tables.alloc(
            LocalSymbolTable::new(parent, global_ptr, return_type)
        ) as *mut LocalSymbolTable
    }

    fn get_new_ident_subscript(&mut self, ident: &Rc<str>) -> usize {
        let prev_highest_subscript = match self.ident_occurences.get(ident) {
            Some(subscript) => *subscript,
            None => 0,
        };

        let new_subscript = prev_highest_subscript + 1;
        self.ident_occurences.insert(Rc::clone(ident), new_subscript);

        new_subscript
    }
}

impl SymbolTableAlloc for GlobalSymbolTable {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef {
        let allocated_symbol_table = self.alloc_empty(None, return_type);
        SymbolTableRef::new(allocated_symbol_table)
    }
}

impl SymbolTableActions for GlobalSymbolTable {
    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAIdent {
        let ssa_subscript = self.get_new_ident_subscript(&ident);
        self.symbols.insert(SSAIdent::new(ident, ssa_subscript), symbol)
    }

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAIdent, &Symbol)> {
        self.symbols.lookup(ident)
    }

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String> {
        self.symbols.lookup_as_fn(ident)
    }

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String> {
        self.symbols.lookup_as_var(ident)
    }

    fn lookup_with_key(&self, ssa_key: &SSAIdent) -> Option<&Symbol> {
        self.symbols.lookup_with_key(ssa_key)
    }
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

    pub fn get_mut_symbol_table(&mut self) -> &mut GlobalSymbolTable {
        &mut self.symbol_table
    }
}
