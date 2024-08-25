mod phase1;
mod phase2;

pub(in crate::compiler) use phase1::{
    ProgramSymbolTablePhase1,
    symbol_table::SymbolTable,
    util_structs::{ SymbolFn, SymbolVar, FnType },
};
pub(in crate::compiler) mod native_symbols;
