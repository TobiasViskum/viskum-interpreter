use crate::value::ValueType;

use super::{ precedence::Precedence, token::token_type::TokenType, Parser };

impl<'a> Parser<'a> {
    pub(super) fn statement(&mut self) {
        match self.get_current().get_ttype() {
            TokenType::TokenInt32 => {
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
            TokenType::TokenMutable => {
                self.advance();
                if self.get_current().get_ttype() == &TokenType::TokenFunction {
                    // Function tokens isn't supported in lexer yet
                    // self.function_definition();
                } else {
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
            _ => self.expression_statement(),
        }
    }

    pub(super) fn variable_definition(&mut self) {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        let (found_type, search_index) = self.resolve_type();

        let (is_mutable, _) = self.resolve_mutabiliy(search_index - 1);

        self.advance();
        self.expression();
        self.consume_expr_end();
        let result = self.ast_generator.emit_variable_definition(lexeme, found_type, is_mutable);

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }

        self.ast_generator.push_stmt();
    }

    pub(super) fn expression_statement(&mut self) {
        self.expression();
        self.consume_expr_end();

        self.ast_generator.push_expr()
    }

    pub(super) fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment);
    }

    pub(super) fn parse_precedence(&mut self, precedence: Precedence) {
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
                self.advance();

                let infix_rule = self.get_parse_rule(self.get_previous().get_ttype()).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self);
                }
            }
        } else {
            if self.get_previous().get_ttype() == &TokenType::TokenEOF {
                self.report_compile_error(
                    "Unexpected end of file (no prefix rule)".to_string(),
                    vec![self.get_previous().get_metadata()]
                )
            } else {
                self.report_compile_error(
                    format!(
                        "Unexpected token: '{}' (no prefix rule)",
                        self.get_previous().get_lexeme(self.source)
                    ),
                    vec![self.get_previous().get_metadata()]
                )
            }
        }
    }

    pub(super) fn resolve_type(&mut self) -> (Option<ValueType>, isize) {
        let /*mut*/ serch_index: isize = -2;
        loop {
            match self.peek(serch_index) {
                Some(token) => {
                    if token.get_ttype() == &TokenType::TokenInt32 {
                        return (Some(ValueType::Int32), serch_index);
                    }
                }
                None => {
                    return (None, serch_index);
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
