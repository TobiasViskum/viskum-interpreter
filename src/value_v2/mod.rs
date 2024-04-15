use ahash::AHashMap;
use lazy_static::lazy_static;

use crate::{ ast::stmt::FunctionArgument, operations::{ BinaryOp, UnaryOp } };

#[derive(Debug, Clone)]
pub struct ValueHolder {
    pub value_type: ValueType,
    pub value: Value,
}

impl ValueHolder {
    pub fn new_bool(bool: bool) -> Self {
        Self {
            value: Value::Bool(bool),
            value_type: ValueType::Bool(&VALUE_TYPE_BOOL),
        }
    }

    pub fn new_int32(i32: i32) -> Self {
        Self {
            value: Value::Int32(i32),
            value_type: ValueType::Int32(&VALUE_TYPE_INT),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Empty,
    Int32(i32),
    Bool(bool),
    // Custom {
    //     name: usize,
    //     value: Box<Value>,
    // },
    // String(String),
    // Vector(Vec<Value>),
    // HashMap(AHashMap<String, Value>),
}

impl Value {
    pub fn to_type_string(&self) -> String {
        (
            match self {
                Value::Int32(_) => "Int32",
                Value::Bool(_) => "booli32",
                Value::Empty => "empty",
                // Value::Custom { .. } => panic!("String should be a usize if I want to impl Copy"),
            }
        ).to_string()
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Empty => "EMPTY".to_string(),
            Self::Int32(i32) => i32.to_string(),
            Self::Bool(bool) => bool.to_string(),
            // Self::Custom { name, value } => (*value).to_string(),
        }
    }

    pub fn add(&self, other: &Value) -> Result<Self, String> {
        let op = BinaryOp::Add;

        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs + rhs)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Self, String> {
        let op = BinaryOp::Mul;

        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs * rhs)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Self, String> {
        let op = BinaryOp::Div;

        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs / rhs)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Self, String> {
        let op = BinaryOp::Sub;

        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs - rhs)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn neg(&self) -> Result<Self, String> {
        let op = UnaryOp::Neg;

        match self {
            Value::Int32(int32) => Ok(Value::Int32(-int32)),
            v => Err(format!("{} is not defined for {}", op.to_op_string(), v.to_type_string())),
        }
    }

    pub fn not(&self) -> Result<Self, String> {
        let op = UnaryOp::Truthy;

        match self {
            Value::Bool(bool) => Ok(Value::Bool(!*bool)),
            Value::Int32(int32) => Ok(Value::Bool(!*int32 != 0)),
            v => Err(format!("{} is not defined for {}", op.to_op_string(), v.to_type_string())),
            // Value::Empty => Value::Empty,
        }
    }
}

// pub enum BaseValueType {
//     Int32,
//     String,
//     Bool,
//     Vector,
//     HashMap,
// }

#[derive(Debug, Clone)]
pub enum ValueType {
    Empty,
    Unkown,
    Int32(&'static ValueTypeAttrs),
    Bool(&'static ValueTypeAttrs),
    // Custom {
    //     name: String,
    //     attrs: &'static ValueTypeAttrs,
    // },
    // String(&'static ValueTypeAttrs),
    // Vector(&'static ValueTypeAttrs),
    // HashMap(&'static ValueTypeAttrs),
}

impl ValueType {
    pub fn new_int32() -> Self {
        Self::Int32(&VALUE_TYPE_INT)
    }

    pub fn new_bool() -> Self {
        Self::Bool(&VALUE_TYPE_BOOL)
    }

    pub fn is(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => true,
            (Self::Unkown, Self::Unkown) => true,
            (Self::Int32(_), Self::Int32(_)) => true,
            (Self::Bool(_), Self::Bool(_)) => true,
            _ => false,
        }
    }

    pub fn to_type_string(&self) -> String {
        (
            match self {
                Self::Int32(_) => "i32",
                Self::Bool(_) => "bool",
                Self::Empty => "empty",
                Self::Unkown => "unkown",
                // Self::Custom { name, .. } => name,
            }
        ).to_string()
    }

    pub fn type_check_binary(&self, other: &ValueType, op: BinaryOp) -> Result<Self, String> {
        match op {
            BinaryOp::Add => self.try_add(other),
            BinaryOp::Mul => self.try_mul(other),
            BinaryOp::Div => self.try_div(other),
            BinaryOp::Sub => self.try_sub(other),
        }
    }

    pub fn type_check_unary(&self, op: UnaryOp) -> Result<Self, String> {
        match op {
            UnaryOp::Neg => self.try_neg(),
            UnaryOp::Truthy => Ok(self.clone()),
        }
    }

    pub fn try_add(&self, other: &Self) -> Result<Self, String> {
        let op = BinaryOp::Add;
        match (self, other) {
            (Self::Int32(_), Self::Int32(_)) => Ok(Self::Int32(&VALUE_TYPE_INT)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_mul(&self, other: &Self) -> Result<Self, String> {
        let op = BinaryOp::Mul;
        match (self, other) {
            (Self::Int32(_), Self::Int32(_)) => Ok(Self::Int32(&VALUE_TYPE_INT)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_sub(&self, other: &Self) -> Result<Self, String> {
        let op = BinaryOp::Sub;
        match (self, other) {
            (Self::Int32(_), Self::Int32(_)) => Ok(Self::Int32(&VALUE_TYPE_INT)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_div(&self, other: &Self) -> Result<Self, String> {
        let op = BinaryOp::Div;
        match (self, other) {
            (Self::Int32(_), Self::Int32(_)) => Ok(Self::Int32(&VALUE_TYPE_INT)),
            _ =>
                Err(
                    format!(
                        "{} is not defined for {} and {}",
                        op.to_op_string(),
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }

    pub fn try_neg(&self) -> Result<Self, String> {
        let op = UnaryOp::Neg;

        match self {
            Self::Int32(_) => Ok(Self::Int32(&VALUE_TYPE_INT)),
            v => Err(format!("{} is not defined for {}", op.to_op_string(), v.to_type_string())),
        }
    }

    pub fn try_truthy(&self) -> Result<Self, String> {
        let op = UnaryOp::Truthy;

        match self {
            Self::Int32(_) => Ok(Self::Int32(&VALUE_TYPE_INT)),
            v => Err(format!("{} is not defined for {}", op.to_op_string(), v.to_type_string())),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ValueTypeAttrs {
    pub operations: ValueTypeOperations,
    pub methods: AHashMap<&'static str, Method>,
}
#[derive(Debug, Clone)]
pub struct ValueTypeOperations {
    pub add: Option<fn(Value, Value) -> Result<Value, String>>,
    pub sub: Option<fn(Value, Value) -> Result<Value, String>>,
    pub mul: Option<fn(Value, Value) -> Result<Value, String>>,
    pub div: Option<fn(Value, Value) -> Result<Value, String>>,

    pub neg: Option<fn(Value) -> Result<Value, String>>,
    pub not: Option<fn(Value) -> Result<Value, String>>,
}
#[derive(Debug, Clone)]
pub struct Method {
    pub args: Option<Vec<FunctionArgument>>,
    pub return_type: ValueType,
}

lazy_static! {
    pub static ref VALUE_TYPE_BOOL: ValueTypeAttrs = ValueTypeAttrs {
        operations: ValueTypeOperations {
            add: None,
            div: None,
            mul: None,
            sub: None,

            neg: None,
            not: Some(|value| value.not()),
        },
        methods: {
            let methods = AHashMap::new();

            methods
        },
    };

    pub static ref VALUE_TYPE_INT: ValueTypeAttrs = ValueTypeAttrs {
        operations: ValueTypeOperations {
            add: Some(|lhs, rhs| lhs.add(&rhs)),
            sub: Some(|lhs, rhs| lhs.sub(&rhs)),
            mul: Some(|lhs, rhs| lhs.mul(&rhs)),
            div: Some(|lhs, rhs| lhs.div(&rhs)),
            neg: Some(|value| value.neg()),
            not: Some(|value| value.not()),
        },
        methods: {
            let mut methods = AHashMap::new();
            methods.insert("toBool", Method {
                args: None,
                return_type: ValueType::Bool(&VALUE_TYPE_BOOL),
            });

            methods
        },
    };
}
