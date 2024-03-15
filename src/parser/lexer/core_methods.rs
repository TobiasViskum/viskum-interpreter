use crate::parser::token::{ Token, token_type::TokenType };

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn make_token(&mut self, ttype: TokenType) -> Option<Token> {
        Some(Token::new(ttype, self.start, self.current - self.start, self.line))
    }

    pub(super) fn make_identifier_token(&mut self) -> Option<Token> {
        let ttype = match self.get_character(self.start) {
            'f' => self.check_keyword(1, 4, "alse", TokenType::TokenFalse),
            'i' => self.check_keyword(1, 2, "32", TokenType::TokenInt32),
            'm' => self.check_keyword(1, 2, "ut", TokenType::TokenMutable),
            't' => self.check_keyword(1, 3, "rue", TokenType::TokenTrue),

            _ => TokenType::TokenIdentifier,
        };

        self.make_token(ttype)
    }

    pub(super) fn make_error_token(&mut self, message: String) -> Option<Token> {
        Some(
            Token::new_error(
                TokenType::TokenError,
                self.start,
                self.current - self.start,
                self.line,
                message
            )
        )
    }
}
