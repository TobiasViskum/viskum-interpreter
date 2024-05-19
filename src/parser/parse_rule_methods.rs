use std::fmt::format;

use super::precedence::Precedence::*;
use crate::ast;
use crate::ast::expr::{ AstIdentifier, AstValue, Expr, ExprBuilder };
use crate::ast::stmt::{ FunctionStmt, IfStmt, ScopeStmt, Stmt, TypeDefStmt };
use crate::error_handler::CompileError;
use crate::operations::{ BinaryOp, UnaryOp };
use crate::parser::token::token_type::TokenType::{ self, * };
use crate::value::Value;

use super::{ ParseMethod, Parser, RuleArg };

impl<'a> Parser<'a> {
    pub fn number(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(&self.source);

        match lexeme.parse::<i32>() {
            Ok(int_value) => {
                expr_builder.emit_constant_literal(
                    AstValue::new(Value::Int32(int_value), token.get_metadata())
                );
            }
            Err(e) => {
                eprintln!("Could not parse i32. In c.number(RuleArg): {}", e);
            }
        }

        Ok(())
    }

    pub fn mut_var_def(&mut self) -> Result<Stmt, CompileError> {
        self.advance();

        match self.get_current().get_ttype() {
            TokenIdentifier => self.var_def(true),
            TokenFunction => {
                panic!("Functions cannot be mutable");
            }
            _ => panic!("Unexpected: {}", self.get_current().get_lexeme(&self.source)),
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
                self.get_current().get_lexeme(&self.source)
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
                        Stmt::FunctionStmt(_) => scope_stmt.forwards_declarations.push(stmt),
                        _ => scope_stmt.cf_stmts.push(stmt),
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
                    format!("Unexpected token: {}", self.get_previous().get_lexeme(self.source)),
                    vec![self.get_previous().get_metadata()]
                )
            );
        } else {
            self.consume(
                TokenString,
                format!(
                    "Unexpected token: {}",
                    self.get_previous().get_lexeme(self.source)
                ).as_str()
            )?;
        }

        let token = self.get_previous();
        expr_builder.emit_constant_literal(
            AstValue::new(Value::String(token.get_lexeme(&self.source)), token.get_metadata())
        );

        self.consume(TokenDoubleQuote, "Expected '\"' after string")?;

        Ok(())
    }

    pub fn literal(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let token = self.get_previous();

        match token.get_ttype() {
            TokenFalse =>
                expr_builder.emit_constant_literal(
                    AstValue::new(Value::Bool(false), token.get_metadata())
                ),
            TokenTrue =>
                expr_builder.emit_constant_literal(
                    AstValue::new(Value::Bool(true), token.get_metadata())
                ),
            _ => {
                return Err(
                    CompileError::new(
                        format!("Unexpected literal: '{}'", token.get_lexeme(&self.source)),
                        vec![token.get_metadata()]
                    )
                );
            }
        }

        Ok(())
    }

    pub fn if_stmt(&mut self) -> Result<IfStmt, CompileError> {
        self.advance();

        let condition = self.expression(PrecAssignment)?;

        let true_block = self.block()?;

        let false_block = if self.get_current().get_ttype().is(&TokenType::TokenElse) {
            self.advance();
            if self.get_current().get_ttype().is(&TokenType::TokenIf) {
                Some(Box::new(self.if_stmt()?))
            } else {
                let true_block = self.block()?;
                Some(Box::new(IfStmt::new(None, true_block, None)))
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

        let return_type = match self.resolve_function_return_type() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let mut body = ScopeStmt::new();

        self.consume(TokenLeftCurlyBrace, "Expected '{' before function body")?;
        while !self.is_at_end() && !matches!(self.get_current().get_ttype(), &TokenRightCurlyBrace) {
            body.push_stmt(self.statement()?);
        }
        self.consume(TokenRightCurlyBrace, "Expected '}' after function body")?;

        let function_stmt = FunctionStmt::new(lexeme, function_args, return_type, body, metadata);

        Ok(Stmt::FunctionStmt(function_stmt))
    }

    pub fn grouping(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        self.parse_precedence(PrecAssignment.get_next(), expr_builder)?;

        self.consume(TokenRightParen, "Expect ')' after expression")?;

        Ok(())
    }

    pub fn unary(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let operator_type = *self.get_previous().get_ttype();

        self.parse_precedence(PrecAssignment.get_next(), expr_builder)?;

        let unary_op = match operator_type.parse_unary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        format!(
                            "Token '{}' is not a valid unary operator",
                            self.get_previous().get_lexeme(&self.source)
                        ),
                        vec![self.get_previous().get_metadata()]
                    )
                );
            }
        };

        expr_builder.emit_unary_op(unary_op)
    }

    pub fn binary(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let operator_type = *self.get_previous().get_ttype();

        self.parse_precedence(
            self.get_parse_rule(&operator_type).get_precedence().get_next(),
            expr_builder
        )?;

        let binary_op = match operator_type.parse_binary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        format!(
                            "Token '{}' is not a valid binary operator",
                            self.get_previous().get_lexeme(&self.source)
                        ),
                        vec![self.get_previous().get_metadata()]
                    )
                );
            }
        };

        expr_builder.emit_binary_op(binary_op)
    }

    pub(super) fn error(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let error_token = self.get_previous();
        let msg = error_token.get_message();

        if let Some(msg) = msg {
            Err(CompileError::new(msg.to_string(), vec![error_token.get_metadata()]))
        } else {
            Err(
                CompileError::new(
                    format!("Unexpected token: {}", error_token.get_lexeme(&self.source)),
                    vec![error_token.get_metadata()]
                )
            )
        }
    }
}
