pub(in crate::compiler) mod ops;

use std::rc::Rc;

use llvm_builder::{ LLVMType, Operand, RawOperand, Type, TypeI32 };
use ops::{ BinaryOp, ComparisonOp, UnaryOp };

use crate::{
    compiler::traits::Dissasemble,
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
    Int,
    Bool,
    Void,
    String,
    // Deref(Box<Self>),
    // Ref(Box<Self>),
    // MutableRef(Box<Self>),
}

impl Dissasemble for ValueType {
    fn dissasemble(&self) -> String {
        match self {
            Self::Int => "int".to_string(),
            Self::Bool => "bool".to_string(),
            Self::Void => "()".to_string(),
            Self::String => "string".to_string(),
            // Self::Deref(boxed) => format!("*{}", boxed.dissasemble()),
            // Self::Ref(boxed) => format!("&{}", boxed.dissasemble()),
            // Self::MutableRef(boxed) => format!("&mut {}", boxed.dissasemble()),
        }
    }
}

impl ValueType {
    pub fn is(&self, other: &ValueType) -> bool {
        self == other
    }

    pub fn to_type_string(&self) -> String {
        match self {
            ValueType::Int => "int".to_string(),
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
            // UnaryOp::Ref => Ok(Self::Ref(Box::new(self.clone()))),
            // UnaryOp::Deref => Ok(Self::Deref(Box::new(self.clone()))),
            // UnaryOp::MutRef => Ok(Self::MutableRef(Box::new(self.clone()))),
        }
    }

    // pub fn get_minified(value_type: &Self) -> Self {
    //     let mut minified_type = value_type.clone();
    //     match minified_type {
    //         ValueType::Deref(inner) => {
    //             match *inner {
    //                 ValueType::Ref(contained) => {
    //                     minified_type = Self::get_minified(&contained);
    //                     minified_type
    //                 }
    //                 ValueType::Deref(_) => {
    //                     minified_type = Self::get_minified(&inner);
    //                     minified_type
    //                 }
    //                 t => t,
    //             }
    //         }
    //         _ => minified_type,
    //     }
    // }

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
        (Int, Int),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_ne, ComparisonOp::Ne => Bool, {
        (Int, Int),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_gt, ComparisonOp::Gt => Bool, {
        (Int, Int),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_ge, ComparisonOp::Ge => Bool, {
        (Int, Int),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_lt, ComparisonOp::Lt => Bool, {
        (Int, Int),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_cmp_le, ComparisonOp::Le => Bool, {
        (Int, Int),
        (Bool, Bool),
        (String, String),
    });

    def_binary_try_method!(try_add, BinaryOp::Add, {
        (Int, Int),
        (String, String)
    });

    def_binary_try_method!(try_sub, BinaryOp::Sub, {
        (Int, Int),
    });

    def_binary_try_method!(try_mul, BinaryOp::Mul, {
        (Int, Int),
    });

    def_binary_try_method!(try_div, BinaryOp::Div, {
        (Int, Int),
    });

    def_unary_try_method!(try_neg, UnaryOp::Neg, { Int });

    def_unary_try_method!(try_not, UnaryOp::Not => Bool, { Int, Bool, String });
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SimpleConst {
    Int(i64),
    Bool(bool),
}

impl SimpleConst {
    pub fn get_as_i64(&self) -> i64 {
        match self {
            Self::Int(int) => *int,
            Self::Bool(bool) => *bool as i64,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ComplexConst {
    String(Rc<str>),
}

// pub enum ValueV2 {
//     SimpleConst(SimpleConst),
//     ComplexConst(ComplexConst),
//     Ref(Box<Self>),
//     MutableRef(Box<Self>),
//     Deref(Box<Self>),
//     Mutable(Box<Self>),
//     Void,
// }

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(Rc<str>),
    // Ref(Box<Self>),
    // MutableRef(Box<Self>),
    // Deref(Box<Self>),
    // Mutable(Box<Self>),
    Void,
}

impl Dissasemble for Value {
    fn dissasemble(&self) -> String {
        match self {
            Self::Int(int) => int.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::String(str) => str.to_string(),
            Self::Void => "()".to_string(),
            // Self::Deref(contained) => format!("*{}", contained.dissasemble()),
            // Self::Ref(contained) => format!("&{}", contained.dissasemble()),
            // Self::MutableRef(contained) => format!("&mut {}", contained.dissasemble()),
            // Self::Mutable(contained) => format!("mut {}", contained.dissasemble()),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Bool(false) // Remove this (only used in outdated VM)
    }
}

impl Value {
    pub fn get_llvm_operand(&self) -> Operand {
        match self {
            Self::Int(int) => { Operand::Raw(RawOperand::TypeI32(TypeI32::new(*int as i32))) }
            _ => { unimplemented!() }
        }
    }

    pub fn get_as_simple_const(&self) -> Option<SimpleConst> {
        match self {
            Self::Int(int) => Some(SimpleConst::Int(*int)),
            Self::Bool(bool) => Some(SimpleConst::Bool(*bool)),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Int(int) => int.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::String(str) => str.to_string(),
            Self::Void => String::new(),
            _ => "not impl yet".to_string(),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::Bool(_) => ValueType::Bool,
            Value::String(_) => ValueType::String,
            Value::Void => ValueType::Void,
            // Value::Ref(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
            // Value::MutableRef(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
            // Value::Deref(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
            // Value::Mutable(boxed) => { ValueType::Ref(Box::new(boxed.to_value_type())) }
        }
    }

    def_binary_val_method!(cmp_eq, |lhs == rhs| -> Bool, {
        (Int, Int),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_ne, |lhs != rhs| -> Bool, {
        (Int, Int),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_gt, |lhs > rhs| -> Bool, {
        (Int, Int),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_ge, |lhs >= rhs| -> Bool, {
        (Int, Int),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_lt, |lhs < rhs| -> Bool, {
        (Int, Int),
        (Bool, Bool),
    });

    def_binary_val_method!(cmp_le, |lhs <= rhs| -> Bool, {
        (Int, Int),
        (Bool, Bool),
    });

    def_binary_val_method!(add, |lhs + rhs| -> Self, {
        (Int, Int),
        (String, String): format!("{}{}", lhs, rhs).into(),
    });

    def_binary_val_method!(mul, |lhs * rhs| -> Self, {
        (Int, Int),
    });

    def_binary_val_method!(div, |lhs / rhs| -> Self, {
        (Int, Int),
    });

    def_binary_val_method!(sub, |lhs - rhs| -> Self, {
        (Int, Int),
    });

    def_unary_val_method!(neg, |-rhs| -> Self, {
        (Int)
    });

    def_unary_val_method!(not, |!rhs| -> Bool, {
        (Int): *rhs == 0,
        (Bool),
        (String): rhs.len() == 0
    });
}
