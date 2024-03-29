use crate::{ ast::AstIdentifier, operations::Op, value::ValueType };

use super::{
    precedence::Precedence,
    token::{ token_type::TokenType, Token, TokenMetadata },
    Parser,
};

impl<'a> Parser<'a> {
    pub(super) fn statement(&mut self) {
        match self.get_current().get_ttype() {
            TokenType::TokenIdentifier => {
                let next = self.get_next();
                if next.is_some() && matches!(next.unwrap().get_ttype(), &TokenType::TokenDefine) {
                    self.advance();
                    self.variable_definition();
                } else if
                    next.is_some() &&
                    matches!(next.unwrap().get_ttype(), &TokenType::TokenAssign)
                {
                    self.advance();
                    self.variable_assignment()
                } else {
                    self.expression_statement();
                }
            }
            TokenType::TokenInt32 | TokenType::TokenBool => {
                while !self.is_at_end() && !self.is_at_expr_end() {
                    let next = self.get_next();
                    if let Some(next) = next {
                        if matches!(next.get_ttype(), &TokenType::TokenIdentifier) {
                            break;
                        }
                    } else {
                        break;
                    }

                    self.advance();
                }

                self.advance();
                self.advance();
                self.variable_definition();
            }
            TokenType::TokenMutable => {
                self.advance();

                match self.get_current().get_ttype() {
                    TokenType::TokenFunction => {
                        // Function tokens isn't supported in lexer yet
                        // self.function_definition();
                    }
                    TokenType::TokenInt32 | TokenType::TokenBool => {
                        while !self.is_at_end() && !self.is_at_expr_end() {
                            let next = self.get_next();
                            if let Some(next) = next {
                                if matches!(next.get_ttype(), &TokenType::TokenIdentifier) {
                                    break;
                                }
                            } else {
                                break;
                            }

                            self.advance();
                        }

                        self.advance();
                        self.advance();
                        self.variable_definition();
                    }
                    TokenType::TokenIdentifier => {
                        let next = self.get_next();

                        if
                            next.is_some() &&
                            matches!(next.unwrap().get_ttype(), &TokenType::TokenDefine)
                        {
                            self.advance();
                            self.variable_definition();
                        } else {
                            self.expression_statement();
                        }
                    }
                    _ => {
                        while
                            !self.is_at_end() &&
                            !matches!(self.get_current().get_ttype(), &TokenType::TokenDefine)
                        {
                            self.advance();
                        }

                        if self.get_current().get_ttype() == &TokenType::TokenDefine {
                            self.variable_definition();
                        }
                    }
                }
            }
            TokenType::TokenLeftCurlyBrace => {
                self.start_scope();
                self.advance();
                while
                    !self.is_at_end() &&
                    !matches!(self.get_current().get_ttype(), &TokenType::TokenRightCurlyBrace)
                {
                    self.statement();
                }
                self.consume(TokenType::TokenRightCurlyBrace, "Expected '}' at the end of block");
                self.end_scope()
            }
            _ => { self.expression_statement() }
        }
    }

    pub(super) fn variable_assignment(&mut self) {
        let (lexeme, token_metadata) = {
            let token = self.get_previous();
            (token.get_lexeme(self.source), token.get_metadata())
        };

        self.ast_generator.emit_identifier_lookup(AstIdentifier::new(lexeme, token_metadata));

        if !self.is_at_expr_end() {
            if
                self.consume(
                    TokenType::TokenAssign,
                    format!(
                        "Expected '=' in variable assignment but got '{}'",
                        self.get_current().get_lexeme(self.source)
                    ).as_str()
                )
            {
                if !self.is_at_expr_end() {
                    self.parse_precedence(Precedence::PrecAssignment.get_next(), None);
                    self.consume_expr_end();
                } else {
                    self.report_compile_error(
                        "Missing right hand side of variable assignment".to_string(),
                        vec![self.get_previous().get_metadata()]
                    );
                }
            }
        }

        let result = self.ast_generator.emit_variable_assignment(token_metadata);

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }

