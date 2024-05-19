use std::{ fmt::{ self, Display }, rc::Rc };

use crate::{ operations::{ BinaryOp, ComparisonOp, UnaryOp }, vm::Instructions };

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    Int32,
    Bool,
    Void,
    String,
    Empty,
    Function,
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
            ValueType::Empty => "empty".to_string(),
            ValueType::String => "string".to_string(),
            ValueType::Function => "<fn>".to_string(),
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
            UnaryOp::Truthy => Ok(*self),
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

    pub fn try_add(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
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
            Ok(*self)
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
}

#[derive(Debug, Clone)]
pub enum Value {
    Int32(i32),
    Bool(bool),
    String(Rc<str>),
    Void,
    Empty,
    Function(Function),
}

impl Default for Value {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int32(int32) => write!(f, "{}", int32),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Empty => write!(f, "empty"),
            Value::Function(_) => { write!(f, "<fn>") }
            Value::String(str) => write!(f, "{}", str),
            Value::Void => write!(f, "()"),
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Self::Int32(i32) => i32.to_string(),
            Self::Bool(bool) => bool.to_string(),
            Self::String(str) => str.to_string(),
            Self::Empty | Self::Void => String::new(),
            Self::Function(_) => "<fn>".to_string(),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        match self {
            Value::Int32(_) => ValueType::Int32,
            Value::Bool(_) => ValueType::Bool,
            Value::Empty => ValueType::Empty,
            Value::Function(_) => ValueType::Function,
            Value::String(_) => ValueType::String,
            Value::Void => ValueType::Void,
        }
    }

    #[inline(always)]
    pub fn cmp_e(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(lhs == rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(lhs == rhs),
            _ =>
                Err(
                    format!(
                        "Comparison '==' is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn cmp_ne(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(lhs != rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(lhs != rhs),
            _ =>
                Err(
                    format!(
                        "Comparison '!=' is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn cmp_g(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(lhs > rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(lhs > rhs),
            _ =>
                Err(
                    format!(
                        "Comparison '>' is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn cmp_ge(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(lhs >= rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(lhs >= rhs),
            _ =>
                Err(
                    format!(
                        "Comparison '>=' is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn cmp_l(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(lhs < rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(lhs < rhs),
            _ =>
                Err(
                    format!(
                        "Comparison '<' is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn cmp_le(&self, other: &Value) -> Result<bool, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(lhs <= rhs),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(lhs <= rhs),
            _ =>
                Err(
                    format!(
                        "Comparison '<=' is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn add(&self, other: &Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs + rhs)),
            (Value::String(lhs), Value::String(rhs)) =>
                Ok(Value::String(format!("{}{}", lhs.as_ref(), rhs.as_ref()).into())),
            _ =>
                Err(
                    format!(
                        "Addition is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn mul(&self, other: &Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs * rhs)),
            _ =>
                Err(
                    format!(
                        "Multiplication is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn div(&self, other: &Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs / rhs)),
            _ =>
                Err(
                    format!(
                        "Division is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn sub(&self, other: &Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs - rhs)),
            _ =>
                Err(
                    format!(
                        "Subtraction is not defined for {} and {}",
                        self.to_value_type().to_type_string(),
                        other.to_value_type().to_type_string()
                    )
                ),
        }
    }

    #[inline(always)]
    pub fn neg(&self) -> Result<Self, String> {
        match self {
            Value::Int32(int32) => Ok(Value::Int32(-int32)),
            v => Err(format!("Negation is not defined for {}", v.to_value_type().to_type_string())),
        }
    }

    #[inline(always)]
    pub fn not(&self) -> Self {
        match self {
            Value::Bool(bool) => Value::Bool(!*bool),
            Value::Int32(int32) => Value::Bool(*int32 == 0),
            Value::String(str) => Value::Bool((*str).len() == 0),
            Value::Function(_) => Value::Bool(false),
            Value::Void | Value::Empty =>
                panic!("Cannot do operations on {}", self.to_value_type().to_type_string()),
        }
    }
}
