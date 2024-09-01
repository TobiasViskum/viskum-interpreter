use std::rc::Rc;

use crate::{
    compiler::{
        ds::value::ValueType,
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::ast::{ expr::{ Expr, ExprBuilder }, stmt::{ FnArg }, AstArena },
        ProgramSymbolTablePhase1,
    },
    macros::merge_chars_range,
};

use super::{
    parser_macros::{ current, peek, previous },
    precedence::Precedence,
    token::TokenMetadata,
    ExprMethodArgs,
    ExprMethodRetType,
    Parser,
    StmtMethodArgs,
    TokenType::*,
};

impl<'a> Parser<'a> {
    // pub(super) fn resolve_type(&mut self) -> Result<Option<ValueType>, >

    pub(super) fn check_if_type(&self, i: isize) -> bool {
        match peek!(self, i, ttype, lexeme) {
            Some((ttype, lexeme)) => {
                let lexeme_str = lexeme.take_lexeme_rc();
                match (ttype, lexeme_str.as_ref()) {
                    (TokenIdentifier, "Int") => true,
                    (TokenIdentifier, "Bool") => true,
                    (TokenIdentifier, "String") => true,
                    (TokenIdentifier, "Void") => true,
                    _ => false,
                }
            }
            None => false,
        }
    }

    pub(super) fn resolve_type(&mut self) -> Result<Option<ValueType>, Vec<TokenMetadata>> {
        let resolved_type = match current!(self, ttype) {
            TokenIdentifier => {
                self.advance();
                let type_lexeme = self.get_previous().get_lexeme(&&self.source);
                match type_lexeme.get_lexeme_str() {
                    "Int" => Ok(Some(ValueType::Int)),
                    "Bool" => Ok(Some(ValueType::Bool)),
                    "String" => Ok(Some(ValueType::String)),
                    "Void" => Ok(Some(ValueType::Void)),
                    _ => Ok(None), // This should make a custom type
                }
            }
            _ => Err(vec![self.get_current().get_metadata()]),
        };

        resolved_type
    }

    pub(super) fn expression<'b, 'c>(
        &mut self,
        precedence: Precedence,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> Result<Expr<'b>, CompileError> {
        let mut expr_builder = ExprBuilder::new(ast_arena);

        self.parse_precedence(precedence, (&mut expr_builder, program_symbol_table, ast_arena))?;

        Ok(expr_builder.get_built_expr())
    }

    pub(super) fn parse_precedence<'b, 'c>(
        &mut self,
        precedence: Precedence,
        (expr_builder, program_symbol_table, ast_arena): ExprMethodArgs<'b, 'c>
    ) -> ExprMethodRetType {
        self.advance();

        let parse_rule = self.get_parse_rule(previous!(self, ttype));

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self, (expr_builder, program_symbol_table, ast_arena))?;

            loop {
                let current_precedence = self
                    .get_parse_rule(current!(self, ttype))
                    .get_precedence();

                if (precedence as usize) > (*current_precedence as usize) {
                    break;
                }

                if previous!(self, line) < current!(self, line) {
                    break;
                }

                self.advance();

                let infix_rule = self.get_parse_rule(previous!(self, ttype)).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self, (expr_builder, program_symbol_table, ast_arena))?;
                }
            }
        } else {
            return Err(
                CompileError::new(
                    ReportedError::new(
                        format!(
                            "Unexpected token: '{}' (no prefix rule)",
                            previous!(self, lexeme).get_lexeme_str()
                        ),
                        previous!(self, metadata).into()
                    )
                )
            );
        }

        Ok(())
    }

    pub(super) fn resolve_function_args(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1
    ) -> Result<Vec<FnArg>, CompileError> {
        let mut args = vec![];

        self.consume(TokenLeftParen, "Expected '(' after function identifier")?;

        while !self.is_at_expr_end() {
            if current!(self, ttype).is(&TokenComma) {
                self.advance();
            }
            if current!(self, ttype).is(&TokenRightParen) {
                break;
            }

            self.consume(
                TokenIdentifier,
                format!(
                    "Expected identifer but got '{}' Syntax: <identifier> <type>",
                    self.get_current().get_lexeme(&self.source).get_lexeme_str()
                ).as_str()
            )?;

            let (ident_lexeme, ident_metadata) = previous!(self, lexeme, metadata);

            let mut_keyword_metadata = match current!(self, ttype).is(&TokenMutable) {
                true => {
                    self.advance();
                    Some(previous!(self, metadata))
                }
                false => None,
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

            let ssa_ident = program_symbol_table.declare_var_ssa_ident(&ident_lexeme);

            args.push(FnArg::new(ssa_ident, arg_type, mut_keyword_metadata, ident_metadata));
        }

        self.consume(TokenRightParen, "Expected a closing ')' after function arguments")?;

        Ok(args)
    }

    pub(super) fn resolve_function_return_type(
        &mut self
    ) -> Result<Option<ValueType>, CompileError> {
        let return_type = match current!(self, ttype) {
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
            if previous!(self, ttype) == &TokenSemicolon {
                return;
            }
            if self.get_previous().get_line() < self.get_current().get_line() {
                // This might need to change, as soon as blocks are implemented
                return;
            }

            match current!(self, ttype) {
                TokenError => {
                    self.advance();
                }
                _ => self.advance(),
            }
        }
    }
}
