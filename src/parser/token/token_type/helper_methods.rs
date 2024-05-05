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
}
