use crate::ast::{ AstValue, ExprOp };
use crate::{ parser::token::token_type::TokenType::*, value::Value };
use super::precedence::Precedence::*;

use super::Parser;

impl<'a> Parser<'a> {
    pub fn number(&mut self) {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        if let Ok(int_value) = lexeme.parse::<i32>() {
            self.ast_generator.emit_constant(AstValue::new(Value::Int32(int_value), token.clone()))
        }
    }

    pub fn variable(&mut self) {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        unimplemented!("Variable parsing not implemented yet")
    }

    pub fn literal(&mut self) {
        let token = self.get_previous();

        match token.get_ttype() {
            TokenFalse =>
                self.ast_generator.emit_constant(AstValue::new(Value::Bool(false), token.clone())),
            TokenTrue =>
                self.ast_generator.emit_constant(AstValue::new(Value::Bool(true), token.clone())),
            _ => {}
        }
    }

    pub fn grouping(&mut self) {
        self.expression();

        self.consume(TokenRightParen)
    }

    pub fn unary(&mut self) {
        let operator_type = { self.get_previous().get_ttype().clone() };

        self.parse_precedence(PrecUnary);

        let result = match operator_type {
            TokenMinus => self.ast_generator.emit_unary_op(ExprOp::Neg),
            TokenBang => self.ast_generator.emit_unary_op(ExprOp::Truthy),
            _ => Ok(()),
        };

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }
    }

    pub fn binary(&mut self) {
        let operator_type = { self.get_previous().get_ttype().clone() };

        let parse_rule = self.get_parse_rule(&operator_type);

        self.parse_precedence(parse_rule.get_precedence().get_next());

        let result = match operator_type {
            TokenPlus => self.ast_generator.emit_binary_op(ExprOp::Add),
            TokenMinus => self.ast_generator.emit_binary_op(ExprOp::Sub),
            TokenStar => self.ast_generator.emit_binary_op(ExprOp::Mul),
            TokenSlash => self.ast_generator.emit_binary_op(ExprOp::Div),
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
            self.report_compile_error(msg.to_string(), error_token.clone());
        } else {
            self.report_compile_error(
                format!("Unexpected token: {}", error_token.get_lexeme(self.source)),
                error_token.clone()
            );
        }
        // self.synchronize()
    }
}
