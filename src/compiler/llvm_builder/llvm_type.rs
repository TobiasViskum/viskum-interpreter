use super::BuildLLVM;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Type {
    I32,
    I1,
    Ptr,
    Array((Box<Self>, usize)),
    Void,
}

impl Type {
    pub fn is(&self, other: &Self) -> bool {
        self == other
    }

    pub fn get_byte_size(&self) -> usize {
        match self {
            Self::I32 => 4,
            Self::I1 => 1,
            Self::Ptr => 8,
            Self::Array((llvm_type, count)) => llvm_type.get_byte_size(),
            Self::Void => panic!("Type void doesn't have a size"),
        }
    }

    pub fn build_as_arg(&self) -> String {
        format!("{} noundef", self.build())
    }

    pub fn to_fmt_str<'a>(&self) -> &'a str {
        match self {
            Self::I32 | Self::I1 => "%d",
            Self::Array(_) => "%p",
            Self::Ptr => "%s",
            Self::Void => panic!("Void cannot be formatted"),
        }
    }
}

impl BuildLLVM for Type {
    fn build(&self) -> String {
        match self {
            Self::I32 => "i32".to_string(),
            Self::I1 => "i1".to_string(),
            Self::Ptr => "ptr".to_string(),
            Self::Void => "void".to_string(),
            Self::Array((llvm_type, item_count)) => {
                format!("[{} x {}]", item_count, llvm_type.build())
            }
        }
    }
}

pub enum RawOperand {
    TypeI1(TypeI1),
    TypeI32(TypeI32),
    ConstStringPointer(ConstStringPointer),
}

impl BuildLLVM for RawOperand {
    fn build(&self) -> String {
        match self {
            Self::TypeI1(v) => v.build(),
            Self::TypeI32(v) => v.build(),
            Self::ConstStringPointer(v) => v.build(),
        }
    }
}

pub struct TypeI1 {
    val: i8,
}
impl TypeI1 {
    pub fn new(val: bool) -> Self {
        Self {
            val: val as i8,
        }
    }
}
impl BuildLLVM for TypeI1 {
    fn build(&self) -> String {
        self.val.to_string()
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
