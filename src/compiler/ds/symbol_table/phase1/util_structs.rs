use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::native_symbols::NativeFn, value::ValueType },
    ir::ast::stmt::FnArg,
    parser::token::TokenMetadata,
};

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
    fn_args: Rc<[FnArg]>,
    ret_type: ValueType,
}

impl SymbolFn {
    pub fn new(ident_metadata: TokenMetadata, fn_args: Rc<[FnArg]>, ret_type: ValueType) -> Self {
        Self { ident_metadata, fn_args, ret_type }
    }

    pub fn get_ret_type(&self) -> &ValueType {
        &self.ret_type
    }

    pub fn get_ident_metadata(&self) -> TokenMetadata {
        self.ident_metadata
    }

    pub fn get_fn_args(&self) -> &Rc<[FnArg]> {
        &self.fn_args
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SymbolType {
    Var,
    Fn,
}

#[derive(Debug)]
pub enum FnType {
    SymbolFn(SymbolFn),
    NativeFn(NativeFn),
}

#[derive(Debug)]
pub enum SymbolMetadata {
    Var(SymbolVar),
    Fn(SymbolFn),
}

#[derive(Debug)]
pub struct Symbol {
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
