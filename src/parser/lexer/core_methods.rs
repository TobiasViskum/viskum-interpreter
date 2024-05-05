use crate::parser::token::{ Token, token_type::TokenType };

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn make_eof_token(&mut self) -> Option<Token> {
        self.start = self.current;
        self.make_token(TokenType::TokenEOF)
    }

    pub(super) fn make_token(&mut self, ttype: TokenType) -> Option<Token> {
        Some(Token::new(ttype, self.start, self.current - self.start, self.line))
    }

    pub(super) fn make_identifier_token(&mut self) -> Option<Token> {
        let ttype = match self.get_character(self.start) {
            //'b' => self.check_keyword(1, 3, "ool", TokenType::TokenBool),
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
            //'i' => self.check_keyword(1, 2, "32", TokenType::TokenInt32),
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
            match self.skip_multi_line_comment() {
                Some(token) => {
                    return Some(token);
                }
                None => {}
            }
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

    fn skip_multi_line_comment(&mut self) -> Option<Token> {
        let mut depth = 0;

        loop {
            if self.is_at_end() {
                if depth != 0 {
                    return self.make_error_token(
                        "Unterminated multi-line comment. Expected '*/'".to_string()
                    );
                }
                break;
            }

            if self.is(0, '\n') {
                self.line += 1;
            }

            if self.is(0, '/') && self.is(1, '*') {
                depth += 1;
            } else if self.is(0, '*') && self.is(1, '/') {
                depth -= 1;
            }

            if depth == 0 {
                self.advance();
                self.advance();
                break;
            } else {
                self.advance();
            }
        }

        None
    }
}
