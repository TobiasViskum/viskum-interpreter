use crate::BuildLLVM;

pub enum Type {
    I32,
}

impl BuildLLVM for Type {
    fn build(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
        }
    }
}

pub enum RawOperand {
    TypeI32(TypeI32),
}

impl BuildLLVM for RawOperand {
    fn build(&self) -> String {
        match self {
            Self::TypeI32(v) => v.build(),
        }
    }
}

pub trait LLVMType {
    fn build_type() -> String;
}

pub struct TypeI32 {
    val: i32,
}

impl TypeI32 {
    pub fn new(val: i32) -> Self {
        Self { val }
    }
}

impl BuildLLVM for TypeI32 {
    fn build(&self) -> String {
        self.val.to_string()
    }
}

impl LLVMType for TypeI32 {
    fn build_type() -> String {
        "i32".to_string()
    }
}
