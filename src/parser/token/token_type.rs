#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    TokenLeftParen,
    TokenRightParen,
    TokenMinus,
    TokenPlus,
    TokenSlash,
    TokenStar,
    TokenNumber,
    TokenIdentifier,
    TokenTrue,
    TokenFalse,
    TokenSemicolon,
    TokenBang,
    TokenAssign,
    TokenDefine,
    TokenError,
    TokenEOF,
}
impl From<TokenType> for usize {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::TokenLeftParen => 0,
            TokenType::TokenRightParen => 1,
            TokenType::TokenMinus => 2,
            TokenType::TokenPlus => 3,
            TokenType::TokenSlash => 4,
            TokenType::TokenStar => 5,
            TokenType::TokenNumber => 6,
            TokenType::TokenIdentifier => 7,
            TokenType::TokenTrue => 8,
            TokenType::TokenFalse => 9,
            TokenType::TokenSemicolon => 10,
            TokenType::TokenBang => 11,
            TokenType::TokenAssign => 12,
            TokenType::TokenDefine => 13,
            TokenType::TokenError => 14,
            TokenType::TokenEOF => 15,
        }
    }
}
