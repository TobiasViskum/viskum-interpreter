use crate::parser::token::{ Token, token_type::TokenType };

use super::{ is_alphabetic, Lexer };

impl<'a> Lexer<'a> {
    pub(super) fn make_eof_token(&mut self) {
        self.start = self.current;
        self.make_token(TokenType::TokenEOF)
    }

    pub(super) fn make_token(&mut self, ttype: TokenType) {
        let token = Token::new(ttype, self.start, self.current - self.start, self.line);
        self.tokens.push(token)
    }

    pub(super) fn make_identifier_token(&mut self) {
        let ttype = match self.get_character(self.start) {
            'b' => self.check_keyword(1, 4, "reak", TokenType::TokenBreak),
            'c' => self.check_keyword(1, 7, "ontinue", TokenType::TokenContinue),
            'e' => self.check_keyword(1, 3, "lse", TokenType::TokenElse),
            'f' => {
                if self.current - self.start > 1 {
                    match self.get_character(self.start + 1) {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::TokenFalse),
                        'n' => TokenType::TokenFunction,
                        _ => TokenType::TokenIdentifier,
                    }
                } else {
                    TokenType::TokenIdentifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::TokenIf),
            'l' => self.check_keyword(1, 3, "oop", TokenType::TokenLoop),
            'm' => self.check_keyword(1, 2, "ut", TokenType::TokenMutable),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::TokenReturn),
            't' => {
                if self.current - self.start > 1 {
                    match self.get_character(self.start + 1) {
                        'r' => self.check_keyword(2, 2, "ue", TokenType::TokenTrue),
                        'y' => self.check_keyword(2, 2, "pe", TokenType::TokenTyping),
                        _ => TokenType::TokenIdentifier,
                    }
                } else {
                    TokenType::TokenIdentifier
                }
            }
            // 'w' => self.check_keyword(1, 4, "hile", TokenType::TokenWhile),

            _ => TokenType::TokenIdentifier,
        };

        self.make_token(ttype)
    }

    pub(super) fn make_error_token(&mut self, message: String) {
        let token = Token::new_error(
            TokenType::TokenError,
            self.start,
            self.current - self.start,
            self.line,
            message
        );

        self.tokens.push(token)
    }

    pub(super) fn string(&mut self) {
        let mut incremented = 0;
        while !self.is(0, '"') {
            if self.is_at_end() || self.is(0, '\n') {
                self.start -= 1;
                self.current -= 1;
                if incremented == 0 {
                    self.make_error_token("Unexpected token: '\"'".to_string());
                } else {
                    self.make_error_token("Expected '\"' at the end of a string".to_string());
                }

                if self.is(0, '\n') {
                    self.line += 1;
                }

                return;
            }

            self.advance();
            incremented += 1;
        }

        self.make_token(TokenType::TokenString);

        self.start = self.current;
        self.advance();

        self.make_token(TokenType::TokenDoubleQuote);
    }

    pub(super) fn skip_whitespace(&mut self) {
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

    pub(super) fn skip_comment(&mut self) -> Option<Token> {
        if self.is(0, '/') && self.is(1, '/') {
            self.skip_single_line_comment();
        } else if self.is(0, '/') && self.is(1, '*') {
            self.skip_multi_line_comment();
        }
        None
    }

    fn skip_single_line_comment(&mut self) {
        while !self.is_at_end() {
            self.advance();
            if self.is(0, '\n') {
                self.line += 1;
                self.skip_whitespace();
                self.skip_comment();
                break;
            }
        }
    }

    fn skip_multi_line_comment(&mut self) {
        let mut depth = 0;

        loop {
            if self.is(0, '/') && self.is(1, '*') {
                depth += 1;
            } else if self.is(0, '*') && self.is(1, '/') {
                depth -= 1;
            }

            if self.is_at_end() {
                if depth != 0 {
                    self.make_error_token(
                        "Unterminated multi-line comment. Expected '*/'".to_string()
                    );
                    return;
                }
                break;
            }

            if self.is(0, '\n') {
                self.line += 1;
            }

            self.advance();
        }
    }
}
