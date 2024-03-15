#[derive(Debug, Clone)]
pub enum Op {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Define,
    Assign,
    NoOp,
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
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
            Op::NoOp => "no operation".to_string(),
        }
    }
}
