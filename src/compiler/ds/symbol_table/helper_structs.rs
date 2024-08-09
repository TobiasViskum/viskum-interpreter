use std::rc::Rc;

use ahash::AHashMap;

use crate::compiler::{
    ds::value::ValueType,
    ir::ast::stmt::FunctionArgument,
    parser::token::TokenMetadata,
    traits::SymbolTableAlloc,
};

use super::{ local_symbol_table::LocalSymbolTable, SSAKey };

#[derive(Debug)]
pub enum SymbolState {
    Unchanged,
    MaybeChanged,
}

#[derive(Debug, Clone)]
pub struct SymbolVariable {
    value_type: ValueType,
    is_mutable: bool,
    metadata: TokenMetadata,
}

impl SymbolVariable {
    pub fn new(value_type: ValueType, is_mutable: bool, metadata: TokenMetadata) -> Self {
        Self { value_type, is_mutable, metadata }
    }

    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }
    pub fn get_is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
}

#[derive(Debug)]
pub struct SymbolFunction {
    return_type: ValueType,
    args: Vec<FunctionArgument>,
    metadata: TokenMetadata,
}

impl SymbolFunction {
    pub fn new(
        return_type: ValueType,
        args: Vec<FunctionArgument>,
        metadata: TokenMetadata
    ) -> Self {
        Self { return_type, args, metadata }
    }

    pub fn get_return_type(&self) -> &ValueType {
        &self.return_type
    }

    pub fn get_args(&self) -> &Vec<FunctionArgument> {
        &self.args
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
}

#[derive(Debug)]
pub enum Symbol {
    Variable(SymbolVariable),
    Function(SymbolFunction),
}

impl Symbol {
    pub fn new_variable(value_type: ValueType, is_mutable: bool, metadata: TokenMetadata) -> Self {
        Self::Variable(SymbolVariable::new(value_type, is_mutable, metadata))
    }

    pub fn is_var(&self) -> bool {
        match self {
            Self::Variable(_) => true,
            _ => false,
        }
    }

    pub fn is_fn(&self) -> bool {
        match self {
            Self::Function(_) => true,
            _ => false,
        }
    }

    pub fn new_function(
        return_type: ValueType,
        args: Vec<FunctionArgument>,
        metadata: TokenMetadata
    ) -> Self {
        Self::Function(SymbolFunction::new(return_type, args, metadata))
    }

    pub fn try_value_type_as_var(&self) -> Result<ValueType, String> {
        match self {
            Self::Variable(symbol_var) => Ok(symbol_var.get_value_type().clone()),
            Self::Function(symbol_fn) => {
                Err(
                    format!(
                        "Expected {} arguments but got 0. Use parentheses to call the function: '(_)'",
                        symbol_fn.get_args().len()
                    )
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Symbols {
    symbols: AHashMap<SSAKey, (Symbol, SymbolState)>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            symbols: AHashMap::new(),
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<SSAKey, (Symbol, SymbolState)> {
        self.symbols.iter()
    }

    pub(super) fn count_ident_occurences(&self, ident: &Rc<str>) -> usize {
        self.symbols
            .iter()
            .filter(|&(key, _)| key.get_ident() == *ident)
            .count()
    }
}

impl Symbols {
    pub fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol> {
        match self.symbols.get(ssa_key) {
            Some((symbol, _)) => Some(symbol),
            None => None,
        }
    }

    pub fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String> {
        match self.lookup(ident) {
            Some((_, symbol)) => {
                match symbol {
                    Symbol::Function(symbol_fn) => Ok(symbol_fn),
                    Symbol::Variable(_) =>
                        Err(
                            format!("Undefined function: '{}'. A variable with a similar name exists.", ident)
                        ),
                }
            }
            None => { Err(format!("Undefined function: '{}'", ident)) }
        }
    }

    pub fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String> {
        match self.lookup(ident) {
            Some((_, symbol)) => {
                match symbol {
                    Symbol::Variable(symbol_var) => Ok(symbol_var),
                    Symbol::Function(_) =>
                        Err(
                            format!("Undefined variable: '{}'. A function with a similar name exists.", ident)
                        ),
                }
            }
            None => { Err(format!("Undefined variable: '{}'", ident)) }
        }
    }

    pub fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.0.ident == *ident)
            .max_by_key(|(key, _)| key.subscript)
            .and_then(|(ssa_key, (symbol, _))| Some((ssa_key, symbol)))
    }

    pub fn insert(&mut self, ssa_key: SSAKey, symbol: Symbol) -> SSAKey {
        self.symbols.insert(ssa_key.clone(), (symbol, SymbolState::Unchanged));
        ssa_key
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolTableRef {
    raw_ptr: *mut LocalSymbolTable,
}

impl SymbolTableRef {
    pub(super) fn new(raw_ptr: *mut LocalSymbolTable) -> Self {
        Self { raw_ptr }
    }

    pub fn get(&self) -> &LocalSymbolTable {
        unsafe { &*self.raw_ptr }
    }

    pub fn get_mut(&mut self) -> &mut LocalSymbolTable {
        unsafe { &mut *self.raw_ptr }
    }
}

impl SymbolTableAlloc for SymbolTableRef {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef {
        unsafe { (*self.raw_ptr).alloc_symbol_table(return_type) }
    }
}
