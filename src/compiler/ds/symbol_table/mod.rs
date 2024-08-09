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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct SSAKey {
    ident: Rc<str>,
    subscript: usize,
}

impl SSAKey {
    pub fn new(ident: Rc<str>, subscript: usize) -> Self {
        Self { ident, subscript }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }

    pub fn get_subscript(&self) -> usize {
        self.subscript
    }
}

impl Dissasemble for SSAKey {
    fn dissasemble(&self) -> String {
        let mut subscript = String::new();
        for char in self.subscript.to_string().chars() {
            subscript += match char {
                '0' => "₀",
                '1' => "₁",
                '2' => "₂",
                '3' => "₃",
                '4' => "₄",
                '5' => "₅",
                '6' => "₆",
                '7' => "₇",
                '8' => "₈",
                '9' => "₉",
                c => panic!("Invalid subscript character: {} (only chars 0-9 is supported)", c),
            };
        }

        format!("{}{}", self.ident, subscript)
    }
}

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
        let d1 = self.allocated_symbol_tables.alloc(
            LocalSymbolTable::new(parent, global_ptr, return_type)
        );
        let d2 = clone_table_ref!(d1);
        d1.insert(
            "sdf".into(),
            Symbol::Variable(
                SymbolVariable::new(ValueType::Bool, true, TokenMetadata::new(1, 1, 1))
            )
        );
        d2.insert(
            "sdf".into(),
            Symbol::Variable(
                SymbolVariable::new(ValueType::Bool, true, TokenMetadata::new(1, 1, 1))
            )
        );
        let idx = self.allocated_symbol_tables.len() - 1;
        todo!()
        // self.allocated_symbol_tables.get_mut(idx).unwrap() as *mut LocalSymbolTable
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
    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAKey {
        let ssa_subscript = self.get_new_ident_subscript(&ident);
        self.symbols.insert(SSAKey::new(ident, ssa_subscript), symbol)
    }

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)> {
        self.symbols.lookup(ident)
    }

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String> {
        self.symbols.lookup_as_fn(ident)
    }

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String> {
        self.symbols.lookup_as_var(ident)
    }

    fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol> {
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
