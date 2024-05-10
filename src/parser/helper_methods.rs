use crate::{
    ast::{ stmt::FunctionArgument, Ast },
    error_handler::CompileError,
    parser::token::{ token_type::TokenType, Token },
    value::ValueType,
};

use super::{ parse_rule::{ ParseRule, PARSE_RULES }, token::TokenMetadata, Parser };

impl<'a> Parser<'a> {
    pub(super) fn advance(&mut self) {
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
                Err(CompileError::new(msg, vec![metadata]))
            } else {
                self.advance();
                Err(CompileError::new(msg.to_string(), vec![self.get_current().get_metadata()]))
            }
        }
    }

    pub(super) fn is_at_expr_end(&mut self) -> bool {
        let result = match self.get_current().get_ttype() {
            TokenType::TokenSemicolon | TokenType::TokenEOF => true,
            _ => {
                let prev_line = match self.peek(-1) {
                    Some(line) => line.get_line(),
                    None => {
                        return false;
                    }
                };

                if self.get_current().get_line() < prev_line {
                    if self.get_current().get_ttype() == &TokenType::TokenError {
                        let msg = self.get_current().get_message().unwrap().to_string();
                        self.report_compile_error(msg, vec![self.get_current().get_metadata()]);
                    }
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
                        return Err(CompileError::new(msg, vec![self.get_current().get_metadata()]));
                    } else {
                        return Err(
                            CompileError::new(
                                format!(
                                    "Unexpected end of expression. Expected new line or ';' but got {}",
                                    self.get_current().get_lexeme(self.source)
                                ),
                                vec![self.get_current().get_metadata()]
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
        PARSE_RULES.get(*ttype as usize).unwrap()
    }

    pub(super) fn enter_panic_mode(&mut self) {
        self.panic_mode = true;
    }

    pub(super) fn exit_panic_mode(&mut self) {
        self.panic_mode = false;
    }

    pub(super) fn report_compile_error(&mut self, message: String, token: Vec<TokenMetadata>) {
        self.error_handler.report_compile_error(message, token);
        self.enter_panic_mode();
    }
}
