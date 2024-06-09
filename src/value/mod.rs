use std::{ boxed, fmt::{ self, Display }, rc::Rc };

use crate::{
    create_tokens_and_parse_rules,
    def_binary_val_method,
    define_unary_val_method,
    operations::{ BinaryOp, ComparisonOp, UnaryOp },
    vm::{ instructions::Instruction, Instructions },
    Dissasemble,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int32,
    Bool,
    Void,
    String,
    Function,
    Deref(Box<Self>),
    Ref(Box<Self>),
    MutableRef(Box<Self>),
}

impl Dissasemble for ValueType {
    fn dissasemble(&self) -> String {
        match self {
            Self::Int32 => "i32".to_string(),
            Self::Bool => "bool".to_string(),
            Self::Void => "()".to_string(),
            Self::String => "string".to_string(),
            Self::Function => "<fn>".to_string(),
            Self::Deref(boxed) => format!("*{}", boxed.dissasemble()),
            Self::Ref(boxed) => format!("&{}", boxed.dissasemble()),
            Self::MutableRef(boxed) => format!("&mut {}", boxed.dissasemble()),
        }
    }
}

impl ValueType {
    pub fn is(&self, other: &ValueType) -> bool {
        matches!(
            (self, other),
            |(Self::Int32, Self::Int32)|
                (Self::Bool, Self::Bool) | (Self::String, Self::String) | (Self::Void, Self::Void)
        )
    }

    pub fn to_type_string(&self) -> String {
        match self {
            ValueType::Int32 => "i32".to_string(),
            ValueType::Bool => "bool".to_string(),
            ValueType::Void => "void".to_string(),
            ValueType::String => "string".to_string(),
            ValueType::Function => "<fn>".to_string(),
            _ => "TO_TYPE_STRING".to_string(),
        }
    }

