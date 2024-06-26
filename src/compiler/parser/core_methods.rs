use crate::{
    compiler::{
        ds::value::ValueType,
        error_handler::{ CompileError, ReportedError },
        ir::ast::{ expr::{ Expr, ExprBuilder }, stmt::FunctionArgument, AstArena },
        error_handler::SrcCharsRange,
    },
    macros::merge_chars_range,
};

use super::{ precedence::Precedence, token::TokenMetadata, Parser, TokenType::* };

impl<'a> Parser<'a> {
    // pub(super) fn resolve_type(&mut self) -> Result<Option<ValueType>, >

    pub(super) fn resolve_type(&mut self) -> Result<Option<ValueType>, Vec<TokenMetadata>> {
        // loop {
        //     match self.get_current().get_ttype() {
        //         TokenReference => {
        //             if let Some(value_type) = &mut value_type {
        //                 value_type.append_type();
        //             } else {
        //                 value_type = ValueType::Ref(Box::new(x));
        //             }
        //         }
        //         TokenMutableReference => {

        //         }
        //     }
        // }

        let resolved_type = match self.get_current().get_ttype() {
            TokenIdentifier => {
                self.advance();
                let type_lexeme = self.get_previous().get_lexeme(&&self.source);
                match type_lexeme.get_lexeme_str() {
                    "i32" => Ok(Some(ValueType::Int32)),
                    "bool" => Ok(Some(ValueType::Bool)),
                    _ => Ok(None), // This should make a custom type
                }
            }
            _ => Err(vec![self.get_current().get_metadata()]),
        };

        resolved_type
    }

    pub(super) fn expression<'b>(
        &mut self,
        precedence: Precedence,
        arena: &'b AstArena<'b>
    ) -> Result<Expr<'b>, CompileError> {
        let mut expr_builder = ExprBuilder::new(arena);

        self.parse_precedence(precedence, &mut expr_builder, arena)?;

        Ok(expr_builder.get_built_expr())
    }

    pub(super) fn parse_precedence<'b>(
        &mut self,
        precedence: Precedence,
        mut expr_builder: &mut ExprBuilder<'b>,
        arena: &'b AstArena<'b>
    ) -> Result<(), CompileError> {
        self.advance();

        let parse_rule = self.get_parse_rule(self.get_previous().get_ttype());

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self, &mut expr_builder, arena)?;

            loop {
                let current_ttype = self.get_current().get_ttype();
                let current_precedence = self.get_parse_rule(current_ttype).get_precedence();

                if (precedence as usize) > (*current_precedence as usize) {
                    break;
                }

                if self.get_previous().get_line() < self.get_current().get_line() {
                    break;
                }

                self.advance();

                let infix_rule = self.get_parse_rule(self.get_previous().get_ttype()).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self, &mut expr_builder, arena)?;
                }
            }
        } else {
            return Err(
                CompileError::new(
                    ReportedError::new(
                        format!(
                            "Unexpected token: '{}' (no prefix rule)",
                            self.get_previous().get_lexeme(&self.source).get_lexeme_str()
                        ),
                        self.get_previous().get_metadata().into()
                    )
                )
            );
        }

        Ok(())
    }

    pub(super) fn resolve_function_args(&mut self) -> Result<Vec<FunctionArgument>, CompileError> {
        let mut args = vec![];

        self.consume(TokenLeftParen, "Expected '(' after function identifier")?;

        while !self.is_at_expr_end() {
            if self.get_current().get_ttype().is(&TokenComma) {
                self.advance();
            }
            if self.get_current().get_ttype().is(&TokenRightParen) {
                break;
            }

            self.consume(
                TokenIdentifier,
                format!(
                    "Expected identifer but got '{}' Syntax: <identifier> <type>",
                    self.get_current().get_lexeme(&self.source).get_lexeme_str()
                ).as_str()
            )?;

            let ident_lexeme = self.get_previous().get_lexeme(&&self.source);
            let metadata = self.get_previous().get_metadata();

            let is_mutable = match self.get_current().get_ttype().is(&TokenMutable) {
                true => {
                    self.advance();
                    true
                }
                false => false,
            };

            let arg_type = match self.resolve_type() {
                Ok(arg_type) =>
                    match arg_type {
                        Some(arg_type) => arg_type,
                        None => panic!("Custom types not yet supported"),
                    }
                Err(error_tokens) => {
                    return Err(
                        CompileError::new(
                            ReportedError::new(
                                "Invalid type".to_string(),
                                merge_chars_range!(
                                    error_tokens
                                        .iter()
                                        .map(|m| (*m).into())
                                        .collect()
                                )
                            )
                        )
                    );
                }
            };

            args.push(FunctionArgument {
                is_mutable,
                name: ident_lexeme.take_lexeme_rc(),
                value_type: arg_type,
                metadata,
            });
        }

        self.consume(TokenRightParen, "Expected a closing ')' after function arguments")?;

        Ok(args)
    }

    pub(super) fn resolve_function_return_type(
        &mut self
    ) -> Result<Option<ValueType>, CompileError> {
        let return_type = match self.get_current().get_ttype() {
            TokenLeftCurlyBrace => {
                return Ok(None);
            }
            TokenIdentifier =>
                match self.resolve_type() {
                    Ok(return_type) => return_type,
                    Err(error_tokens) => {
                        return Err(
                            CompileError::new(
                                ReportedError::new(
                                    "Invalid function return type".to_string(),
                                    merge_chars_range!(
                                        error_tokens
                                            .iter()
                                            .map(|m| (*m).into())
                                            .collect()
                                    )
                                )
                            )
                        );
                    }
                }
            _ => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Expected function return type, but got: {}",
                                self.get_current().get_lexeme(&self.source).get_lexeme_str()
                            ),
                            self.get_current().get_metadata().into()
                        )
                    )
                );
            }
        };

        Ok(return_type)
    }

    pub(super) fn synchronize(&mut self) {
        self.exit_panic_mode();

        while !self.is_at_end() {
            if self.get_previous().get_ttype() == &TokenSemicolon {
                return;
            }
            if self.get_previous().get_line() < self.get_current().get_line() {
                // This might need to change, as soon as blocks are implemented
                return;
            }

            match self.get_current().get_ttype() {
                TokenError => {
                    self.advance();
                }
                _ => self.advance(),
            }
        }
    }
}
