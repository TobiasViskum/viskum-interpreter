use crate::operations::{ BinaryOp, UnaryOp };

use super::TokenType;

impl TokenType {
    pub fn is_type(&self) -> bool {
        match self {
            Self::TokenInt32 | Self::TokenBool => true,
            _ => false,
        }
    }

    pub fn is(&self, other: &TokenType) -> bool {
        self == other
    }

    pub fn parse_binary(&self) -> Result<BinaryOp, ()> {
        match self {
            Self::TokenPlus => Ok(BinaryOp::Add),
            Self::TokenMinus => Ok(BinaryOp::Sub),
            Self::TokenSlash => Ok(BinaryOp::Div),
            Self::TokenStar => Ok(BinaryOp::Mul),
            Self::TokenEqualEqual => Ok(BinaryOp::Equal),
            Self::TokenBangEqual => Ok(BinaryOp::NotEqual),
            Self::TokenGreater => Ok(BinaryOp::Greater),
            Self::TokenGreaterEqual => Ok(BinaryOp::GreaterEqual),
            Self::TokenLess => Ok(BinaryOp::Less),
            Self::TokenLessEqual => Ok(BinaryOp::LessEqual),
            _ => Err(()),
        }
    }

    pub fn parse_unary(&self) -> Result<UnaryOp, ()> {
        match self {
            Self::TokenBang => Ok(UnaryOp::Truthy),
            Self::TokenMinus => Ok(UnaryOp::Neg),
            _ => Err(()),
        }
    }
}
