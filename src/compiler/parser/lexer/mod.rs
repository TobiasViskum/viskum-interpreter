mod core_methods;
mod helper_methods;

use super::{ token::Token, TokenType::* };

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

    pub fn get_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }

        self.tokens
    }

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
            '(' => self.make_token(TokenLeftParen),
            ')' => self.make_token(TokenRightParen),
            '{' => self.make_token(TokenLeftCurlyBrace),
            '}' => self.make_token(TokenRightCurlyBrace),
            '[' => self.make_token(TokenLeftSquareBrace),
            ']' => self.make_token(TokenRightSquareBrace),
            '+' => self.make_token(TokenPlus),
            '-' => self.make_token(TokenMinus),
            '*' => self.make_token(TokenStar),
            '/' => self.make_token(TokenSlash),
            ';' => self.make_token(TokenSemicolon),
            ',' => self.make_token(TokenComma),
            '&' => {
                if self.is_keyword(1, 3, "mut") {
                    self.current += 3;
                    self.make_token(TokenMutableReference)
                } else {
                    self.make_token(TokenReference)
                }
            }
            '"' => {
                self.make_token(TokenDoubleQuote);
                self.start = self.current;
                self.advance();
                self.string()
            }
            '<' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenLessEqual)
                } else {
                    self.make_token(TokenLess)
                }
            }
            '>' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenGreaterEqual)
                } else {
                    self.make_token(TokenGreater)
                }
            }
            '=' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenEqualEqual)
                } else {
                    self.make_token(TokenAssign)
                }
            }
            ':' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenDefine)
                } else {
                    self.make_error_token(format!("Unexpected character: {}", c))
                }
            }
            '!' => {
                if self.is(0, '=') {
                    self.advance();
                    self.make_token(TokenBangEqual)
                } else {
                    self.make_token(TokenBang)
                }
            }
            c => {
                if Self::is_digit(Some(c)) {
                    while !self.is_at_end() && Self::is_digit(self.peek(0)) {
                        self.advance();
                    }

                    self.make_token(TokenNumber)
                } else if Self::is_alphabetic(Some(c)) {
                    while
                        !self.is_at_end() &&
                        (Self::is_alphabetic(self.peek(0)) || Self::is_digit(self.peek(0)))
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
