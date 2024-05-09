use crate::{
    ast::{
        expr::{ AstIdentifier, Expr, ExprBuilder },
        stmt::{ FunctionArgument, Stmt, Typing, VarAssignStmt, VarDefStmt },
    },
    error_handler::CompileError,
    value::ValueType,
};

use super::{
    precedence::Precedence,
    token::{ token_type::TokenType, Token, TokenMetadata },
    ParseMethod,
    Parser,
    RuleArg,
};

impl<'a> Parser<'a> {
    pub(super) fn statement(&mut self) -> Stmt {
        let current = self.get_current().get_ttype();

        match current {
            TokenType::TokenLeftCurlyBrace => { self.block(RuleArg::None) }
            TokenType::TokenMutable => { self.mut_var_def(RuleArg::None) }
            TokenType::TokenFunction => { self.function(RuleArg::None) }
            TokenType::TokenIf => { self.if_statement(RuleArg::None) }
            // TokenType::TokenTyping => {}
            _ => self.expression_statement(),
        }

        /*
        match self.get_current().get_ttype() {
            TokenType::TokenIdentifier => {
                let next = self.get_next();
                if next.is_some() && matches!(next.unwrap().get_ttype(), &TokenType::TokenDefine) {
                    self.advance();
                    self.variable_definition();
                } else if
                    next.is_some() &&
                    matches!(next.unwrap().get_ttype(), &TokenType::TokenAssign)
                {
                    self.advance();
                    self.variable_assignment()
                } else {
                    self.expression_statement();
                }
            }
            TokenType::TokenInt32 | TokenType::TokenBool => {
                while !self.is_at_end() && !self.is_at_expr_end() {
                    let next = self.get_next();
                    if let Some(next) = next {
                        if matches!(next.get_ttype(), &TokenType::TokenIdentifier) {
                            break;
                        }
                    } else {
                        break;
                    }

                    self.advance();
                }

                self.advance();
                self.advance();
                self.variable_definition();
            }
            TokenType::TokenMutable => {
                self.advance();

                match self.get_current().get_ttype() {
                    TokenType::TokenFunction => {
                        // Function tokens isn't supported in lexer yet
                        // self.function_definition();
                    }
                    TokenType::TokenInt32 | TokenType::TokenBool => {
                        while !self.is_at_end() && !self.is_at_expr_end() {
                            let next = self.get_next();
                            if let Some(next) = next {
                                if matches!(next.get_ttype(), &TokenType::TokenIdentifier) {
                                    break;
                                }
                            } else {
                                break;
                            }

                            self.advance();
                        }

                        self.advance();
                        self.advance();
                        self.variable_definition();
                    }
                    TokenType::TokenIdentifier => {
                        let next = self.get_next();

                        if
                            next.is_some() &&
                            matches!(next.unwrap().get_ttype(), &TokenType::TokenDefine)
                        {
                            self.advance();
                            self.variable_definition();
                        } else {
                            self.expression_statement();
                        }
                    }
                    _ => {
                        while
                            !self.is_at_end() &&
                            !matches!(self.get_current().get_ttype(), &TokenType::TokenDefine)
                        {
                            self.advance();
                        }

                        if self.get_current().get_ttype() == &TokenType::TokenDefine {
                            self.variable_definition();
                        }
                    }
                }
            }
            TokenType::TokenLeftCurlyBrace => {
                self.start_scope();
                self.advance();
                while
                    !self.is_at_end() &&
                    !matches!(self.get_current().get_ttype(), &TokenType::TokenRightCurlyBrace)
                {
                    self.statement();
                }
                self.consume(TokenType::TokenRightCurlyBrace, "Expected '}' at the end of block");
                self.end_scope()
            }
            _ => { self.expression_statement() }
        }
        */
    }

    pub(super) fn var_assign(&mut self) -> Result<Stmt, CompileError> {
        let (lexeme, token_metadata) = {
            let token = self.get_previous();
            (token.get_lexeme(self.source), token.get_metadata())
        };

        let ident_lookup = AstIdentifier::new(lexeme, token_metadata);

        self.consume(
            TokenType::TokenAssign,
            format!(
                "Expected '=' in variable assignment but got '{}'",
                self.get_current().get_lexeme(self.source)
            ).as_str()
        )?;

        let value = self.parse_precedence(Precedence::PrecAssignment.get_next(), None)?;

        Ok(Stmt::VarAssignStmt(VarAssignStmt { field: ident_lookup, target_expr: None, value }))
    }

