use crate::{ operations::{ BinaryOp, Op, UnaryOp }, util::make_first_char_uppercase };

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int32,
    Bool,
}
impl ValueType {
    pub fn to_type_string(&self) -> String {
        match self {
            ValueType::Int32 => "i32".to_string(),
            ValueType::Bool => "bool".to_string(),
        }
    }

    pub fn type_check_binary(&self, other: &ValueType, op: BinaryOp) -> Result<ValueType, String> {
        match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                if self == other {
                    Ok(self.clone())
                } else {
                    Err(
                        format!(
                            "{} is not defined for {} and {}",
                            make_first_char_uppercase(Op::BinaryOp(op).to_op_string()),
                            self.to_type_string(),
                            other.to_type_string()
                        )
                    )
                }
            }
        }
    }

    pub fn type_check_unary(&self, op: UnaryOp) -> Result<ValueType, String> {
        match op {
            UnaryOp::Neg | UnaryOp::Truthy => {
                if self == &ValueType::Int32 {
                    Ok(self.clone())
                } else {
                    Err(
                        format!(
                            "{} is not defined for {}",
                            make_first_char_uppercase(Op::UnaryOp(op).to_op_string()),
                            self.to_type_string()
                        )
                    )
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int32(i32),
    Bool(bool),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int32(int32) => int32.to_string(),
            Value::Bool(bool) => bool.to_string(),
        }
    }

    pub fn to_value_type(&self) -> ValueType {
        match self {
            Value::Int32(_) => ValueType::Int32,
            Value::Bool(_) => ValueType::Bool,
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
        }
    }
}
