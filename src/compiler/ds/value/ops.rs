use std::fmt::Debug;

use crate::compiler::Dissasemble;

pub trait OpTrait: Dissasemble + Debug + Clone + Copy {
    fn get_op_len(&self) -> usize;

    fn get_can_constant_fold(&self) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    ComparisonOp(ComparisonOp),
}

impl Dissasemble for BinaryOp {
    fn dissasemble(&self) -> String {
        match self {
            Self::Add => "+".to_string(),
            Self::Sub => "-".to_string(),
            Self::Div => "/".to_string(),
            Self::Mul => "*".to_string(),
            Self::ComparisonOp(comparison_op) => comparison_op.dissasemble(),
        }
    }
}

impl OpTrait for BinaryOp {
    fn get_op_len(&self) -> usize {
        self.dissasemble().len()
    }

    fn get_can_constant_fold(&self) -> bool {
        match self {
            Self::ComparisonOp(comparison_op) => comparison_op.get_can_constant_fold(),
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComparisonOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl Dissasemble for ComparisonOp {
    fn dissasemble(&self) -> String {
        match self {
            Self::Eq => "==".to_string(),
            Self::Ne => "!=".to_string(),
            Self::Gt => ">".to_string(),
            Self::Ge => ">=".to_string(),
            Self::Lt => "<".to_string(),
            Self::Le => "<=".to_string(),
        }
    }
}

impl OpTrait for ComparisonOp {
    fn get_op_len(&self) -> usize {
        self.dissasemble().len()
    }

    fn get_can_constant_fold(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
    Ref,
    MutRef,
    Deref,
}

impl Dissasemble for UnaryOp {
    fn dissasemble(&self) -> String {
        match self {
            Self::Neg => "-".to_string(),
            Self::Not => "!".to_string(),
            Self::Ref => "&".to_string(),
            Self::MutRef => "&mut ".to_string(),
            Self::Deref => "*".to_string(),
        }
    }
}

impl OpTrait for UnaryOp {
    fn get_op_len(&self) -> usize {
        match self {
            Self::MutRef => self.dissasemble().len() - 1,
            _ => self.dissasemble().len(),
        }
    }

    fn get_can_constant_fold(&self) -> bool {
        match self {
            Self::Neg | Self::Not => true,
            _ => false,
        }
    }
}