    pub(super) fn var_def(&mut self, rule_arg: RuleArg) -> Result<Stmt, CompileError> {
        let is_mutable = rule_arg == RuleArg::MutVar;

        let (lexeme, token_metadata) = {
            let token = self.get_previous();
            (token.get_lexeme(self.source), token.get_metadata())
        };

        let found_type = match self.resolve_type() {
            Ok(found_type) => found_type,
            Err(_) => None,
        };

        let last_token_ind_definition = self.get_previous().get_metadata();

        if !self.is_at_expr_end() {
            if
                self.consume(
                    TokenType::TokenDefine,
                    format!(
                        "Expected ':=' in variable definition but got '{}'",
                        self.get_current().get_lexeme(self.source)
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
            identifier_metadata,
            found_type,
            is_mutable,
            last_token_ind_definition
        );

        if let Err((message, token_vec)) = result {
            self.report_compile_error(message, token_vec);
        }
    }

    pub(super) fn resolve_type(&mut self) -> Result<Option<ValueType>, Vec<TokenMetadata>> {
        let found_type = match self.get_current().get_ttype() {
            TokenType::TokenIdentifier => {
                self.advance();
                let type_lexeme = self.get_previous().get_lexeme(&self.source);
                match type_lexeme.as_str() {
                    "i32" => Ok(Some(ValueType::Int32)),
                    "bool" => Ok(Some(ValueType::Bool)),
                    _ => Ok(None), // This should make a custom type
                }
            }
            _ => Err(vec![self.get_current().get_metadata()]),
        };

        found_type
    }

    pub(super) fn expression_statement(&mut self) -> Result<Stmt, CompileError> {
        let expr = self.expression()?;

        self.consume_expr_end();

        Ok(Stmt::ExprStmt(expr))

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

    pub(super) fn expression(&mut self) -> Result<Expr, CompileError> {
        self.parse_precedence(Precedence::PrecAssignment, None)
    }

    pub(super) fn parse_precedence(
        &mut self,
        precedence: Precedence,
        msg: Option<&str>
    ) -> Result<Expr, CompileError> {
        let mut expr_builder = ExprBuilder::new();

        self.advance();

        let parse_rule = self.get_parse_rule(self.get_previous().get_ttype());

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            let parse_method = prefix_rule(self, &mut expr_builder)?;

            loop {
                let current_ttype = self.get_current().get_ttype();
                let current_precedence = self.get_parse_rule(current_ttype).get_precedence();

                if (*current_precedence as usize) < (precedence as usize) {
                    break;
                }

                if self.get_previous().get_line() < self.get_current().get_line() {
                    break;
                }

                self.advance();

                let infix_rule = self.get_parse_rule(self.get_previous().get_ttype()).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self, &mut expr_builder);
                }
            }
        } else {
            return Err(
                CompileError::new(
                    format!(
                        "Unexpected token: '{}' (no prefix rule)",
                        self.get_previous().get_lexeme(self.source)
                    ),
                    vec![self.get_previous().get_metadata()]
                )
            );
        }

        Ok(expr_builder.get_built_expr())
    }

    pub(super) fn ident_lookup(
        &mut self,
        expr_builder: &mut ExprBuilder
    ) -> Result<(), CompileError> {
        let token = self.get_previous();
        let lexeme = token.get_lexeme(self.source);

        expr_builder.emit_ident_lookup(AstIdentifier::new(lexeme, token.get_metadata()));

        Ok(())
    }

