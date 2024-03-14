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

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Truthy,
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
