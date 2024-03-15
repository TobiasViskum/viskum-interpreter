use crate::parser::token::token_type::TokenType;

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn advance(&mut self) {
        self.current += 1;
    }

    pub(super) fn peek(&self, offset: i8) -> Option<char> {
        let index = ((self.current as i8) + offset) as usize;

        if index > self.source.chars().count() {
            None
        } else {
            self.source.chars().nth(index)
        }
    }

    pub(super) fn is(&self, offset: i8, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.peek(offset) {
            c == expected
        } else {
            false
        }
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    pub(super) fn get_character(&self, index: usize) -> char {
        self.source.chars().nth(index).unwrap()
    }

    pub(super) fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        ttype: TokenType
    ) -> TokenType {
        let mut search_lexeme = String::new();

        for i in 0..length {
            search_lexeme.push(
                self.source
                    .chars()
                    .nth(self.start + start + i)
                    .unwrap()
            );
        }

        if self.current - self.start == start + length && search_lexeme == rest {
            ttype
        } else {
            TokenType::TokenIdentifier
        }
    }
}
