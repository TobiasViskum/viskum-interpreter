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
    token::{ token_type::TokenType::{ self, * }, Token, TokenMetadata },
    ParseMethod,
    Parser,
    RuleArg,
};

impl<'a> Parser<'a> {
    pub(super) fn statement(&mut self) -> Result<Stmt, CompileError> {
        let curr = self.get_current().get_ttype();

        match curr {
            TokenLeftCurlyBrace => { Ok(Stmt::ScopeStmt(self.block()?)) }
            TokenMutable => { self.mut_var_def() }
            TokenFunction => { self.function() }
            TokenIf => { Ok(Stmt::IfStmt(self.if_statement()?)) }

            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_line(TokenAssign) => {
                self.var_assign()
            }
            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_line(TokenDefine) => {
                self.var_def(false)
            }

            // TokenTyping => {}
            _ => self.expression_statement(),
        }

        /*
        match self.get_current().get_ttype() {
            TokenIdentifier => {
                let next = self.get_next();
                if next.is_some() && matches!(next.unwrap().get_ttype(), &TokenDefine) {
                    self.advance();
                    self.variable_definition();
                } else if
                    next.is_some() &&
                    matches!(next.unwrap().get_ttype(), &TokenAssign)
                {
                    self.advance();
                    self.variable_assignment()
                } else {
                    self.expression_statement();
                }
            }
            TokenInt32 | TokenBool => {
                while !self.is_at_end() && !self.is_at_expr_end() {
                    let next = self.get_next();
                    if let Some(next) = next {
                        if matches!(next.get_ttype(), &TokenIdentifier) {
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
            TokenMutable => {
                self.advance();

                match self.get_current().get_ttype() {
                    TokenFunction => {
                        // Function tokens isn't supported in lexer yet
                        // self.function_definition();
                    }
                    TokenInt32 | TokenBool => {
                        while !self.is_at_end() && !self.is_at_expr_end() {
                            let next = self.get_next();
                            if let Some(next) = next {
                                if matches!(next.get_ttype(), &TokenIdentifier) {
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
                    TokenIdentifier => {
                        let next = self.get_next();

                        if
                            next.is_some() &&
                            matches!(next.unwrap().get_ttype(), &TokenDefine)
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
                            !matches!(self.get_current().get_ttype(), &TokenDefine)
                        {
                            self.advance();
                        }

                        if self.get_current().get_ttype() == &TokenDefine {
                            self.variable_definition();
                        }
                    }
                }
            }
            TokenLeftCurlyBrace => {
                self.start_scope();
                self.advance();
                while
                    !self.is_at_end() &&
                    !matches!(self.get_current().get_ttype(), &TokenRightCurlyBrace)
                {
                    self.statement();
                }
                self.consume(TokenRightCurlyBrace, "Expected '}' at the end of block");
                self.end_scope()
            }
            _ => { self.expression_statement() }
        }
        */
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
        let target_expr = self.expression(Precedence::PrecCall)?;

        self.consume(
            TokenAssign,
            format!(
                "Expected '=' in variable assignment but got '{}'",
                self.get_current().get_lexeme(self.source)
            ).as_str()
        )?;

        let value = self.expression(Precedence::PrecAssignment.get_next())?;

        Ok(Stmt::VarAssignStmt(VarAssignStmt { target_expr, value }))
    }

    pub(super) fn var_def(&mut self, is_mutable: bool) -> Result<Stmt, CompileError> {
        self.advance();

        let (lexeme, token_metadata) = {
            let token = self.get_previous();
            (token.get_lexeme(self.source), token.get_metadata())
        };

        let found_type = match self.resolve_type() {
            Ok(found_type) => found_type,
            Err(_) => None,
        };

        let value = if found_type.is_none() {
            self.consume(
                TokenDefine,
                format!(
                    "Expected ':=' in variable definition but got '{}'",
                    self.get_current().get_lexeme(self.source)
                ).as_str()
            )?;

            if self.is_at_expr_end() {
                return Err(
                    CompileError::new(
                        "Missing right hand side of variable definition".to_string(),
                        vec![self.get_previous().get_metadata()]
                    )
                );
            }
            let value = self.expression(Precedence::PrecAssignment.get_next())?;

            Some(value)
        } else {
            None
        };

        Ok(Stmt::VarDefStmt(VarDefStmt::new(lexeme, found_type, is_mutable, value, token_metadata)))

        /*
        if !self.is_at_expr_end() {
            if
                self.consume(
                    TokenDefine,
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
            last_token_in_definition
        );

        if let Err((message, token_vec)) = result {
            self.report_compile_error(message, token_vec);
        }
        */
    }

    pub(super) fn resolve_type(&mut self) -> Result<Option<ValueType>, Vec<TokenMetadata>> {
        let found_type = match self.get_current().get_ttype() {
            TokenIdentifier => {
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
        let expr = self.expression(Precedence::PrecAssignment.get_next())?;

        self.consume_expr_end()?;

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
                        self.get_previous().get_lexeme(self.source)
                    ),
                    vec![self.get_previous().get_metadata()]
                )
            );
        }

        Ok(())
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

        self.consume(TokenLeftParen, "Expected '(' after function identifier");

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
                    self.get_current().get_lexeme(self.source)
                ).as_str()
            )?;

            let ident_lexeme = self.get_previous().get_lexeme(&self.source);

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
                    return Err(CompileError::new("Invalid type".to_string(), error_tokens));
                }
            };

            args.push(FunctionArgument {
                is_mutable,
                name: ident_lexeme,
                value_type: arg_type,
            });
        }

        self.consume(TokenRightParen, "Expected a closing ')' after function arguments");

        Ok(args)
    }

    // Temporary functions
    fn consume_type(&mut self, msg: &str) -> Option<&TokenType> {
        if matches!(self.get_current().get_ttype(), TokenInt32 | TokenBool) {
            Some(self.get_current().get_ttype())
        } else {
            self.report_compile_error(msg.to_string(), vec![self.get_current().get_metadata()]);
            None
        }
    }

    fn consume_identifier(&mut self, msg: &str) -> Option<String> {
        if matches!(self.get_current().get_ttype(), TokenIdentifier) {
            Some(self.get_current().get_lexeme(self.source))
        } else {
            self.report_compile_error(msg.to_string(), vec![self.get_current().get_metadata()]);
            None
        }
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
                            self.get_previous().get_lexeme(self.source)
                        ),
                        vec![self.get_previous().get_metadata()]
                    )
                );
            }
        }

        Ok(())
    }

    pub(super) fn resolve_function_return_type(&mut self) -> Result<Option<ValueType>, ()> {
        let return_type = match self.get_current().get_ttype() {
            TokenLeftCurlyBrace => {
                return Ok(None);
            }
            TokenIdentifier =>
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
                    TokenDefine,
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
