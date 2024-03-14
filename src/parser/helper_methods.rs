use crate::{ operations::Op, parser::token::{ token_type::TokenType, Token } };

use super::{ parse_rule::{ ParseRule, PARSE_RULES }, token::TokenMetadata, Parser };

impl<'a> Parser<'a> {
    pub(super) fn advance(&mut self) {
        if let Some(token) = self.current.take() {
            self.previous_tokens.push(token);
        }
        self.current = Some(self.lexer.scan_token());
    }

    pub(super) fn get_current(&self) -> &Token {
        self.current.as_ref().unwrap()
    }

    pub(super) fn get_previous(&self) -> &Token {
        self.peek(-1).unwrap()
    }

    pub(super) fn consume(&mut self, ttype: TokenType) {
        if self.get_current().get_ttype() == &ttype {
            self.advance();
        } else {
            if self.panic_mode {
                return;
            }

            if self.get_current().get_ttype() == &TokenType::TokenError {
                let msg = self.get_current().get_message().unwrap().to_string();
                self.report_compile_error(msg, vec![self.get_current().get_metadata()]);
                self.advance();
            } else {
                self.report_compile_error(
                    format!(
                        "Expected token of type {:?}, found {:?}",
                        ttype,
                        self.get_current().get_ttype()
                    ),
                    vec![self.get_current().get_metadata()]
                );
            }
        }
    }

    pub(super) fn consume_expr_end(&mut self) {
        match self.get_current().get_ttype() {
            TokenType::TokenSemicolon => self.advance(),
            TokenType::TokenEOF => {}
            _ => {
                let prev_line = self.get_previous().get_line();

                if self.get_current().get_line() <= prev_line {
                    if self.panic_mode {
                        return;
                    }

                    if self.get_current().get_ttype() == &TokenType::TokenError {
                        let msg = self.get_current().get_message().unwrap().to_string();
                        self.report_compile_error(msg, vec![self.get_current().get_metadata()]);
                        self.advance();
                    } else {
                        self.report_compile_error(
                            format!(
                                "Unexpected end of expression. Expected new line or ';' but got {}",
                                self.get_current().get_lexeme(self.source)
                            ),
                            vec![self.get_current().get_metadata()]
                        );
                    }
                }
            }
        }
    }

    pub(super) fn peek(&self, offset: isize) -> Option<&Token> {
        if offset == 0 {
            Some(self.get_current())
        } else {
            self.previous_tokens.get(((self.previous_tokens.len() as isize) + offset) as usize)
        }
    }

    pub(super) fn is_at_end(&self) -> bool {
        self.get_current().get_ttype() == &TokenType::TokenEOF
    }

    pub(super) fn get_parse_rule(&self, ttype: &TokenType) -> &ParseRule {
        PARSE_RULES.get(*ttype as usize).unwrap()
    }

    pub(super) fn enter_panic_mode(&mut self) {
        self.panic_mode = true;
        self.ast_generator.enter_panic_mode()
    }

    pub(super) fn exit_panic_mode(&mut self) {
        self.panic_mode = false;
        self.ast_generator.exit_panic_mode()
    }

    pub(super) fn report_compile_error(&mut self, message: String, token: Vec<TokenMetadata>) {
        self.error_handler.report_compile_error(message, token);
        self.enter_panic_mode();
    }
}
