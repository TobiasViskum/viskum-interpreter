use crate::parser::token::{ token_type::TokenType, Token };

mod core_methods;
mod helper_methods;
mod lexer_util;

use lexer_util::*;

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    panic_mode: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            panic_mode: false,
        }
    }

    pub fn free(&mut self) {
        self.source = "";
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek(0) {
            if self.is_at_end() {
                break;
            }
            match c {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
    }

    #[profiler::function_tracker]
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_at_end() {
            return self.make_token(TokenType::TokenEOF);
        }

        self.start = self.current;

        self.advance();

        let c = self.peek(-1).unwrap();

        match c {
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '+' => self.make_token(TokenType::TokenPlus),
            '-' => self.make_token(TokenType::TokenMinus),
            '*' => self.make_token(TokenType::TokenStar),
            '/' => self.make_token(TokenType::TokenSlash),
            ';' => self.make_token(TokenType::TokenSemicolon),
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