    pub(super) fn resolve_function_args(&mut self) -> Result<Vec<FunctionArgument>, CompileError> {
        let mut args = vec![];

        self.consume(TokenType::TokenLeftParen, "Expected '(' after function identifier");

        while !self.is_at_expr_end() {
            if self.get_current().get_ttype().is(&TokenType::TokenComma) {
                self.advance();
            }
            if self.get_current().get_ttype().is(&TokenType::TokenRightParen) {
                break;
            }

            self.consume(
                TokenType::TokenIdentifier,
                format!(
                    "Expected identifer but got '{}' Syntax: <identifier> <type>",
                    self.get_current().get_lexeme(self.source)
                ).as_str()
            )?;

            let ident_lexeme = self.get_previous().get_lexeme(&self.source);

            let is_mutable = match self.get_current().get_ttype().is(&TokenType::TokenMutable) {
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
                    return Err(CompileError::new("Invalid type".to_string(), error_tokens));
                }
            };

            args.push(FunctionArgument {
                is_mutable,
                name: ident_lexeme,
                value_type: arg_type,
            });
        }

        self.consume(TokenType::TokenRightParen, "Expected a closing ')' after function arguments");

        Ok(args)
    }

    // Temporary functions
    fn consume_type(&mut self, msg: &str) -> Option<&TokenType> {
        if matches!(self.get_current().get_ttype(), TokenType::TokenInt32 | TokenType::TokenBool) {
            Some(self.get_current().get_ttype())
        } else {
            self.report_compile_error(msg.to_string(), vec![self.get_current().get_metadata()]);
            None
        }
    }

    fn consume_identifier(&mut self, msg: &str) -> Option<String> {
        if matches!(self.get_current().get_ttype(), TokenType::TokenIdentifier) {
            Some(self.get_current().get_lexeme(self.source))
        } else {
            self.report_compile_error(msg.to_string(), vec![self.get_current().get_metadata()]);
            None
        }
    }

    pub(super) fn resolve_function_return_type(&mut self) -> Result<Option<ValueType>, ()> {
        let return_type = match self.get_current().get_ttype() {
            TokenType::TokenLeftCurlyBrace => {
                return Ok(None);
            }
            TokenType::TokenIdentifier =>
                match self.resolve_type() {
                    Ok(return_type) => return_type,
                    Err(error_tokens) => {
                        self.report_compile_error(
                            "Invalid function return type".to_string(),
                            error_tokens
                        );
                        return Err(());
                    }
                }
            _ => {
                self.report_compile_error(
                    format!(
                        "Expected function return type, but got: {}",
                        self.get_current().get_lexeme(self.source)
                    ),
                    vec![self.get_current().get_metadata()]
                );
                return Err(());
            }
        };

        Ok(return_type)
    }

    pub(super) fn synchronize(&mut self) {
        self.exit_panic_mode();

        while !self.is_at_end() {
            if self.get_previous().get_ttype() == &TokenType::TokenSemicolon {
                return;
            }
            if self.get_previous().get_line() < self.get_current().get_line() {
                // This might need to change, as soon as blocks are implemented
                return;
            }

            match self.get_current().get_ttype() {
                TokenType::TokenError => {
                    let current = self.get_current();
                    self.report_compile_error(
                        current.get_message().unwrap().to_string(),
                        vec![current.get_metadata()]
                    );
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
            TokenType::TokenInt32 => Typing::new_int32(token_metadata),
            TokenType::TokenBool => Typing::new_bool(token_metadata),
            TokenType::TokenIdentifier =>
                Typing::new_custom(current_token.get_lexeme(self.source), token_metadata),
            _ => {
                self.report_compile_error(
                    format!(
                        "Unexpected '{}' cannot be used as a type",
                        current_token.get_lexeme(self.source)
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
        //     TokenType::TokenIdentifier => crate::value_v2::ValueType::
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
                        matches!(token.get_ttype(), TokenType::TokenEOF | TokenType::TokenSemicolon)
                    {
                        return (None, None, search_index);
                    }

                    if token.get_ttype() == &TokenType::TokenInt32 {
                        return (Some(ValueType::Int32), Some(token.clone()), search_index);
                    } else if token.get_ttype() == &TokenType::TokenBool {
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
            Some(token) => (token.get_ttype() == &TokenType::TokenMutable, start_search_index),
            None => (false, start_search_index),
        }
    }

pub(super) fn variable_definition(&mut self) {
        let token = self.get_previous().clone();

        let lexeme = token.get_lexeme(self.source);

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
                    TokenType::TokenDefine,
                    format!(
                        "Expected ':=' in variable definition but got '{}'",
                        self.get_current().get_lexeme(self.source)
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