    pub fn type_check_binary(&self, other: &ValueType, op: BinaryOp) -> Result<ValueType, String> {
        match op {
            BinaryOp::Add => self.try_add(other),
            BinaryOp::Mul => self.try_mul(other),
            BinaryOp::Div => self.try_div(other),
            BinaryOp::Sub => self.try_sub(other),
            | BinaryOp::ComparisonOp(ComparisonOp::Equal)
            | BinaryOp::ComparisonOp(ComparisonOp::NotEqual)
            | BinaryOp::ComparisonOp(ComparisonOp::Greater)
            | BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual)
            | BinaryOp::ComparisonOp(ComparisonOp::Less)
            | BinaryOp::ComparisonOp(ComparisonOp::LessEqual) => self.try_cmp(other, op),
        }
    }

    pub fn type_check_unary(&self, op: UnaryOp) -> Result<ValueType, String> {
        match op {
            UnaryOp::Neg => self.try_neg(),
            UnaryOp::Truthy => Ok(self.clone()),
            UnaryOp::Ref => Ok(Self::Ref(Box::new(self.clone()))),
            UnaryOp::Deref => Ok(Self::Deref(Box::new(self.clone()))),
            UnaryOp::MutRef => Ok(Self::MutableRef(Box::new(self.clone()))),
        }
    }

    pub fn try_cmp(&self, other: &ValueType, op: BinaryOp) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Bool),
            (ValueType::Bool, ValueType::Bool) => Ok(ValueType::Bool),
            _ => {
                let op_lexeme = match op {
                    BinaryOp::ComparisonOp(ComparisonOp::Equal) => "==",
                    BinaryOp::ComparisonOp(ComparisonOp::NotEqual) => "!=",
                    BinaryOp::ComparisonOp(ComparisonOp::Greater) => ">",
                    BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual) => ">=",
                    BinaryOp::ComparisonOp(ComparisonOp::Less) => "<",
                    BinaryOp::ComparisonOp(ComparisonOp::LessEqual) => "<=",
                    _ => "ERROR",
                };

                Err(
                    format!(
                        "Comparison '{}' is not defined for {} and {}",
                        op_lexeme,
                        self.to_type_string(),
                        other.to_type_string()
                    )
                )
            }
        }
    }

    pub fn get_minified(value_type: &Self) -> Self {
        let mut minified_type = value_type.clone();
        match minified_type {
            ValueType::Deref(inner) => {
                match *inner {
                    ValueType::Ref(contained) => {
                        minified_type = Self::get_minified(&contained);
                        minified_type
                    }
                    ValueType::Deref(_) => {
                        minified_type = Self::get_minified(&inner);
                        minified_type
                    }
                    t => t,
                }
            }
            _ => minified_type,
        }
    }

    pub fn try_add(&self, other: &ValueType) -> Result<ValueType, String> {
        let minified_self = Self::get_minified(self);
        let minified_other = Self::get_minified(other);

        match (minified_self, minified_other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            (ValueType::String, ValueType::String) => Ok(ValueType::String),
            _ =>
                Err(
                    format!(
                        "Addition is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_mul(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            _ =>
                Err(
                    format!(
                        "Multiplication is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_div(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            _ =>
                Err(
                    format!(
                        "Division is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_sub(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            _ =>
                Err(
                    format!(
                        "Subtraction is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_neg(&self) -> Result<ValueType, String> {
        if self.is(&ValueType::Int32) {
            Ok(self.clone())
        } else {
            Err(format!("Negation is not defined for {}", self.to_type_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub instructions: Instructions,
    pub args_count: usize,
}

impl Function {
    pub fn new(args_count: usize) -> Self {
        Self {
            instructions: Instructions::new(),
            args_count,
        }
    }

    pub fn from(args_count: usize, instructions: Vec<Instruction>) -> Self {
        Self {
            instructions: Instructions::from(instructions),
            args_count,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int32(i32),
    Bool(bool),
    String(Rc<str>),
    Ref(Box<Self>),
    MutableRef(Box<Self>),
    Deref(Box<Self>),
    Mutable(Box<Self>),
    Void,
}

impl Dissasemble for Value {
    fn dissasemble(&self) -> String {
        match self {
            Self::Int32(i32) => i32.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::String(str) => str.to_string(),
            Self::Void => "()".to_string(),
            Self::Deref(contained) => format!("*{}", contained.dissasemble()),
            Self::Ref(contained) => format!("&{}", contained.dissasemble()),
            Self::MutableRef(contained) => format!("&mut {}", contained.dissasemble()),
            Self::Mutable(contained) => format!("mut {}", contained.dissasemble()),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Bool(false) // Remove this (only used in outdated VM)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int32(int32) => write!(f, "{}", int32),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::String(str) => write!(f, "{}", str),
            Value::Void => write!(f, "()"),
            _ => write!(f, "sdfdsfsdfasdfas"),
        }
    }
}

impl Value {
    // pub fn to_u8(&self) -> u8 {
    //     match self {
    //         Value::Bool(_) => 0,
    //         Value::Int32(_) => 1,
    //         Value::String(_) => 2,
    //         Value::Function(_) => 3,
    //         Value::Void => 4,
    //         Value::Empty => 5,
    //     }
    // }

    pub fn to_string(&self) -> String {
        match self {
            Self::Int32(i32) => i32.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::String(str) => str.to_string(),
            Self::Void => String::new(),
            _ => "not impl yet".to_string(),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        match self {
            Value::Int32(_) => ValueType::Int32,
            Value::Bool(_) => ValueType::Bool,
            Value::String(_) => ValueType::String,
            Value::Void => ValueType::Void,
            _ => panic!(),
        }
    }

    def_binary_val_method!(cmp_eq, |lhs == rhs| -> Bool, {
        (Int32, Int32),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_ne, |lhs != rhs| -> Bool, {
        (Int32, Int32),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_gt, |lhs > rhs| -> Bool, {
        (Int32, Int32),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_ge, |lhs >= rhs| -> Bool, {
        (Int32, Int32),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_lt, |lhs < rhs| -> Bool, {
        (Int32, Int32),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_le, |lhs <= rhs| -> Bool, {
        (Int32, Int32),
        (Bool, Bool),
    });

    def_binary_val_method!(add, |lhs + rhs| -> Self, {
        (Int32, Int32),
        (String, String): format!("{}{}", lhs, rhs).into(),
    });

    def_binary_val_method!(mul, |lhs * rhs| -> Self, {
        (Int32, Int32),
    });

    def_binary_val_method!(div, |lhs / rhs| -> Self, {
        (Int32, Int32),
    });

    def_binary_val_method!(sub, |lhs - rhs| -> Self, {
        (Int32, Int32),
    });

    define_unary_val_method!(neg, |-rhs| -> Self, {
        (Int32)
    });

    define_unary_val_method!(not, |!rhs| -> Bool, {
        (Int32): *rhs == 0,
        (Bool),
        (String): rhs.len() == 0
    });
}
