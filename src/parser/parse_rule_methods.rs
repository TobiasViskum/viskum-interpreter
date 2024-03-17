use crate::ast::{ AstIdentifier, AstValue };
use crate::operations::{ BinaryOp, UnaryOp };
use crate::{ parser::token::token_type::TokenType::*, value::Value };
use super::precedence::Precedence::*;

use super::Parser;

impl<'a> Parser<'a> {
    pub fn number(&mut self) {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        if let Ok(int_value) = lexeme.parse::<i32>() {
            self.ast_generator.emit_constant_literal(
                AstValue::new(Value::Int32(int_value), token.get_metadata())
            )
        }
    }

    pub fn identifier(&mut self) {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        self.ast_generator.emit_identifier_lookup(AstIdentifier::new(lexeme, token.get_metadata()));
    }

    pub fn literal(&mut self) {
        let token = self.get_previous();

        match token.get_ttype() {
            TokenFalse =>
                self.ast_generator.emit_constant_literal(
                    AstValue::new(Value::Bool(false), token.get_metadata())
                ),
            TokenTrue =>
                self.ast_generator.emit_constant_literal(
                    AstValue::new(Value::Bool(true), token.get_metadata())
                ),
            _ => {}
        }
    }

    pub fn grouping(&mut self) {
        self.expression();

        self.consume(TokenRightParen, "Expect ')' after expression");
    }

    pub fn unary(&mut self) {
        let operator_type = { *self.get_previous().get_ttype() };

        self.parse_precedence(PrecUnary, None);

        let result = match operator_type {
            TokenMinus => self.ast_generator.emit_unary_op(UnaryOp::Neg),
            TokenBang => self.ast_generator.emit_unary_op(UnaryOp::Truthy),
            _ => Ok(()),
        };

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }
    }

    pub fn binary(&mut self) {
        let operator_type = { *self.get_previous().get_ttype() };

        let parse_rule = self.get_parse_rule(&operator_type);

        self.parse_precedence(parse_rule.get_precedence().get_next(), None);

        let result = match operator_type {
            TokenPlus => self.ast_generator.emit_binary_op(BinaryOp::Add),
            TokenMinus => self.ast_generator.emit_binary_op(BinaryOp::Sub),
            TokenStar => self.ast_generator.emit_binary_op(BinaryOp::Mul),
            TokenSlash => self.ast_generator.emit_binary_op(BinaryOp::Div),
            _ => Ok(()),
        };

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }
    }

    pub(super) fn error(&mut self) {
        let error_token = self.get_previous();
        let msg = error_token.get_message();
        if let Some(msg) = msg {
            self.report_compile_error(msg.to_string(), vec![error_token.get_metadata()]);
        } else {
            self.report_compile_error(
                format!("Unexpected token: {}", error_token.get_lexeme(self.source)),
                vec![error_token.get_metadata()]
            );
        }
    }

    pub(super) fn skip(&mut self) {
        self.parse_precedence(PrecAssignment, None)
    }
}
