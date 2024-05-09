use crate::ast;
use crate::ast::expr::{ AstIdentifier, AstValue, Expr, ExprBuilder };
use crate::ast::stmt::TypeDefStmt;
use crate::error_handler::CompileError;
use crate::operations::{ BinaryOp, UnaryOp };
use crate::value::Value;
use crate::parser::token::token_type::TokenType::{ self, * };
use super::precedence::Precedence::*;

use super::{ ParseMethod, Parser, RuleArg };

impl<'a> Parser<'a> {
    pub fn number(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

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

    pub fn mut_var_def(&mut self) {
        match self.get_current().get_ttype() {
            TokenIdentifier => {
                self.advance();

                self.var_def(RuleArg::MutVar);
            }
            TokenFunction => {
                panic!("Functions cannot be mutable");
            }
            _ => panic!("Unexpected: {}", self.get_current().get_lexeme(self.source)),
        }
    }

    pub fn identifier(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        match self.is_at_expr_end() {
            true => self.ident_lookup(expr_builder)?,
            false => {
                match self.get_current().get_ttype() {
                    TokenAssign => self.var_assign(),
                    TokenIdentifier | TokenDefine => self.var_def(RuleArg::None),
                    TokenLeftParen => panic!("identifier: function"),
                    _ => self.ident_lookup(expr_builder)?,
                }
            }
        }

        Ok(())
    }

    pub fn block(&mut self) {
        self.start_scope();

        while
            !self.is_at_end() &&
            !matches!(self.get_current().get_ttype(), &TokenType::TokenRightCurlyBrace)
        {
            self.statement();
        }
        self.consume(TokenType::TokenRightCurlyBrace, "Expected '}' at the end of block");

        self.end_scope()
    }

    pub fn typing(&mut self) {
        panic!("Typings not supported yet")
        /*
        self.advance();

        let lexeme = self.get_previous().get_lexeme(self.source);

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
            _ => {}
        }

        Ok(())
    }

    pub fn if_statement(&mut self) {
        self.expression_statement();

        self.statement();

        println!("{:#?}", self.ast_generator);

        panic!("OH NO, IF IS NOT IMPL YET")
    }

    pub fn function(&mut self) {
        self.advance();
        let lexeme = self.get_previous().get_lexeme(self.source);

        let function_args = match self.resolve_function_args() {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        };

        println!("{:#?}", function_args);

        let return_type = match self.resolve_function_return_type() {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        };

        println!("Return type: {:?}", return_type);

        self.start_function(lexeme, function_args, return_type);

        println!("I run");

        self.consume(TokenLeftCurlyBrace, "Expected '{' before function body");
        while !self.is_at_end() && !matches!(self.get_current().get_ttype(), &TokenRightCurlyBrace) {
            self.statement();
        }
        self.consume(TokenRightCurlyBrace, "Expected '}' after function body");

        self.end_function();
    }

    pub fn grouping(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        self.expression();

        self.consume(TokenRightParen, "Expect ')' after expression");

        Ok(())
    }

    pub fn unary(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let operator_type = { *self.get_previous().get_ttype() };

        self.parse_precedence(PrecUnary, None);

        let unary_op = match operator_type.parse_unary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        format!(
                            "Token '{}' is not a valid unary operator",
                            self.get_previous().get_lexeme(self.source)
                        ),
                        vec![self.get_previous().get_metadata()]
                    )
                );
            }
        };

        expr_builder.emit_unary_op(unary_op)
    }

    pub fn binary(&mut self, expr_builder: &mut ExprBuilder) -> Result<(), CompileError> {
        let operator_type = { *self.get_previous().get_ttype() };

        let parse_rule = self.get_parse_rule(&operator_type);

        self.parse_precedence(parse_rule.get_precedence().get_next(), None);

        let binary_op = match operator_type.parse_binary() {
            Ok(op) => op,
            Err(_) => {
                return Err(
                    CompileError::new(
                        format!(
                            "Token '{}' is not a valid binary operator",
                            self.get_previous().get_lexeme(self.source)
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
                    format!("Unexpected token: {}", error_token.get_lexeme(self.source)),
                    vec![error_token.get_metadata()]
                )
            )
        }
    }
}
