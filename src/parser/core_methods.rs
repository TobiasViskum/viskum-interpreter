use crate::{
    ast::{
        expr::{ Expr, ExprBuilder, FnCallExpr, IdentifierExpr },
        stmt::{
            BreakStmt,
            ContinueStmt,
            ExprStmt,
            FunctionArgument,
            LoopStmt,
            ReturnStmt,
            Stmt,
            VarAssignStmt,
            VarDefStmt,
        },
    },
    error_handler::{ CompileError, SrcCharsRange },
    merge_chars_range,
    value::ValueType,
    vm::instructions::NativeCall,
};

use super::{ precedence::Precedence, token::TokenMetadata, Parser, TokenType::{ self, * } };

impl<'a> Parser<'a> {
    pub(super) fn statement(&mut self) -> Result<Stmt, CompileError> {
        let curr = self.get_current().get_ttype();

        match curr {
            TokenLeftCurlyBrace => Ok(Stmt::ScopeStmt(self.block()?)),
            TokenMutable => self.mut_var_def(),
            TokenFunction => self.function(),
            TokenIf => Ok(Stmt::IfStmt(self.if_stmt()?)),
            TokenLoop => self.loop_stmt(),
            TokenBreak => self.break_stmt(),
            TokenContinue => self.continue_stmt(),
            TokenReturn => self.return_stmt(),

            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_line(TokenAssign) => {
                self.var_assign()
            }
            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_line(TokenDefine) => {
                self.var_def(false)
            }

            _ => self.expression_statement(),
        }
    }

    pub(super) fn return_stmt(&mut self) -> Result<Stmt, CompileError> {
        let metadata = self.get_current().get_metadata();

        self.advance();

        let return_expr = if !self.is_at_expr_end() {
            Some(ExprStmt::new(self.expression(Precedence::PrecAssignment.get_next())?))
        } else {
            None
        };

        Ok(Stmt::ReturnStmt(ReturnStmt::new(return_expr, metadata)))
    }

    pub(super) fn continue_stmt(&mut self) -> Result<Stmt, CompileError> {
        self.advance();
        self.consume_expr_end()?;

        Ok(Stmt::ContinueStmt(ContinueStmt::new()))
    }

    pub(super) fn break_stmt(&mut self) -> Result<Stmt, CompileError> {
        self.advance();
        self.consume_expr_end()?;

        Ok(Stmt::BreakStmt(BreakStmt::new()))
    }

    pub(super) fn loop_stmt(&mut self) -> Result<Stmt, CompileError> {
        self.advance();

        let body = self.block()?;

        Ok(Stmt::LoopStmt(LoopStmt::new(None, body)))
    }

