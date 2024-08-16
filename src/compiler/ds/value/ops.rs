use std::fmt::Debug;

use crate::compiler::traits::{ Dissasemble, OpTrait };

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

    fn build_llvm(&self) -> String {
        match self {
            Self::Add => "add".to_string(),
            Self::Sub => "sub".to_string(),
            Self::Div => "div".to_string(),
            Self::Mul => "mul".to_string(),
            Self::ComparisonOp(cmp) => cmp.build_llvm(),
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

    fn build_llvm(&self) -> String {
        match self {
            Self::Eq => "eq".to_string(),
            Self::Ne => "ne".to_string(),
            Self::Gt => "sgt".to_string(),
            Self::Ge => "sge".to_string(),
            Self::Le => "sle".to_string(),
            Self::Lt => "slt".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
    // Ref,
    // MutRef,
    // Deref,
}

impl Dissasemble for UnaryOp {
    fn dissasemble(&self) -> String {
        match self {
            Self::Neg => "-".to_string(),
            Self::Not => "!".to_string(),
            // Self::Ref => "&".to_string(),
            // Self::MutRef => "&mut ".to_string(),
            // Self::Deref => "*".to_string(),
        }
    }
}

impl OpTrait for UnaryOp {
    fn build_llvm(&self) -> String {
        unimplemented!()
    }

    fn get_op_len(&self) -> usize {
        match self {
            // Self::MutRef => self.dissasemble().len() - 1,
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
