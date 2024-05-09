#[derive(Debug, Clone, Copy)]
pub enum Op {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Define,
    Assign,
    StartScope,
    EndScope,
    NoOp,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl BinaryOp {
    pub fn to_op_string(&self) -> String {
        (
            match self {
                Self::Add => "Addition",
                Self::Sub => "Subtraction",
                Self::Mul => "Multiplication",
                Self::Div => "Division",
                Self::Equal => "Equal",
                Self::NotEqual => "NotEqual",
                Self::Greater => "Greater",
                Self::GreaterEqual => "GreaterEqual",
                Self::Less => "Less",
                Self::LessEqual => "LessEqual",
            }
        ).to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    Truthy,
}

impl UnaryOp {
    pub fn get_op_len(&self) -> usize {
        match self {
            UnaryOp::Neg => 1,
            UnaryOp::Truthy => 1,
        }
    }

    pub fn to_op_string(&self) -> String {
        (
            match self {
                Self::Neg => "Negation",
                Self::Truthy => "Truthy",
            }
        ).to_string()
    }
}

impl Op {
    pub fn to_op_string(&self) -> String {
        match self {
            Op::BinaryOp(binary_op) =>
                match binary_op {
                    BinaryOp::Add => "addition".to_string(),
                    BinaryOp::Sub => "subtraction".to_string(),
                    BinaryOp::Mul => "multiplication".to_string(),
                    BinaryOp::Div => "division".to_string(),
                    BinaryOp::Equal => "equal".to_string(),
                    BinaryOp::NotEqual => "not equal".to_string(),
                    BinaryOp::Greater => "greater".to_string(),
                    BinaryOp::GreaterEqual => "greater equal".to_string(),
                    BinaryOp::Less => "less".to_string(),
                    BinaryOp::LessEqual => "less equal".to_string(),
                }
            Op::UnaryOp(unary_op) =>
                match unary_op {
                    UnaryOp::Neg => "negation".to_string(),
                    UnaryOp::Truthy => "truthy".to_string(),
                }

            Op::Define => "definement".to_string(),
            Op::Assign => "assignment".to_string(),
            Op::StartScope => "start scope".to_string(),
            Op::EndScope => "end scope".to_string(),
            Op::NoOp => "no operation".to_string(),
        }
    }
}
