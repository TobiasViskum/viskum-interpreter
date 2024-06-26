use crate::compiler::error_handler::{ CompileError, ReportedError };

use super::{ parse_rule::ParseRule, token::Token, Parser, TokenType };

impl<'a> Parser<'a> {
    pub(super) fn advance(&mut self) {
        if self.current == self.tokens.len() - 1 {
            return;
        }
        self.current += 1;
        /*
        if let Some(prev_next) = self.next.take() {
            if let Some(prev_current) = self.current.take() {
                self.previous_tokens.push(prev_current);
            }
            self.current = Some(prev_next);
        }
        self.next = self.lexer.scan_token();
        */
    }

    pub(super) fn get_current(&self) -> &Token {
        // self.current.as_ref().unwrap()

        self.tokens.get(self.current).unwrap()
    }

    pub(super) fn get_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    pub(super) fn get_previous(&self) -> &Token {
        self.peek(-1).unwrap()
    }

    pub(super) fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<(), CompileError> {
        if self.get_current().get_ttype() == &ttype {
            self.advance();
            Ok(())
        } else {
            if self.get_current().get_ttype() == &TokenType::TokenError {
                let msg = self.get_current().get_message().unwrap().to_string();
                let metadata = self.get_current().get_metadata();
                self.advance();
                Err(CompileError::new(ReportedError::new(msg, metadata.into())))
            } else {
                self.advance();
                Err(
                    CompileError::new(
                        ReportedError::new(
                            msg.to_string(),
                            self.get_current().get_metadata().into()
                        )
                    )
                )
            }
        }
    }

    pub(super) fn is_at_expr_end(&mut self) -> bool {
        if self.is_at_end() {
            return true;
        }
        let result = match self.get_current().get_ttype() {
            TokenType::TokenSemicolon | TokenType::TokenEOF => true,
            _ => {
                let prev_line = self.get_previous().get_line();

                if self.get_current().get_line() > prev_line {
                    true
                } else {
                    false
                }
            }
        };

        result
    }

    pub(super) fn consume_expr_end(&mut self) -> Result<(), CompileError> {
        match self.get_current().get_ttype() {
            TokenType::TokenSemicolon => {
                self.advance();
                return Ok(());
            }
            TokenType::TokenEOF => {
                return Ok(());
            }
            _ => {
                let prev_line = self.get_previous().get_line();

                if self.get_current().get_line() <= prev_line {
                    if self.get_current().get_ttype() == &TokenType::TokenError {
                        let msg = self.get_current().get_message().unwrap().to_string();
                        return Err(
                            CompileError::new(
                                ReportedError::new(msg, self.get_current().get_metadata().into())
                            )
                        );
                    } else {
                        return Err(
                            CompileError::new(
                                ReportedError::new(
                                    format!(
                                        "Unexpected end of expression. Expected new line or ';' but got {}",
                                        self.get_current().get_lexeme(&self.source).get_lexeme_str()
                                    ),
                                    self.get_current().get_metadata().into()
                                )
                            )
                        );
                    }
                }
            }
        }

        Ok(())
    }

    pub(super) fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get(((self.current as isize) + offset) as usize)

        /*
        if offset == 0 {
            Some(self.get_current())
        } else {
            self.previous_tokens.get(((self.previous_tokens.len() as isize) + offset) as usize)
        }
        */
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.get_current().get_ttype() == &TokenType::TokenEOF
    }

    pub(super) fn get_parse_rule(&self, ttype: &TokenType) -> &ParseRule {
        self.parse_rules.get(*ttype as usize).unwrap()
    }

    pub(super) fn enter_panic_mode(&mut self) {
        self.panic_mode = true;
    }

    pub(super) fn exit_panic_mode(&mut self) {
        self.panic_mode = false;
    }

    pub(super) fn report_compile_error(&mut self, err: CompileError) {
        self.error_handler.report_compile_error(err);
        self.enter_panic_mode();
    }

    pub(super) fn is_ttype_in_stmt(&self, token_type: TokenType) -> bool {
        let mut i: usize = 0;

        let start_line = self.get_current().get_line();

        loop {
            i += 1;

            match self.peek(i as isize) {
                Some(token) => {
                    if token.get_ttype().is(&TokenType::TokenSemicolon) {
                        return false;
                    } else if token.get_line() > start_line {
                        return false;
                    } else if token.get_ttype() == &token_type {
                        return true;
                    } else {
                        continue;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
}
