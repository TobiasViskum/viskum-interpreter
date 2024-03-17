use crate::parser::token::token_type::TokenType;

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn advance(&mut self) {
        self.current += 1;
    }

    #[profiler::function_tracker]
    pub(super) fn peek(&self, offset: i8) -> Option<&char> {
        let index = ((self.current as isize) + (offset as isize)) as usize;

        if index > self.source.len() {
            None
        } else {
            let result = self.source.get(index);
            result
        }
    }

    pub(super) fn is(&self, offset: i8, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.peek(offset) {
            c == &expected
        } else {
            false
        }
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    #[profiler::function_tracker]
    pub(super) fn get_character(&self, index: usize) -> &char {
        self.source.get(index).unwrap()
    }

    #[profiler::function_tracker]
    pub(super) fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        ttype: TokenType
    ) -> TokenType {
        let mut search_lexeme = String::new();

        for i in 0..length {
            search_lexeme.push(*self.source.get(self.start + start + i).unwrap());
        }

        if self.current - self.start == start + length && search_lexeme == rest {
            ttype
        } else {
            TokenType::TokenIdentifier
        }
    }
}
