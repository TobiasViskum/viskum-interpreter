use super::TokenType;

impl TokenType {
    pub fn is_type(&self) -> bool {
        match self {
            Self::TokenInt32 | Self::TokenBool => true,
            _ => false,
        }
    }
}
