use crate::Dissasemble;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Dissasemble for ComparisonOp {
    fn dissasemble(&self) -> String {
        (
            match self {
                Self::Equal => "==",
                Self::NotEqual => "!=",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
                Self::Less => "<",
                Self::LessEqual => "<=",
            }
        ).to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    ComparisonOp(ComparisonOp),
}

impl Dissasemble for BinaryOp {
    fn dissasemble(&self) -> String {
        (
            match self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Div => "/",
                Self::Mul => "*",
                Self::ComparisonOp(comparison_op) => {
                    return comparison_op.dissasemble();
                }
            }
        ).to_string()
    }
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
    Ref,
    MutRef,
    Deref,
}

impl Dissasemble for UnaryOp {
    fn dissasemble(&self) -> String {
        (
            match self {
                Self::Neg => "-",
                Self::Truthy => "!",
                Self::Ref => "&",
                Self::MutRef => "&mut ",
                Self::Deref => "*",
            }
        ).to_string()
    }
}

impl UnaryOp {
    pub fn get_op_len(&self) -> usize {
        match self {
            Self::Neg => 1,
            Self::Truthy => 1,
            Self::Ref => 1,
            Self::MutRef => 4,
            Self::Deref => 1,
        }
    }

    pub fn to_op_string(&self) -> String {
        (
            match self {
                Self::Neg => "Negation",
                Self::Truthy => "Truthy",
                Self::Ref => "Reference",
                Self::MutRef => "Mutable reference",
                Self::Deref => "Dereference",
            }
        ).to_string()
    }
}

// impl Op {
//     pub fn to_op_string(&self) -> String {
//         match self {
//             Op::BinaryOp(binary_op) =>
//                 match binary_op {
//                     BinaryOp::Add => "addition".to_string(),
//                     BinaryOp::Sub => "subtraction".to_string(),
//                     BinaryOp::Mul => "multiplication".to_string(),
//                     BinaryOp::Div => "division".to_string(),
//                     BinaryOp::ComparisonOp(ComparisonOp::Equal) => "equal".to_string(),
//                     BinaryOp::ComparisonOp(ComparisonOp::NotEqual) => "not equal".to_string(),
//                     BinaryOp::ComparisonOp(ComparisonOp::Greater) => "greater".to_string(),
//                     BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual) =>
//                         "greater equal".to_string(),
//                     BinaryOp::ComparisonOp(ComparisonOp::Less) => "less".to_string(),
//                     BinaryOp::ComparisonOp(ComparisonOp::LessEqual) => "less equal".to_string(),
//                 }
//             Op::UnaryOp(unary_op) =>
//                 match unary_op {
//                     UnaryOp::Neg => "negation".to_string(),
//                     UnaryOp::Truthy => "truthy".to_string(),
//                 }

//             Op::Define => "definement".to_string(),
//             Op::Assign => "assignment".to_string(),
//             Op::StartScope => "start scope".to_string(),
//             Op::EndScope => "end scope".to_string(),
//             Op::NoOp => "no operation".to_string(),
//         }
//     }
// }
