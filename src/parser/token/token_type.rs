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
    TokenInt32,
    TokenSemicolon,
    TokenBang,
    TokenAssign,
    TokenDefine,
    TokenMutable,
    TokenFunction,
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
            TokenType::TokenInt32 => 10,
            TokenType::TokenSemicolon => 11,
            TokenType::TokenBang => 12,
            TokenType::TokenAssign => 13,
            TokenType::TokenDefine => 14,
            TokenType::TokenMutable => 15,
            TokenType::TokenFunction => 16,
            TokenType::TokenError => 17,
            TokenType::TokenEOF => 18,
        }
    }
}
