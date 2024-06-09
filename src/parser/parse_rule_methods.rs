use super::precedence::Precedence::*;
use crate::ast::expr::{ ExprBuilder, LiteralExpr };
use crate::ast::stmt::{ ExprStmt, FunctionStmt, IfStmt, ScopeStmt, Stmt };
use crate::error_handler::CompileError;
use crate::value::{ Value, ValueType };
use super::Parser;
use super::TokenType::{ self, * };

impl<'a> Parser<'a> {
    pub fn number(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
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
                        format!("Could not parse number '{}'", lexeme.get_lexeme_str()),
                        self.get_previous().get_metadata().into()
                    )
                );
            }
        }
    }

    pub fn mut_var_def(&mut self) -> Result<Stmt, CompileError> {
        self.advance();

        match self.get_current().get_ttype() {
            TokenIdentifier => self.var_def(true),
            TokenFunction => {
                panic!("Functions cannot be mutable");
            }
            _ =>
                panic!(
                    "Unexpected: {}",
                    self.get_current().get_lexeme(&self.source).get_lexeme_str()
                ),
        }
    }

    pub fn identifier(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        match self.get_current().get_ttype() {
            TokenLeftParen => self.fn_call(expr_builder)?,
            _ => self.ident_lookup(expr_builder)?,
        }

        Ok(())
    }

    pub fn block(&mut self) -> Result<ScopeStmt, CompileError> /* Only error if no '}' */ {
        self.consume(
            TokenType::TokenLeftCurlyBrace,
            format!(
                "Expected '{{' but got: {}",
                self.get_current().get_lexeme(&self.source).get_lexeme_str()
            ).as_str()
        )?;

        let mut scope_stmt = ScopeStmt::new();

        while
            !self.is_at_end() &&
            !matches!(self.get_current().get_ttype(), &TokenType::TokenRightCurlyBrace)
        {
            match self.statement() {
                Ok(stmt) =>
                    match stmt {
                        Stmt::FunctionStmt(_) => scope_stmt.push_forward_stmt(stmt),
                        _ => scope_stmt.push_stmt(stmt),
                    }
                Err(e) => {
                    // RCE
                }
            }
        }
        self.consume(TokenType::TokenRightCurlyBrace, "Expected '}' at the end of block")?;

        Ok(scope_stmt)
    }

    pub fn typing(&mut self) {
        panic!("Typings not supported yet")
        /*
        self.advance();

        let lexeme = self.get_previous().get_lexeme(&self.source);

        self.consume(
            TokenType::TokenAssign,
            format!("Expected '=' after type definition but got '{}'", lexeme).as_str()
        );

        let typing = match self.resolve_typing() {
            Ok(typing) => typing,
            Err(_) => {
                return;
            }
        };

        match self.ast_generator.emit_type_definition(TypeDefStmt::new(lexeme, typing)) {
            Ok(_) => {}
            Err((msg, token_vec)) => { self.report_compile_error(msg, token_vec) }
        }
        */
    }

    pub fn string(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        if self.is_at_end() {
            return Err(
                CompileError::new(
                    format!(
                        "Unexpected token: {}",
                        self.get_previous().get_lexeme(self.source).get_lexeme_str()
                    ),
                    self.get_previous().get_metadata().into()
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

    pub fn literal(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
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
                        format!(
                            "Unexpected literal: '{}'",
                            token.get_lexeme(&self.source).get_lexeme_str()
                        ),
                        token.get_metadata().into()
                    )
                );
            }
        }

        Ok(())
    }

    pub fn if_stmt(&mut self) -> Result<IfStmt, CompileError> {
        self.advance();

        let condition = ExprStmt::new(self.expression(PrecAssignment)?);

        let true_block = self.block()?;

        let false_block = if self.get_current().get_ttype().is(&TokenType::TokenElse) {
            self.advance();
            if self.get_current().get_ttype().is(&TokenType::TokenIf) {
                Some(self.if_stmt()?)
            } else {
                let true_block = self.block()?;
                Some(IfStmt::new(None, true_block, None))
            }
        } else {
            None
        };

        Ok(IfStmt::new(Some(condition), true_block, false_block))
    }

    pub fn function(&mut self) -> Result<Stmt, CompileError> {
        self.advance();

        let lexeme = self.get_current().get_lexeme(&self.source);
        let metadata = self.get_current().get_metadata();

        self.advance();

        let function_args = match self.resolve_function_args() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let return_type = (
            match self.resolve_function_return_type() {
                Ok(v) => v,
                Err(e) => {
                    return Err(e);
                }
            }
        ).unwrap_or(ValueType::Void);

        let mut body = ScopeStmt::new();

        self.consume(TokenLeftCurlyBrace, "Expected '{' before function body")?;
        while !self.is_at_end() && !matches!(self.get_current().get_ttype(), &TokenRightCurlyBrace) {
            body.push_stmt(self.statement()?);
        }
        self.consume(TokenRightCurlyBrace, "Expected '}' after function body")?;

        let function_stmt = FunctionStmt::new(
            lexeme.take_lexeme_rc(),
            function_args,
            return_type,
            body,
            metadata
        );

        Ok(Stmt::FunctionStmt(function_stmt))
    }

    pub fn grouping(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        self.parse_precedence(PrecAssignment.get_next(), expr_builder)?;

        self.consume(TokenRightParen, "Expect ')' after expression")?;

        Ok(())
    }

    pub fn unary(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let unary_token = self.get_previous().clone();

        self.parse_precedence(PrecUnary.get_next(), expr_builder)?;

        let unary_op = match unary_token.get_ttype().parse_unary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        format!(
                            "Token '{}' is not a valid unary operator",
                            unary_token.get_lexeme(&self.source).get_lexeme_str()
                        ),
                        unary_token.get_metadata().into()
                    )
                );
            }
        };

        expr_builder.emit_unary_op(unary_op, unary_token.get_metadata())
    }

    pub fn binary(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let binary_token = self.get_previous().clone();

        self.parse_precedence(
            self.get_parse_rule(&binary_token.get_ttype()).get_precedence().get_next(),
            expr_builder
        )?;

        let binary_op = match binary_token.get_ttype().parse_binary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        format!(
                            "Token '{}' is not a valid binary operator",
                            binary_token.get_lexeme(&self.source).get_lexeme_str()
                        ),
                        binary_token.get_metadata().into()
                    )
                );
            }
        };

        expr_builder.emit_binary_op(binary_op, binary_token.get_metadata())
    }

    pub(super) fn error(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let error_token = self.get_previous();
        let msg = error_token.get_message();

        if let Some(msg) = msg {
            Err(CompileError::new(msg.to_string(), error_token.get_metadata().into()))
        } else {
            Err(
                CompileError::new(
                    format!(
                        "Unexpected token: {}",
                        error_token.get_lexeme(&self.source).get_lexeme_str()
                    ),
                    error_token.get_metadata().into()
                )
            )
        }
    }
}
