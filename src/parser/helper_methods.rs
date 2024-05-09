use crate::{
    ast::{ stmt::FunctionArgument, Ast },
    error_handler::CompileError,
    parser::token::{ token_type::TokenType, Token },
    value::ValueType,
};

use super::{ parse_rule::{ ParseRule, PARSE_RULES }, token::TokenMetadata, Parser };

impl<'a> Parser<'a> {
    pub(super) fn advance(&mut self) {
        if let Some(prev_next) = self.next.take() {
            if let Some(prev_current) = self.current.take() {
                self.previous_tokens.push(prev_current);
            }
            self.current = Some(prev_next);
        }
        self.next = self.lexer.scan_token();
    }

    pub(super) fn get_current(&self) -> &Token {
        self.current.as_ref().unwrap()
    }

    pub(super) fn get_next(&self) -> Option<&Token> {
        self.next.as_ref()
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
                Err(CompileError::new(msg.to_string(), vec![self.get_current().get_metadata()]))
            }
        }
    }

    pub(super) fn is_next_at_expr_end(&mut self) -> bool {
        let result = match self.get_next() {
            Some(token) =>
                match token.get_ttype() {
                    TokenType::TokenSemicolon | TokenType::TokenEOF => true,
                    _ => {
                        let prev_line = self.get_current().get_line();

                        if token.get_line() <= prev_line {
                            if token.get_ttype() == &TokenType::TokenError {
                                let msg = token.get_message().unwrap().to_string();
                                self.report_compile_error(msg, vec![token.get_metadata()]);
                            }
                            false
                        } else {
                            true
                        }
                    }
                }
            None => false,
        };

        result
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

                if self.get_current().get_line() <= prev_line {
                    if self.get_current().get_ttype() == &TokenType::TokenError {
                        let msg = self.get_current().get_message().unwrap().to_string();
                        self.report_compile_error(msg, vec![self.get_current().get_metadata()]);
                    }
                    false
                } else {
                    true
                }
            }
        };

        result
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
                        match self.get_current().get_ttype() {
                            TokenType::TokenAssign => {
                                if let Some(expr) = self.ast_generator.pop_expr() {
                                    let mut token_vec: Vec<TokenMetadata> = Vec::new();
                                    expr.push_to_token_vec(&mut token_vec);

                                    let lexeme = &self.source[
                                        token_vec.last().unwrap().get_start()..token_vec
                                            .first()
                                            .unwrap()
                                            .get_start() + token_vec.first().unwrap().get_len()
                                    ]
                                        .iter()
                                        .collect::<String>();

                                    self.report_compile_error(
                                        format!("Invalid assignment target: '{}'", lexeme),
                                        token_vec
                                    );

                                    if self.is_next_at_expr_end() {
                                        self.report_compile_error(
                                            "Missing right hand side of variable assignment".to_string(),
                                            vec![self.get_current().get_metadata()]
                                        );
                                    }
                                } else {
                                    self.report_compile_error(
                                        format!(
                                            "Invalid assignment target: '{}'",
                                            self.get_previous().get_lexeme(self.source)
                                        ),
                                        vec![
                                            self.get_current().get_metadata(),
                                            self.get_previous().get_metadata()
                                        ]
                                    );
                                }
                            }
                            TokenType::TokenDefine => {
                                if let Some(expr) = self.ast_generator.pop_expr() {
                                    let mut token_vec: Vec<TokenMetadata> = Vec::new();
                                    expr.push_to_token_vec(&mut token_vec);

                                    let lexeme = &self.source[
                                        token_vec.last().unwrap().get_start()..token_vec
                                            .first()
                                            .unwrap()
                                            .get_start() + token_vec.first().unwrap().get_len()
                                    ]
                                        .iter()
                                        .collect::<String>();

                                    self.report_compile_error(
                                        format!("Invalid definition target: '{}'", lexeme),
                                        token_vec
                                    );

                                    if self.is_next_at_expr_end() {
                                        self.report_compile_error(
                                            "Missing right hand side of variable definition".to_string(),
                                            vec![self.get_current().get_metadata()]
                                        );
                                    }
                                } else {
                                    self.report_compile_error(
                                        format!(
                                            "Invalid definition target: '{}'",
                                            self.get_previous().get_lexeme(self.source)
                                        ),
                                        vec![
                                            self.get_current().get_metadata(),
                                            self.get_previous().get_metadata()
                                        ]
                                    );
                                }
                            }
                            _ => {
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
        }

        self.previous_tokens.clear()
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

    pub(super) fn start_scope(&mut self) {
        self.ast_generator.start_scope();
    }

    pub(super) fn end_scope(&mut self) {
        self.ast_generator.end_scope();
    }

    pub(super) fn start_function(
        &mut self,
        name: String,
        args: Vec<FunctionArgument>,
        return_type: Option<ValueType>
    ) {
        self.ast_generator.start_function(name, args, return_type);
    }

    pub(super) fn end_function(&mut self) {
        self.ast_generator.end_function();
    }

    pub(super) fn report_compile_error(&mut self, message: String, token: Vec<TokenMetadata>) {
        self.error_handler.report_compile_error(message, token);
        self.enter_panic_mode();
    }
}
