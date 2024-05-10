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
pub enum ComparisonOp {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    ComparisonOp(ComparisonOp),
}

impl BinaryOp {
    pub fn to_op_string(&self) -> String {
        (
            match self {
                Self::Add => "Addition",
                Self::Sub => "Subtraction",
                Self::Mul => "Multiplication",
                Self::Div => "Division",
                Self::ComparisonOp(ComparisonOp::Equal) => "Equal",
                Self::ComparisonOp(ComparisonOp::NotEqual) => "NotEqual",
                Self::ComparisonOp(ComparisonOp::Greater) => "Greater",
                Self::ComparisonOp(ComparisonOp::GreaterEqual) => "GreaterEqual",
                Self::ComparisonOp(ComparisonOp::Less) => "Less",
                Self::ComparisonOp(ComparisonOp::LessEqual) => "LessEqual",
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
                    BinaryOp::ComparisonOp(ComparisonOp::Equal) => "equal".to_string(),
                    BinaryOp::ComparisonOp(ComparisonOp::NotEqual) => "not equal".to_string(),
                    BinaryOp::ComparisonOp(ComparisonOp::Greater) => "greater".to_string(),
                    BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual) =>
                        "greater equal".to_string(),
                    BinaryOp::ComparisonOp(ComparisonOp::Less) => "less".to_string(),
                    BinaryOp::ComparisonOp(ComparisonOp::LessEqual) => "less equal".to_string(),
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
