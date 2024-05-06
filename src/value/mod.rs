use crate::operations::{ BinaryOp, UnaryOp };

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ValueType {
    Int32,
    Bool,
    Unkown,
    Empty,
    Void,
}
impl ValueType {
    pub fn is(&self, other: &ValueType) -> bool {
        self == other
    }

    pub fn to_type_string(&self) -> String {
        match self {
            ValueType::Int32 => "i32".to_string(),
            ValueType::Bool => "bool".to_string(),
            ValueType::Unkown => "unknown".to_string(),
            ValueType::Empty => "empty".to_string(),
            ValueType::Void => "void".to_string(),
        }
    }

    pub fn type_check_binary(&self, other: &ValueType, op: BinaryOp) -> Result<ValueType, String> {
        match op {
            BinaryOp::Add => self.try_add(other),
            BinaryOp::Mul => self.try_mul(other),
            BinaryOp::Div => self.try_div(other),
            BinaryOp::Sub => self.try_sub(other),
            | BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Greater
            | BinaryOp::GreaterEqual
            | BinaryOp::Less
            | BinaryOp::LessEqual => self.try_cmp(other, op),
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
                    BinaryOp::Equal => "==",
                    BinaryOp::NotEqual => "!=",
                    BinaryOp::Greater => ">",
                    BinaryOp::GreaterEqual => ">=",
                    BinaryOp::Less => "<",
                    BinaryOp::LessEqual => "<=",
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
            _ => {
                Err(
                    format!(
                        "Addition is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                )
            }
        }
    }

    pub fn try_mul(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            _ => {
                Err(
                    format!(
                        "Multiplication is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                )
            }
        }
    }

    pub fn try_div(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            _ => {
                Err(
                    format!(
                        "Division is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                )
            }
        }
    }

    pub fn try_sub(&self, other: &ValueType) -> Result<ValueType, String> {
        match (self, other) {
            (ValueType::Int32, ValueType::Int32) => Ok(ValueType::Int32),
            _ => {
                Err(
                    format!(
                        "Subtraction is not defined for {} and {}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                )
            }
        }
    }

    pub fn try_neg(&self) -> Result<ValueType, String> {
        if self == &ValueType::Int32 {
            Ok(*self)
        } else {
            Err(format!("Negation is not defined for {}", self.to_type_string()))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int32(i32),
    Bool(bool),
    Empty,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int32(int32) => int32.to_string(),
            Value::Bool(bool) => bool.to_string(),
            Value::Empty => "empty".to_string(),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        match self {
            Value::Int32(_) => ValueType::Int32,
            Value::Bool(_) => ValueType::Bool,
            Value::Empty => ValueType::Empty,
        }
    }

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

    pub fn add(&self, other: &Value) -> Result<Self, String> {
        match (self, other) {
            (Value::Int32(lhs), Value::Int32(rhs)) => Ok(Value::Int32(lhs + rhs)),
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

    pub fn neg(&self) -> Result<Self, String> {
        match self {
            Value::Int32(int32) => Ok(Value::Int32(-int32)),
            v => Err(format!("Negation is not defined for {}", v.to_value_type().to_type_string())),
        }
    }

    pub fn not(&self) -> Self {
        match self {
            Value::Bool(bool) => Value::Bool(!*bool),
            Value::Int32(int32) => Value::Bool(!*int32 != 0),
            Value::Empty => Value::Empty,
        }
    }
}