        // match self.ast_generator.push_stmt() {
        //     Ok(_) => {}
        //     Err((message, token)) => {
        //         self.report_compile_error(message, token);
        //     }
        // }
    }

    pub(super) fn variable_definition(&mut self) {
        let token = self.get_previous().clone();

        let lexeme = token.get_lexeme(self.source);

        let (found_type, type_token, search_index) = self.resolve_type();

        let type_or_identifier_token = if let Some(type_token) = type_token {
            type_token
        } else {
            token.clone()
        };

        let (is_mutable, _) = if found_type.is_some() {
            self.resolve_mutabiliy(search_index - 1)
        } else {
            self.resolve_mutabiliy(-2)
        };

        if !self.is_at_expr_end() {
            if
                self.consume(
                    TokenType::TokenDefine,
                    format!(
                        "Expected ':=' in variable definition but got '{}'",
                        self.get_current().get_lexeme(self.source)
                    ).as_str()
                )
            {
                if !self.is_at_expr_end() {
                    self.parse_precedence(Precedence::PrecAssignment.get_next(), None);
                    self.consume_expr_end();
                } else {
                    self.report_compile_error(
                        "Missing right hand side of variable definition".to_string(),
                        vec![self.get_previous().get_metadata()]
                    );
                }
            }
        }

        let result = self.ast_generator.emit_variable_definition(
            lexeme,
            token,
            found_type,
            is_mutable,
            type_or_identifier_token
        );

        if let Err((message, token_vec)) = result {
            self.report_compile_error(message, token_vec);
        }

        // match self.ast_generator.push_stmt() {
        //     Ok(_) => {}
        //     Err((message, token)) => {
        //         self.report_compile_error(message, token);
        //     }
        // }
    }

    pub(super) fn expression_statement(&mut self) {
        self.expression();
        self.consume_expr_end();

        match self.ast_generator.push_expr() {
            Ok(_) => {}
            Err((message, token)) => {
                self.report_compile_error(message, token);
                self.synchronize()
            }
        }
    }

    pub(super) fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment, None);
    }

    pub(super) fn parse_precedence(&mut self, precedence: Precedence, msg: Option<&str>) {
        self.advance();

        let parse_rule = self.get_parse_rule(self.get_previous().get_ttype());

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self);

            loop {
                let current_ttype = self.get_current().get_ttype();
                let current_precedence = self.get_parse_rule(current_ttype).get_precedence();

                if (*current_precedence as usize) < (precedence as usize) {
                    break;
                }

                if self.get_previous().get_line() < self.get_current().get_line() {
                    break;
                }

                self.advance();

                let infix_rule = self.get_parse_rule(self.get_previous().get_ttype()).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self);
                }
            }
        } else {
            // if let Some(msg) = msg {
            //     let mut msg = msg.to_string();
            //     let report_token = if self.get_previous().get_ttype() == &TokenType::TokenEOF {
            //         msg = format!("{}unexpected end of file", msg);
            //         self.peek(-2).unwrap()
            //     } else {
            //         self.get_previous()
            //     };

            //     self.report_compile_error(msg.to_string(), vec![report_token.get_metadata()]);
            // } else {
            //     if self.get_previous().get_ttype() == &TokenType::TokenEOF {
            //         self.report_compile_error(
            //             "Unexpected end of file (no prefix rule)".to_string(),
            //             vec![self.get_previous().get_metadata()]
            //         )
            //     } else {
            //         self.report_compile_error(
            //             format!(
            //                 "Unexpected token: '{}' (no prefix rule)",
            //                 self.get_previous().get_lexeme(self.source)
            //             ),
            //             vec![self.get_previous().get_metadata()]
            //         )
            //     }
            // }
        }
    }

    pub(super) fn resolve_type(&mut self) -> (Option<ValueType>, Option<Token>, isize) {
        let mut search_index: isize = -2;
        let line = self.get_previous().get_line();

        loop {
            match self.peek(search_index) {
                Some(token) => {
                    if
                        token.get_line() < line ||
                        matches!(token.get_ttype(), TokenType::TokenEOF | TokenType::TokenSemicolon)
                    {
                        return (None, None, search_index);
                    }

                    if token.get_ttype() == &TokenType::TokenInt32 {
                        return (Some(ValueType::Int32), Some(token.clone()), search_index);
                    } else if token.get_ttype() == &TokenType::TokenBool {
                        return (Some(ValueType::Bool), Some(token.clone()), search_index);
                    } else {
                        search_index -= 1;
                    }
                }
                None => {
                    return (None, None, search_index);
                }
            }
        }
    }

    pub(super) fn resolve_mutabiliy(&mut self, start_search_index: isize) -> (bool, isize) {
        match self.peek(start_search_index) {
            Some(token) => (token.get_ttype() == &TokenType::TokenMutable, start_search_index),
            None => (false, start_search_index),
        }
    }

    pub(super) fn synchronize(&mut self) {
        self.exit_panic_mode();

        while !self.is_at_end() {
            if self.get_previous().get_ttype() == &TokenType::TokenSemicolon {
                return;
            }
            if self.get_previous().get_line() < self.get_current().get_line() {
                // This might need to change, as soon as blocks are implemented
                return;
            }

            match self.get_current().get_ttype() {
                TokenType::TokenError => {
                    let current = self.get_current();
                    self.report_compile_error(
                        current.get_message().unwrap().to_string(),
                        vec![current.get_metadata()]
                    );
                    self.advance();
                }
                _ => self.advance(),
            }
        }
    }
}
