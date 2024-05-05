use crate::parser::token::{ token_type::TokenType, Token };

mod core_methods;
mod helper_methods;
mod lexer_util;

use lexer_util::*;

pub struct Lexer<'a> {
    source: &'a Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Vec<char>) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn free(&mut self) {
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    #[profiler::function_tracker]
    pub fn scan_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.skip_comment() {
            Some(token) => {
                return Some(token);
            }
            None => {}
        }

        if self.current == self.source.len() {
            return self.make_eof_token();
        } else if self.current > self.source.len() {
            return None;
        }

        self.start = self.current;

        self.advance();

        let c = self.peek(-1).unwrap();

        match c {
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '{' => self.make_token(TokenType::TokenLeftCurlyBrace),
            '}' => self.make_token(TokenType::TokenRightCurlyBrace),
            '[' => self.make_token(TokenType::TokenLeftSquareBracket),
            ']' => self.make_token(TokenType::TokenRightSquareBracket),
            '+' => self.make_token(TokenType::TokenPlus),
            '-' => self.make_token(TokenType::TokenMinus),
            '*' => self.make_token(TokenType::TokenStar),
            '/' => self.make_token(TokenType::TokenSlash),
            ';' => self.make_token(TokenType::TokenSemicolon),
            ',' => self.make_token(TokenType::TokenComma),
            '=' => {
                // if self.peek(0).unwrap() == '=' {
                //     self.advance();
                //     self.make_token(TokenType::TokenEqualEqual)
                // } else {
                self.make_token(TokenType::TokenAssign)
                // }
            }
            ':' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenType::TokenDefine)
                } else {
                    self.make_error_token(format!("Unexpected character: {}", c))
                }
            }
            '!' => {
                // if self.peek(0).unwrap() == '=' {
                //     self.advance();
                //     self.make_token(TokenType::TokenBangEqual)
                // } else {
                self.make_token(TokenType::TokenBang)
                // }
            }
            c => {
                if is_digit(Some(c)) {
                    while !self.is_at_end() && is_digit(self.peek(0)) {
                        self.advance();
                    }

                    self.make_token(TokenType::TokenNumber)
                } else if is_alphabetic(Some(c)) {
                    while
                        !self.is_at_end() &&
                        (is_alphabetic(self.peek(0)) || is_digit(self.peek(0)))
                    {
                        self.advance();
                    }

                    self.make_identifier_token()
                } else {
                    self.make_error_token(format!("Unexpected character: {}", c))
                }
            }
        }
    }
}
