use crate::compiler::{
    ds::value::Value,
    error_handler::{ CompileError, ReportedError },
    ir::ast::{
        expr::{ ExprBuilder, FnCallExpr, IdentifierExpr, LiteralExpr },
        stmt::ExprStmt,
        AstArena,
    },
};

use super::{ precedence::Precedence, Parser, TokenType::* };

type ReturnType = Result<(), CompileError>;

impl<'a> Parser<'a> {
    pub fn number<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(&self.source);

        let value = lexeme.parse_number();

        match value {
            Some(val) => {
                expr_builder.emit_constant_literal(LiteralExpr::new(val, token.get_metadata()));
                Ok(())
            }
            None => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!("Could not parse number '{}'", lexeme.get_lexeme_str()),
                            self.get_previous().get_metadata().into()
                        )
                    )
                );
            }
        }
    }

    pub fn identifier<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        match self.get_current().get_ttype() {
            TokenLeftParen => self.fn_call(expr_builder, arena)?,
            _ => self.ident_lookup(expr_builder, arena)?,
        }

        Ok(())
    }

    pub fn string<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        if self.is_at_end() {
            return Err(
                CompileError::new(
                    ReportedError::new(
                        format!(
                            "Unexpected token: {}",
                            self.get_previous().get_lexeme(self.source).get_lexeme_str()
                        ),
                        self.get_previous().get_metadata().into()
                    )
                )
            );
        } else {
            self.consume(
                TokenString,
                format!(
                    "Unexpected token: {}",
                    self.get_previous().get_lexeme(self.source).get_lexeme_str()
                ).as_str()
            )?;
        }

        let token = self.get_previous();
        expr_builder.emit_constant_literal(
            LiteralExpr::new(
                Value::String(token.get_lexeme(&self.source).take_lexeme_rc()),
                token.get_metadata()
            )
        );

        self.consume(TokenDoubleQuote, "Expected '\"' after string")?;

        Ok(())
    }

    pub fn literal<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let token = self.get_previous();

        match token.get_ttype() {
            TokenFalse =>
                expr_builder.emit_constant_literal(
                    LiteralExpr::new(Value::Bool(false), token.get_metadata())
                ),
            TokenTrue =>
                expr_builder.emit_constant_literal(
                    LiteralExpr::new(Value::Bool(true), token.get_metadata())
                ),
            _ => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Unexpected literal: '{}'",
                                token.get_lexeme(&self.source).get_lexeme_str()
                            ),
                            token.get_metadata().into()
                        )
                    )
                );
            }
        }

        Ok(())
    }

    pub fn unary<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let unary_token = self.get_previous().clone();

        self.parse_precedence(Precedence::PrecUnary.get_next(), expr_builder, arena)?;

        let unary_op = match unary_token.get_ttype().parse_unary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Token '{}' is not a valid unary operator",
                                unary_token.get_lexeme(&self.source).get_lexeme_str()
                            ),
                            unary_token.get_metadata().into()
                        )
                    )
                );
            }
        };

        expr_builder.emit_unary_op(unary_op, unary_token.get_metadata())
    }

    pub fn binary<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let binary_token = self.get_previous().clone();

        self.parse_precedence(
            self.get_parse_rule(&binary_token.get_ttype()).get_precedence().get_next(),
            expr_builder,
            arena
        )?;

        let binary_op = match binary_token.get_ttype().parse_binary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Token '{}' is not a valid binary operator",
                                binary_token.get_lexeme(&self.source).get_lexeme_str()
                            ),
                            binary_token.get_metadata().into()
                        )
                    )
                );
            }
        };

        expr_builder.emit_binary_op(binary_op, binary_token.get_metadata(), arena)
    }

    pub fn grouping<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let left_paren_metadata = self.get_previous().get_metadata();

        self.parse_precedence(Precedence::PrecAssignment.get_next(), expr_builder, arena)?;

        expr_builder.emit_grouping(left_paren_metadata)?;

        self.consume(TokenRightParen, "Expect ')' after expression")?;

        Ok(())
    }

    pub(super) fn fn_call<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(&self.source);
        let metadata = token.get_metadata();

        self.advance();

        let mut fn_args = vec![];
        while !self.is_at_expr_end() {
            if self.get_current().get_ttype().is(&TokenRightParen) {
                break;
            }
            let arg = self.expression(Precedence::PrecAssignment.get_next(), arena)?;

            fn_args.push(ExprStmt::new(arg));

            if !self.get_current().get_ttype().is(&TokenRightParen) {
                self.consume(
                    TokenComma,
                    format!(
                        "Expected ',' between call arguments, but received: '{}'",
                        self.get_current().get_lexeme(&self.source).get_lexeme_str()
                    ).as_str()
                )?;
            }
        }

        self.consume(
            TokenRightParen,
            format!(
                "Expected ')' in function call but got: '{}'",
                self.get_current().get_lexeme(&self.source).get_lexeme_str()
            ).as_str()
        )?;

        expr_builder.emit_fn_call(
            FnCallExpr::new(lexeme.take_lexeme_rc(), metadata, fn_args),
            arena
        );

        Ok(())
    }

    pub(super) fn ident_lookup<'b>(
        &mut self,
        expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> ReturnType {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(&self.source);

        expr_builder.emit_ident_lookup(
            IdentifierExpr::new(lexeme.take_lexeme_rc(), token.get_metadata())
        );

        Ok(())
    }

    pub(super) fn error(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let error_token = self.get_previous();
        let msg = error_token.get_message();

        if let Some(msg) = msg {
            Err(
                CompileError::new(
                    ReportedError::new(msg.to_string(), error_token.get_metadata().into())
                )
            )
        } else {
            Err(
                CompileError::new(
                    ReportedError::new(
                        format!(
                            "Unexpected token: {}",
                            error_token.get_lexeme(&self.source).get_lexeme_str()
                        ),
                        error_token.get_metadata().into()
                    )
                )
            )
        }
    }
}
