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

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn _get_op_len(&self) -> usize {
        match self {
            BinaryOp::Add => 1,
            BinaryOp::Sub => 1,
            BinaryOp::Mul => 1,
            BinaryOp::Div => 1,
        }
    }

    pub fn to_op_string(&self) -> String {
        (
            match self {
                Self::Add => "Addition",
                Self::Sub => "Subtraction",
                Self::Mul => "Multiplication",
                Self::Div => "Division",
            }
        ).to_string()
    }
}

#[derive(Debug, Clone, Copy)]
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
