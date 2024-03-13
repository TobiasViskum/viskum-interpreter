use super::{ precedence::Precedence, token::token_type::TokenType, Parser };

impl<'a> Parser<'a> {
    pub(super) fn statement(&mut self) {
        self.expression_statement()
    }

    pub(super) fn expression_statement(&mut self) {
        self.expression();
        self.consume_expr_end();

        self.ast_generator.push_expr_stmt()
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
            todo!("No prefix rule found for token: {:?}", self.get_previous())
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
                        current.clone()
                    );
                    self.advance();
                }
                _ => self.advance(),
            }
        }
    }
}
