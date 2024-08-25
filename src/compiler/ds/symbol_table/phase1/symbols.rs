use std::rc::Rc;

use ahash::AHashMap;

use crate::{
    compiler::{
        ds::ssa_ident::SSAIdent,
        error_handler::{ CompileError, ReportedError },
        parser::token::TokenMetadata,
        traits::SymbolTableActionsPhase1,
        Dissasemble,
    },
    macros::merge_chars_range,
};

use super::{
    util_structs::{ FnType, Symbol, SymbolFn, SymbolMetadata, SymbolVar },
    ProgramSymbolTablePhase1,
};

#[derive(Debug)]
pub struct Symbols {
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

impl SymbolTableActionsPhase1 for Symbols {
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
        _: &'a ProgramSymbolTablePhase1
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
