use crate::ast;
use crate::ast::expr::{ AstIdentifier, AstValue, Expr };
use crate::ast::stmt::TypeDefStmt;
use crate::operations::{ BinaryOp, UnaryOp };
use crate::value::Value;
use crate::parser::token::token_type::TokenType::{ self, * };
use super::precedence::Precedence::*;

use super::{ Parser, RuleArg };

impl<'a> Parser<'a> {
    pub fn number(&mut self, rule_arg: RuleArg) {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        match lexeme.parse::<i32>() {
            Ok(int_value) => {
                self.ast_generator.emit_constant_literal(
                    AstValue::new(Value::Int32(int_value), token.get_metadata())
                );
            }
            Err(e) => {
                eprintln!("Could not parse i32. In c.number(RuleArg): {}", e);
            }
        }
    }

    pub fn mut_var_def(&mut self, rule_arg: RuleArg) {
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

    pub fn identifier(&mut self, rule_arg: RuleArg) {
        match self.is_at_expr_end() {
            true => self.ident_lookup(),
            false => {
                match self.get_current().get_ttype() {
                    TokenAssign => self.var_assign(),
                    TokenIdentifier | TokenDefine => self.var_def(RuleArg::None),
                    TokenLeftParen => println!("identifier: function"),
                    _ => self.ident_lookup(),
                }
            }
        }
    }

    pub fn block(&mut self, rule_arg: RuleArg) {
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

    pub fn typing(&mut self, rule_arg: RuleArg) {
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

    pub fn literal(&mut self, rule_arg: RuleArg) {
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

    pub fn if_statement(&mut self, rule_arg: RuleArg) {
        self.expression_statement();

        self.statement();

        println!("{:#?}", self.ast_generator);

        panic!("OH NO, IF IS NOT IMPL YET")
    }

    pub fn function(&mut self, rule_arg: RuleArg) {
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

    pub fn grouping(&mut self, rule_arg: RuleArg) {
        self.expression();

        self.consume(TokenRightParen, "Expect ')' after expression");
    }

    pub fn unary(&mut self, rule_arg: RuleArg) {
        let operator_type = { *self.get_previous().get_ttype() };

        self.parse_precedence(PrecUnary, None);

        let unary_op = match operator_type.parse_unary() {
            Ok(op) => op,
            Err(_) => {
                self.report_compile_error(
                    format!(
                        "Token '{}' is not a valid unary operator",
                        self.get_previous().get_lexeme(self.source)
                    ),
                    vec![self.get_previous().get_metadata()]
                );
                return;
            }
        };

        let result = self.ast_generator.emit_unary_op(unary_op);

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }
    }

    pub fn binary(&mut self, rule_arg: RuleArg) {
        let operator_type = { *self.get_previous().get_ttype() };

        let parse_rule = self.get_parse_rule(&operator_type);

        self.parse_precedence(parse_rule.get_precedence().get_next(), None);

        let binary_op = match operator_type.parse_binary() {
            Ok(op) => op,
            Err(_) => {
                self.report_compile_error(
                    format!(
                        "Token '{}' is not a valid binary operator",
                        self.get_previous().get_lexeme(self.source)
                    ),
                    vec![self.get_previous().get_metadata()]
                );
                return;
            }
        };

        let result = self.ast_generator.emit_binary_op(binary_op);

        if let Err((message, token)) = result {
            self.report_compile_error(message, token);
        }
    }

    pub(super) fn error(&mut self, rule_arg: RuleArg) {
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

    pub(super) fn skip(&mut self, rule_arg: RuleArg) {
        self.parse_precedence(PrecAssignment, None)
    }
}
