use crate::operations::{ BinaryOp, ComparisonOp, UnaryOp };

use super::TokenType;

impl TokenType {
    pub fn is(&self, other: &TokenType) -> bool {
        self == other
    }

    pub fn parse_binary(&self) -> Result<BinaryOp, ()> {
        match self {
            Self::TokenPlus => Ok(BinaryOp::Add),
            Self::TokenMinus => Ok(BinaryOp::Sub),
            Self::TokenSlash => Ok(BinaryOp::Div),
            Self::TokenStar => Ok(BinaryOp::Mul),
            Self::TokenEqualEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::Equal)),
            Self::TokenBangEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::NotEqual)),
            Self::TokenGreater => Ok(BinaryOp::ComparisonOp(ComparisonOp::Greater)),
            Self::TokenGreaterEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual)),
            Self::TokenLess => Ok(BinaryOp::ComparisonOp(ComparisonOp::Less)),
            Self::TokenLessEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::LessEqual)),
            _ => Err(()),
        }
    }

    pub fn parse_unary(&self) -> Result<UnaryOp, ()> {
        match self {
            Self::TokenBang => Ok(UnaryOp::Truthy),
            Self::TokenMinus => Ok(UnaryOp::Neg),
            Self::TokenReference => Ok(UnaryOp::Ref),
            Self::TokenMutableReference => Ok(UnaryOp::MutRef),
            Self::TokenStar => Ok(UnaryOp::Deref),
            _ => Err(()),
        }
    }
}
