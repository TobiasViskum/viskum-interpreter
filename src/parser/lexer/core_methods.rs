use crate::parser::token::{ Token, token_type::TokenType };

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn make_token(&mut self, ttype: TokenType) -> Token {
        Token::new(ttype, self.start, self.current - self.start, self.line)
    }

    pub(super) fn make_identifier_token(&mut self) -> Token {
        let ttype = match self.get_character(self.start) {
            't' => self.check_keyword(1, 3, "rue", TokenType::TokenTrue),
            'f' => self.check_keyword(1, 4, "alse", TokenType::TokenFalse),
            _ => TokenType::TokenIdentifier,
        };

        self.make_token(ttype)
    }

    pub(super) fn make_error_token(&mut self, message: String) -> Token {
        Token::new_error(
            TokenType::TokenError,
            self.start,
            self.current - self.start,
            self.line,
            message
        )
    }
}
