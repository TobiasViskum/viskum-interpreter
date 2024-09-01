use crate::compiler::{ ds::value::ValueType, llvm_builder::Type, traits::NativeFnTrait };

macro_rules! native_symbol_fn {
    ($enum_name:ident, $($fn_ident:ident),+) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
        pub enum $enum_name {
            $(
                $fn_ident($fn_ident),
            )+
        }

        impl NativeFnTrait for $enum_name {
            fn declare_llvm(&self) -> String {
                match self {
                    $(
                        Self::$fn_ident(f) => f.declare_llvm(),
                    )+
                }
            }

            fn get_lang_ident<'a>(&self) -> Option<&'a str> {
                match self {
                    $(
                        Self::$fn_ident(f) => f.get_lang_ident(),
                    )+
                }
            }

            fn get_llvm_ident<'a>(&self) -> &'a str {
                match self {
                    $(
                        Self::$fn_ident(f) => f.get_llvm_ident(),
                    )+
                }
            }

            fn get_lang_ret_type(&self) -> Option<ValueType> {
                match self {
                    $(
                        Self::$fn_ident(f) => f.get_lang_ret_type(),
                    )+
                }
            }

            fn get_llvm_ret_type(&self) -> Type {
                match self {
                    $(
                        Self::$fn_ident(f) => f.get_llvm_ret_type(),
                    )+
                }
            }

            fn get_args_type(&self) -> Vec<ValueType> {
                match self {
                    $(
                        Self::$fn_ident(f) => f.get_args_type(),
                    )+
                }
            }

            fn build_call_type(&self) -> String {
                match self {
                    $(
                        Self::$fn_ident(f) => f.build_call_type(),
                    )+
                }
            }
        }
    };
}

native_symbol_fn!(UserNativeSymbolFn, PrintFn);

impl UserNativeSymbolFn {
    pub fn to_native_symbol_fn(&self) -> NativeSymbolFn {
        match self {
            Self::PrintFn(f) => NativeSymbolFn::PrintFn(*f),
        }
    }
}

native_symbol_fn!(NativeSymbolFn, PrintFn, StrCmp);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PrintFn;

impl NativeFnTrait for PrintFn {
    fn get_lang_ident<'a>(&self) -> Option<&'a str> {
        Some("print")
    }

    fn get_llvm_ident<'a>(&self) -> &'a str {
        "printf"
    }

    fn declare_llvm(&self) -> String {
        format!("declare i32 @{}(ptr noundef, ...)\n", self.get_llvm_ident())
    }

    fn build_call_type(&self) -> String {
        "i32 (ptr, ...)".to_string()
    }

    fn get_lang_ret_type(&self) -> Option<ValueType> {
        Some(ValueType::Void)
    }

    fn get_llvm_ret_type(&self) -> Type {
        Type::I32
    }

    fn get_args_type(&self) -> Vec<ValueType> {
        vec![ValueType::String]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StrCmp;

impl NativeFnTrait for StrCmp {
    fn build_call_type(&self) -> String {
        "i32".to_string()
    }

    fn declare_llvm(&self) -> String {
        format!(
            "declare {} @{}(ptr noundef, ptr noundef)",
            self.build_call_type(),
            self.get_llvm_ident()
        )
    }

    fn get_args_type(&self) -> Vec<ValueType> {
        vec![ValueType::String, ValueType::String]
    }

    fn get_lang_ident<'a>(&self) -> Option<&'a str> {
        None
    }

    fn get_lang_ret_type(&self) -> Option<ValueType> {
        None
    }

    fn get_llvm_ident<'a>(&self) -> &'a str {
        "strcmp"
    }

    fn get_llvm_ret_type(&self) -> Type {
        Type::I32
    }
}