    pub(super) fn is_ttype_in_line(&self, token_type: TokenType) -> bool {
        let mut i: usize = 0;

        let start_line = self.get_current().get_line();

        loop {
            i += 1;

            match self.peek(i as isize) {
                Some(token) => {
                    if token.get_line() > start_line {
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

    pub(super) fn var_assign(&mut self) -> Result<Stmt, CompileError> {
        let target_expr = ExprStmt::new(self.expression(Precedence::PrecCall)?);

        if !self.get_current().get_ttype().is(&TokenAssign) {
            let mut token_vec = vec![self.get_previous().get_metadata()];
            while !self.get_current().get_ttype().is(&TokenAssign) {
                token_vec.push(self.get_current().get_metadata());
                self.advance();
            }

            return Err(
                CompileError::new(
                    "Invalid left-hand side of assignment".to_string(),
                    merge_chars_range!(
                        token_vec
                            .iter()
                            .map(|m| (*m).into())
                            .collect()
                    )
                )
            );
        }

        self.advance();

        let value = ExprStmt::new(self.expression(Precedence::PrecAssignment.get_next())?);

        Ok(Stmt::VarAssignStmt(VarAssignStmt { target_expr, value }))
    }

    pub(super) fn var_def(&mut self, is_mutable: bool) -> Result<Stmt, CompileError> {
        self.advance();

        let (lexeme, token_metadata) = {
            let token = self.get_previous();
            (token.get_lexeme(&self.source), token.get_metadata())
        };

        let found_type = match self.resolve_type() {
            Ok(found_type) => { found_type }
            Err(_) => None,
        };

        let value = if !self.is_at_expr_end() {
            self.consume(
                TokenDefine,
                format!(
                    "Expected ':=' in variable definition but got '{}'",
                    self.get_current().get_lexeme(&self.source).get_lexeme_str()
                ).as_str()
            )?;

            if self.is_at_expr_end() {
                return Err(
                    CompileError::new(
                        "Missing right hand side of variable definition".to_string(),
                        self.get_previous().get_metadata().into()
                    )
                );
            }
            let value = self.expression(Precedence::PrecAssignment.get_next())?;

            Some(ExprStmt::new(value))
        } else {
            None
        };

        Ok(
            Stmt::VarDefStmt(
                VarDefStmt::new(
                    lexeme.take_lexeme_rc(),
                    found_type,
                    is_mutable,
                    value,
                    token_metadata
                )
            )
        )
    }

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

    pub(super) fn expression_statement(&mut self) -> Result<Stmt, CompileError> {
        let expr = self.expression(Precedence::PrecAssignment.get_next())?;

        self.consume_expr_end()?;

        Ok(Stmt::ExprStmt(ExprStmt::new(expr)))

        /*
        match self.ast_generator.push_expr() {
            Ok(_) => {}
            Err((message, token)) => {
                self.report_compile_error(message, token);
                self.synchronize()
            }
        }
        */
    }

    pub(super) fn expression(&mut self, precedence: Precedence) -> Result<Expr, CompileError> {
        let mut expr_builder = ExprBuilder::new();

        self.parse_precedence(precedence, &mut expr_builder)?;

        Ok(expr_builder.get_built_expr())
    }

    pub(super) fn parse_precedence(
        &mut self,
        precedence: Precedence,
        mut expr_builder: &mut ExprBuilder
    ) -> Result<(), CompileError> {
        self.advance();

        let parse_rule = self.get_parse_rule(self.get_previous().get_ttype());

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self, &mut expr_builder)?;

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
                    infix_rule(self, &mut expr_builder)?;
                }
            }
        } else {
            return Err(
                CompileError::new(
                    format!(
                        "Unexpected token: '{}' (no prefix rule)",
                        self.get_previous().get_lexeme(&self.source).get_lexeme_str()
                    ),
                    self.get_previous().get_metadata().into()
                )
            );
        }

        Ok(())
    }

    pub(super) fn fn_call(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(&self.source);
        let metadata = token.get_metadata();

        self.advance();

        let mut fn_args = vec![];
        while !self.is_at_expr_end() {
            if self.get_current().get_ttype().is(&TokenRightParen) {
                break;
            }
            let arg = self.expression(Precedence::PrecAssignment.get_next())?;

            fn_args.push(arg);

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

        let native_call = NativeCall::get_native(lexeme.get_lexeme_str());

        if let Some(native_call) = native_call {
            panic!("Native calls not supported yet!");
            // expr_builder.emit_native_call(AstNativeCall::new(metadata, fn_args, native_call));
        } else {
            expr_builder.emit_fn_call(FnCallExpr::new(lexeme.take_lexeme_rc(), metadata, fn_args));
        }

        Ok(())
    }

    pub(super) fn ident_lookup(
        &mut self,
        expr_builder: &mut ExprBuilder
    ) -> Result<(), CompileError> {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(&self.source);

        expr_builder.emit_ident_lookup(
            IdentifierExpr::new(lexeme.take_lexeme_rc(), token.get_metadata())
        );

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
                            "Invalid type".to_string(),
                            merge_chars_range!(
                                error_tokens
                                    .iter()
                                    .map(|m| (*m).into())
                                    .collect()
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

    pub(super) fn emit_operand(
        &mut self,
        expr_builder: &mut ExprBuilder
    ) -> Result<(), CompileError> {
        self.advance();
        match self.get_previous().get_ttype() {
            TokenIdentifier => self.identifier(expr_builder)?,
            TokenTrue | TokenFalse => self.literal(expr_builder)?,
            TokenNumber => self.number(expr_builder)?,
            _ => {
                return Err(
                    CompileError::new(
                        format!(
                            "Expected operand but got: '{}'",
                            self.get_previous().get_lexeme(&self.source).get_lexeme_str()
                        ),
                        self.get_previous().get_metadata().into()
                    )
                );
            }
        }

        Ok(())
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
                                "Invalid function return type".to_string(),
                                merge_chars_range!(
                                    error_tokens
                                        .iter()
                                        .map(|m| (*m).into())
                                        .collect()
                                )
                            )
                        );
                    }
                }
            _ => {
                return Err(
                    CompileError::new(
                        format!(
                            "Expected function return type, but got: {}",
                            self.get_current().get_lexeme(&self.source).get_lexeme_str()
                        ),
                        self.get_current().get_metadata().into()
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

/*
pub(super) fn resolve_typing(&mut self) -> Result<Typing, ()> {
        let current_token = self.get_current();
        let token_metadata = current_token.get_metadata();

        let typing = match current_token.get_ttype() {
            TokenInt32 => Typing::new_int32(token_metadata),
            TokenBool => Typing::new_bool(token_metadata),
            TokenIdentifier =>
                Typing::new_custom(current_token.get_lexeme(&self.source), token_metadata),
            _ => {
                self.report_compile_error(
                    format!(
                        "Unexpected '{}' cannot be used as a type",
                        current_token.get_lexeme(&self.source)
                    ),
                    vec![token_metadata]
                );
                return Err(());
            }
        };

        self.advance();

        Ok(typing)
        /*
        Idea for resolving types like: state<i32>

        .append_type() -> Result<(), String> // x does not take any type parameters
        */

        // match current_token.get_ttype() {
        //     TokenIdentifier => crate::value_v2::ValueType::
        // }
    }

    pub(super) fn resolve_type(&mut self) -> (Option<ValueType>, Option<Token>, isize) {
        let mut search_index: isize = -2;
        let line = self.get_previous().get_line();

        loop {
            match self.peek(search_index) {
                Some(token) => {
                    if
                        token.get_line() < line ||
                        matches!(token.get_ttype(), TokenEOF | TokenSemicolon)
                    {
                        return (None, None, search_index);
                    }

                    if token.get_ttype() == &TokenInt32 {
                        return (Some(ValueType::Int32), Some(token.clone()), search_index);
                    } else if token.get_ttype() == &TokenBool {
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
            Some(token) => (token.get_ttype() == &TokenMutable, start_search_index),
            None => (false, start_search_index),
        }
    }

pub(super) fn variable_definition(&mut self) {
        let token = self.get_previous().clone();

        let lexeme = token.get_lexeme(&self.source);

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
                    TokenDefine,
                    format!(
                        "Expected ':=' in variable definition but got '{}'",
                        self.get_current().get_lexeme(&self.source)
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
            token.get_metadata(),
            found_type,
            is_mutable,
            type_or_identifier_token.get_metadata()
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
*/
