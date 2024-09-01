mod native_fn;
mod native_var;

use std::rc::Rc;

pub use native_fn::*;
pub use native_var::*;

use crate::compiler::traits::NativeFnTrait;

#[derive(Debug)]
pub enum NativeSymbol {
    NativeFn(NativeSymbolFn),
    // NativeVar(NativeVar),
    // NativeMod(NativeMod),
    // NativeClass(NativeClass)
}

#[derive(Debug)]
pub struct NativeSymbolsHandler;

impl NativeSymbolsHandler {
    pub fn lookup_fn(&self, ident: &Rc<str>) -> Result<UserNativeSymbolFn, String> {
        let native_fns = [UserNativeSymbolFn::PrintFn(PrintFn)];

        for native_fn in native_fns {
            match native_fn.get_lang_ident() {
                Some(lang_ident) if lang_ident == ident.as_ref() => {
                    return Ok(native_fn);
                }
                _ => {}
            }
        }
        Err(format!("Undefined function '{}'", ident))
    }
    pub fn lookup_var(&self, ident: &Rc<str>) -> Result<NativeSymbolVar, String> {
        match ident.as_ref() {
            _ => Err(format!("Undefined variable '{}'", ident)),
        }
    }
}
