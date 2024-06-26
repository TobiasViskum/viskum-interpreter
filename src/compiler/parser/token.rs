use std::rc::Rc;

use crate::compiler::{ error_handler::SrcCharsRange, parser::Lexeme };

use super::TokenType;

#[derive(Debug, Clone, Copy)]
pub struct TokenMetadata {
    start: usize,
    length: usize,
    line: usize,
}

impl Into<SrcCharsRange> for TokenMetadata {
    fn into(self) -> SrcCharsRange {
        let char_info = (self.start, self.start + self.length);
        let line_info = (self.line, self.line);
        SrcCharsRange::new(char_info, line_info)
    }
}

impl TokenMetadata {
    pub fn new(start: usize, length: usize, line: usize) -> Self {
        Self {
            start,
            length,
            line,
        }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_len(&self) -> usize {
        self.length
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
    optional_message: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, length: usize, line: usize) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
            optional_message: None,
        }
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        TokenMetadata {
            start: self.start,
            length: self.length,
            line: self.line,
        }
    }

    pub fn new_error(
        token_type: TokenType,
        start: usize,
        length: usize,
        line: usize,
        message: String
    ) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
            optional_message: Some(message),
        }
    }

    pub fn get_ttype(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_lexeme(&self, source: &Vec<char>) -> Lexeme {
        let mut lexeme = String::new();

        for i in self.start..self.start + self.length {
            lexeme.push(*source.get(i).unwrap());
        }

        let lexeme_str: Rc<str> = lexeme.into();
        Lexeme::new(lexeme_str)
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_message(&self) -> Option<&String> {
        self.optional_message.as_ref()
    }
}
