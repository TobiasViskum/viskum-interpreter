pub(in crate::compiler) mod ops;

use std::rc::Rc;

use ops::{ BinaryOp, ComparisonOp, UnaryOp };

use crate::{
    compiler::Dissasemble,
    macros::{
        def_binary_try_method,
        def_binary_val_method,
        def_unary_try_method,
        def_unary_val_method,
    },
};

/*
It should expand into:

pub enum Value {
    Int(isize),
    Bool(bool),
    String(Rc<str>),
    Deref(Box<Self>),
    Ref(Box<Self>),
    MutableRef(Box<Self>),
    Mutable(Box<Self>)
};

pub enum ValueType {
    Int,
    Bool,
    String,
    Deref(Box<Self>),
    Ref(Box<Self>),
    MutableRef(Box<Self>),
    Mutable(Box<Self>)
};
*/

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int32,
    Bool,
    Void,
    String,
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
            Self::Deref(boxed) => format!("*{}", boxed.dissasemble()),
            Self::Ref(boxed) => format!("&{}", boxed.dissasemble()),
            Self::MutableRef(boxed) => format!("&mut {}", boxed.dissasemble()),
        }
    }
}

impl ValueType {
    pub fn is(&self, other: &ValueType) -> bool {
        self == other
    }

    pub fn to_type_string(&self) -> String {
        match self {
            ValueType::Int32 => "i32".to_string(),
            ValueType::Bool => "bool".to_string(),
            ValueType::Void => "void".to_string(),
            ValueType::String => "string".to_string(),
            _ => "TO_TYPE_STRING".to_string(),
        }
    }

    pub fn try_unary(&self, op: UnaryOp) -> Result<ValueType, String> {
        match op {
            UnaryOp::Neg => self.try_neg(),
            UnaryOp::Not => self.try_not(),
            UnaryOp::Ref => Ok(Self::Ref(Box::new(self.clone()))),
            UnaryOp::Deref => Ok(Self::Deref(Box::new(self.clone()))),
            UnaryOp::MutRef => Ok(Self::MutableRef(Box::new(self.clone()))),
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

    pub fn try_binary(&self, other: &ValueType, binary_op: BinaryOp) -> Result<Self, String> {
        match binary_op {
            BinaryOp::Add => self.try_add(other),
            BinaryOp::Sub => self.try_sub(other),
            BinaryOp::Mul => self.try_mul(other),
            BinaryOp::Div => self.try_div(other),
            BinaryOp::ComparisonOp(comparison_op) => {
                match comparison_op {
                    ComparisonOp::Eq => self.try_cmp_eq(other),
                    ComparisonOp::Ne => self.try_cmp_ne(other),
                    ComparisonOp::Gt => self.try_cmp_gt(other),
                    ComparisonOp::Ge => self.try_cmp_ge(other),
                    ComparisonOp::Lt => self.try_cmp_lt(other),
                    ComparisonOp::Le => self.try_cmp_le(other),
                }
            }
        }
    }

    def_binary_try_method!(try_cmp_eq, ComparisonOp::Eq => Bool, {
        (Int32, Int32),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_ne, ComparisonOp::Ne => Bool, {
        (Int32, Int32),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_gt, ComparisonOp::Gt => Bool, {
        (Int32, Int32),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_ge, ComparisonOp::Ge => Bool, {
        (Int32, Int32),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_lt, ComparisonOp::Lt => Bool, {
        (Int32, Int32),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_le, ComparisonOp::Le => Bool, {
        (Int32, Int32),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_add, BinaryOp::Add, {
        (Int32, Int32),
        (String, String)
    });

    def_binary_try_method!(try_sub, BinaryOp::Sub, {
        (Int32, Int32),
    });

    def_binary_try_method!(try_mul, BinaryOp::Mul, {
        (Int32, Int32),
    });

    def_binary_try_method!(try_div, BinaryOp::Div, {
        (Int32, Int32),
    });

    def_unary_try_method!(try_neg, UnaryOp::Neg, { Int32 });

    def_unary_try_method!(try_not, UnaryOp::Not => Bool, { Int32, Bool, String });
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

impl Value {
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
            Value::Ref(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
            Value::MutableRef(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
            Value::Deref(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
            Value::Mutable(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
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

    def_unary_val_method!(neg, |-rhs| -> Self, {
        (Int32)
    });

    def_unary_val_method!(not, |!rhs| -> Bool, {
        (Int32): *rhs == 0,
        (Bool),
        (String): rhs.len() == 0
    });
}
