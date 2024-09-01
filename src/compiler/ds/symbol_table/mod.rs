mod phase1;
mod phase2;
mod native_symbol;

pub(in crate::compiler) use phase1::{
    ProgramSymbolTablePhase1,
    symbol_table::SymbolTable,
    util_structs::{ UserSymbol, UserSymbolFn, UserSymbolVar },
};
pub(in crate::compiler) use native_symbol::*;

use crate::compiler::traits::{ NativeFnTrait, NativeVarTrait };

use super::value::ValueType;

pub enum Symbol {
    UserSymbol(UserSymbol),
    NativeSymbol(NativeSymbol),
}

pub enum SymbolVar<'a> {
    UserSymbolVar(&'a UserSymbolVar),
    NativeSymbolVar(NativeSymbolVar),
}

impl<'a> SymbolVar<'a> {
    pub fn get_value_type(&self) -> ValueType {
        match self {
            Self::UserSymbolVar(v) => v.get_value_type().clone(),
            Self::NativeSymbolVar(v) => v.get_value_type(),
        }
    }
}

pub enum SymbolFn<'a> {
    UserSymbolFn(&'a UserSymbolFn),
    NativeSymbolFn(UserNativeSymbolFn),
}

impl<'a> SymbolFn<'a> {
    pub fn get_ret_type(&self) -> ValueType {
        match self {
            Self::UserSymbolFn(f) => f.get_ret_type().clone(),
            Self::NativeSymbolFn(f) => f.get_lang_ret_type().unwrap(),
        }
    }

    pub fn get_args_count(&self) -> usize {
        match self {
            Self::UserSymbolFn(f) => f.get_fn_args().len(),
            Self::NativeSymbolFn(f) => f.get_args_type().len(),
        }
    }
}
