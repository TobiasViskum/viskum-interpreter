use core::panic;
use std::{ marker::PhantomData, mem, time::Instant };

use crate::parser::{ token::Token, TokenType };

mod core_methods;
mod helper_methods;
mod lexer_util;

use lexer_util::*;

pub struct Lexer<'a> {
    source: &'a Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Vec<char>) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::with_capacity(256),
        }
    }

    pub fn free(&mut self) {
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    #[profiler::function_tracker]
    pub fn get_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        mem::take(&mut self.tokens)
    }

    #[profiler::function_tracker]
    pub fn scan_token(&mut self) {
        self.skip_whitespace();

        match self.skip_comment() {
            Some(token) => {
                self.tokens.push(token);
                return;
            }
            None => {}
        }

        if self.current == self.source.len() {
            self.make_eof_token();
            return;
        } else if self.current > self.source.len() {
            return;
        }

        self.start = self.current;

        self.advance();

        let c = self.peek(-1).unwrap();

        match c {
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '{' => self.make_token(TokenType::TokenLeftCurlyBrace),
            '}' => self.make_token(TokenType::TokenRightCurlyBrace),
            '[' => self.make_token(TokenType::TokenLeftSquareBrace),
            ']' => self.make_token(TokenType::TokenRightSquareBrace),
            '+' => self.make_token(TokenType::TokenPlus),
            '-' => self.make_token(TokenType::TokenMinus),
            '*' => self.make_token(TokenType::TokenStar),
            '/' => self.make_token(TokenType::TokenSlash),
            ';' => self.make_token(TokenType::TokenSemicolon),
            ',' => self.make_token(TokenType::TokenComma),
            '&' => {
                if self.is_keyword(1, 3, "mut") {
                    self.current += 3;
                    self.make_token(TokenType::TokenMutableReference)
                } else {
                    self.make_token(TokenType::TokenReference)
                }
            }
            '"' => {
                self.make_token(TokenType::TokenDoubleQuote);
                self.start = self.current;
                self.advance();
                self.string()
            }
            '<' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenType::TokenLessEqual)
                } else {
                    self.make_token(TokenType::TokenLess)
                }
            }
            '>' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenType::TokenGreaterEqual)
                } else {
                    self.make_token(TokenType::TokenGreater)
                }
            }
            '=' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenType::TokenEqualEqual)
                } else {
                    self.make_token(TokenType::TokenAssign)
                }
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
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenType::TokenBangEqual)
                } else {
                    self.make_token(TokenType::TokenBang)
                }
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
