use super::BuildLLVM;

#[derive(PartialEq)]
pub enum Type {
    I32,
    I1,
    Ptr,
    Void,
}

impl Type {
    pub fn is(&self, other: &Self) -> bool {
        self == other
    }
}

impl BuildLLVM for Type {
    fn build(&self) -> String {
        (
            match self {
                Self::I32 => "i32",
                Self::I1 => "i1",
                Self::Ptr => "ptr",
                Self::Void => "void",
            }
        ).to_string()
    }
}

pub enum RawOperand {
    TypeI32(TypeI32),
    ConstStringPointer(ConstStringPointer),
}

impl BuildLLVM for RawOperand {
    fn build(&self) -> String {
        match self {
            Self::TypeI32(v) => v.build(),
            Self::ConstStringPointer(v) => v.build(),
        }
    }
}

pub struct ConstStringPointer {
    idx: usize,
}
impl ConstStringPointer {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}
impl BuildLLVM for ConstStringPointer {
    fn build(&self) -> String {
        format!("@.str.{}", self.idx)
    }
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
