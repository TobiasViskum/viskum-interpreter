use std::rc::Rc;

use crate::compiler::{ ds::value::ValueType, llvm_builder::Type, traits::NativeFnTrait };

#[derive(Debug, Clone, Copy)]
pub enum NativeSymbolFn {
    PrintFn(PrintFn),
}

impl NativeFnTrait for NativeSymbolFn {
    fn declare_llvm(&self) -> String {
        match self {
            Self::PrintFn(f) => f.declare_llvm(),
        }
    }

    fn get_ident<'a>(&self) -> &'a str {
        match self {
            Self::PrintFn(f) => f.get_ident(),
        }
    }

    fn get_lang_ret_type(&self) -> ValueType {
        match self {
            Self::PrintFn(f) => f.get_lang_ret_type(),
        }
    }

    fn get_llvm_ret_type(&self) -> Type {
        match self {
            Self::PrintFn(f) => f.get_llvm_ret_type(),
        }
    }

    fn get_args_type(&self) -> Vec<ValueType> {
        match self {
            Self::PrintFn(f) => f.get_args_type(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PrintFn;

impl NativeFnTrait for PrintFn {
    fn get_ident<'a>(&self) -> &'a str {
        "print"
    }

    fn declare_llvm(&self) -> String {
        "declare i32 @printf(ptr noundef, ...)\n".to_string()
    }

    fn get_lang_ret_type(&self) -> ValueType {
        ValueType::Void
    }

    fn get_llvm_ret_type(&self) -> Type {
        Type::I32
    }

    fn get_args_type(&self) -> Vec<ValueType> {
        vec![ValueType::String]
    }
}
