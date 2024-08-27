use crate::compiler::{ ds::value::ValueType, traits::NativeVarTrait };

#[derive(Debug)]
pub enum NativeSymbolVar {}

impl NativeVarTrait for NativeSymbolVar {
    fn get_ident<'a>(&self) -> &'a str {
        panic!("No global vars yet")
    }

    fn get_value_type(&self) -> ValueType {
        panic!("No global vars yet")
    }
}
