use crate::{
    compiler::{
        ds::{ value::ValueType },
        error_handler::{ CompileError, ReportedError },
        ir::ast::{
            expr::IdentifierExpr,
            stmt::{
                BlockStmt,
                BreakStmt,
                ContinueStmt,
                ExprStmt,
                FunctionStmt,
                IfStmt,
                LoopStmt,
                ReturnStmt,
                Stmt,
                VarAssignStmt,
                VarDefStmt,
            },
            AstArena,
        },
        ProgramSymbolTablePhase1,
    },
    macros::merge_chars_range,
};

use super::{
    parser_macros::{ current, next, previous },
    precedence::Precedence,
    Parser,
    StmtMethodArgs,
    StmtMethodRetType,
    TokenType::{ self, * },
};

impl<'a> Parser<'a> {
    pub(super) fn statement<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        let curr = current!(self, ttype);

        match curr {
            TokenLeftCurlyBrace =>
                Ok(Stmt::BlockStmt(self.block((program_symbol_table, ast_arena), None)?)),
            TokenMutable => self.mut_var_def((program_symbol_table, ast_arena)),
            TokenFunction => self.function((program_symbol_table, ast_arena)),
            TokenIf => Ok(Stmt::IfStmt(self.if_stmt((program_symbol_table, ast_arena))?)),
            TokenLoop => self.loop_stmt((program_symbol_table, ast_arena)),
            TokenWhile => self.while_stmt((program_symbol_table, ast_arena)),
            TokenBreak => self.break_stmt((program_symbol_table, ast_arena)),
            TokenContinue => self.continue_stmt((program_symbol_table, ast_arena)),
            TokenReturn => self.return_stmt((program_symbol_table, ast_arena)),
            TokenDef => self.function_v2((program_symbol_table, ast_arena)),
            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_stmt(TokenAssign) => {
                self.var_assign((program_symbol_table, ast_arena))
            }
            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_stmt(TokenDefine) => {
                self.var_def((program_symbol_table, ast_arena))
            }

            _ => self.expression_statement((program_symbol_table, ast_arena)),
        }
    }

    pub(super) fn expression_statement<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        let expr = self.expression(Precedence::PrecAssignment.get_next(), (
            program_symbol_table,
            ast_arena,
        ))?;

        self.consume_expr_end()?;

        Ok(Stmt::ExprStmt(ExprStmt::new(expr)))
    }

    pub(super) fn return_stmt<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        let metadata = current!(self, metadata);

        self.advance();

        let return_expr = if !self.is_at_expr_end() {
            let return_expr = self.expression(Precedence::PrecAssignment.get_next(), (
                program_symbol_table,
                ast_arena,
            ))?;
            Some(ExprStmt::new(return_expr))
        } else {
            None
        };

        self.consume_expr_end()?;

        Ok(Stmt::ReturnStmt(ReturnStmt::new(return_expr, metadata)))
    }

    pub(super) fn continue_stmt<'b, 'c>(
        &mut self,
        _: StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        self.advance();
        self.consume_expr_end()?;

        Ok(Stmt::ContinueStmt(ContinueStmt::new()))
    }

    pub(super) fn break_stmt<'b, 'c>(
        &mut self,
        _: StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        let metadata = current!(self, metadata);
        self.advance();
        self.consume_expr_end()?;

        Ok(Stmt::BreakStmt(BreakStmt::new(metadata)))
    }

    pub(super) fn while_stmt<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        self.advance();

        let condition = ExprStmt::new(
            self.expression(Precedence::PrecAssignment, (program_symbol_table, ast_arena))?
        );

        let body = self.block((program_symbol_table, ast_arena), None)?;

        self.consume_expr_end()?;

        Ok(Stmt::LoopStmt(LoopStmt::new(Some(condition), body)))
    }

    pub(super) fn loop_stmt<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        self.advance();

        let body = self.block_v2((program_symbol_table, ast_arena), None)?;

        self.consume(TokenType::TokenEnd, "Expected end token")?;

        self.consume_expr_end()?;

        println!("current: {:?}", current!(self, ttype));

        Ok(Stmt::LoopStmt(LoopStmt::new(None, body)))
    }

    pub(super) fn var_assign<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        // Temporary block start (until chained assignments is supported: struct.value = some_value)
        self.advance();
        let (lexeme, token_metadata) = previous!(self, lexeme, metadata);
        // Temporary block end

        // let target_expr = ExprStmt::new(self.expression(Precedence::PrecCall, arena)?);

        if !current!(self, ttype).is(&TokenAssign) {
            let mut token_vec = vec![previous!(self, metadata)];
            while !current!(self, ttype).is(&TokenAssign) {
                token_vec.push(current!(self, metadata));
                self.advance();
            }

            return Err(
                CompileError::new(
                    ReportedError::new(
                        "Invalid left-hand side of assignment".to_string(),
                        merge_chars_range!(
                            token_vec
                                .iter()
                                .map(|m| (*m).into())
                                .collect()
                        )
                    )
                )
            );
        }

        self.advance();

        let value = ExprStmt::new(
            self.expression(Precedence::PrecAssignment.get_next(), (
                program_symbol_table,
                ast_arena,
            ))?
        );

        self.consume_expr_end()?;

        let ssa_ident = program_symbol_table.declare_var_ssa_ident(&lexeme);

        Ok(
            Stmt::VarAssignStmt(
                VarAssignStmt::new(IdentifierExpr::new(ssa_ident, token_metadata), value)
            )
        )
    }

    pub(super) fn var_def<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        let mut_keyword_metadata = if let TokenMutable = previous!(self, ttype) {
            Some(previous!(self, metadata))
        } else {
            None
        };

        self.advance();

        let (lexeme, token_metadata) = previous!(self, lexeme, metadata);

        let found_type = match self.resolve_type() {
            Ok(found_type) => { found_type }
            Err(_) => None,
        };

        let value = if !self.is_at_expr_end() {
            self.consume(
                TokenDefine,
                format!(
                    "Expected ':=' in variable definition but got '{}'",
                    current!(self, lexeme).get_lexeme_str()
                ).as_str()
            )?;

            if self.is_at_expr_end() {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            "Missing right hand side of variable definition".to_string(),
                            previous!(self, metadata).into()
                        )
                    )
                );
            }
            let value = self.expression(Precedence::PrecAssignment.get_next(), (
                program_symbol_table,
                ast_arena,
            ))?;

            Some(ExprStmt::new(value))
        } else {
            None
        };

        self.consume_expr_end()?;

        let ssa_ident = program_symbol_table.declare_var_ssa_ident(&lexeme);

        Ok(
            Stmt::VarDefStmt(
                VarDefStmt::new(
                    IdentifierExpr::new(ssa_ident, token_metadata),
                    found_type,
                    mut_keyword_metadata,
                    value
                )
            )
        )
    }

    pub fn mut_var_def<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        self.advance();

        match current!(self, ttype) {
            TokenIdentifier => self.var_def((program_symbol_table, ast_arena)),
            TokenFunction => {
                panic!("Functions cannot be mutable");
            }
            _ => panic!("Unexpected: {}", current!(self, lexeme).get_lexeme_str()),
        }
    }

    pub fn block_v2<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>,
        return_type: Option<ValueType>
    ) -> Result<BlockStmt<'b>, CompileError> {
        let prev_symbol_table_id = program_symbol_table.get_current_symbol_table_id();

        let new_symbol_table_id = program_symbol_table.new_symbol_table(
            Some(prev_symbol_table_id),
            return_type
        );

        let mut scope_stmt = BlockStmt::new(new_symbol_table_id);

        while !self.is_at_end() && !current!(self, ttype).can_terminate_block() {
            match self.statement((program_symbol_table, ast_arena)) {
                Ok(stmt) => {
                    match scope_stmt.push_stmt(stmt, program_symbol_table) {
                        Ok(_) => {}
                        Err(err) => self.error_handler.report_compile_error(err),
                    }
                }
                Err(e) => {
                    self.error_handler.report_compile_error(e);
                }
            }
        }

        program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id);

        Ok(scope_stmt)
    }

    pub fn block<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>,
        return_type: Option<ValueType>
    ) -> Result<BlockStmt<'b>, CompileError> {
        println!("DON'T RUN");
        self.consume(
            TokenType::TokenLeftCurlyBrace,
            format!("Expected '{{' but got: {}", current!(self, lexeme).get_lexeme_str()).as_str()
        )?;

        let current_symbol_table_id = program_symbol_table.get_current_symbol_table_id();

        let new_symbol_table_id = program_symbol_table.new_symbol_table(
            Some(current_symbol_table_id),
            return_type
        );

        let mut scope_stmt = BlockStmt::new(new_symbol_table_id);

        while
            !self.is_at_end() &&
            !matches!(current!(self, ttype), &TokenType::TokenRightCurlyBrace)
        {
            match self.statement((program_symbol_table, ast_arena)) {
                Ok(stmt) => {
                    match scope_stmt.push_stmt(stmt, program_symbol_table) {
                        Ok(_) => {}
                        Err(err) => self.error_handler.report_compile_error(err),
                    }
                }
                Err(e) => {
                    self.error_handler.report_compile_error(e);
                }
            }
        }

        self.consume(TokenType::TokenRightCurlyBrace, "Expected '}' at the end of block")?;

        Ok(scope_stmt)
    }

    pub fn if_stmt<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> Result<IfStmt<'b>, CompileError> {
        self.advance();

        let condition = ExprStmt::new(
            self.expression(Precedence::PrecAssignment, (program_symbol_table, ast_arena))?
        );

        self.consume(TokenType::TokenDo, "Expected do token")?;

        let true_block = self.block_v2((program_symbol_table, ast_arena), None)?;

        let false_block = if current!(self, ttype).is(&TokenType::TokenElif) {
            let if_stmt = self.if_stmt((program_symbol_table, ast_arena))?;
            Some(ast_arena.alloc_if_stmt(if_stmt))
        } else if current!(self, ttype).is(&TokenType::TokenElse) {
            self.advance();
            let true_block = self.block_v2((program_symbol_table, ast_arena), None)?;
            self.consume(TokenType::TokenEnd, "Expected end token")?;
            let if_stmt = IfStmt::new(None, true_block, None);
            Some(ast_arena.alloc_if_stmt(if_stmt))
        } else if current!(self, ttype).is(&TokenType::TokenEnd) {
            self.advance();
            None
        } else {
            None
        };

        Ok(IfStmt::new(Some(condition), true_block, false_block))
    }

    pub fn function_v2<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        self.advance();

        let (lexeme, metadata) = current!(self, lexeme, metadata);

        let prev_symbol_table_id = program_symbol_table.get_current_symbol_table_id();
        // program_symbol_table.set_current_symbol_table_id(
        //     program_symbol_table.get_next_symbol_table_id()
        // );

        let ssa_ident = program_symbol_table.declare_fn_ssa_ident(&lexeme);

        self.advance();

        let function_args = match self.resolve_function_args(program_symbol_table) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let return_type = match current!(self, ttype) {
            TokenType::TokenIdentifier => {
                match current!(self, lexeme).get_lexeme_str() {
                    "Int" => {
                        self.advance();
                        ValueType::Int
                    }
                    "Bool" => {
                        self.advance();
                        ValueType::Bool
                    }
                    _ => ValueType::Void,
                }
            }
            _ => ValueType::Void,
        };

        // program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id);
        let body = self.block_v2((program_symbol_table, ast_arena), Some(return_type.clone()))?;

        let function_stmt = FunctionStmt::new(
            ssa_ident,
            function_args,
            body,
            return_type,
            metadata
        );

        self.consume(TokenType::TokenEnd, "Expected end after function declaration")?;

        program_symbol_table.set_current_symbol_table_id(prev_symbol_table_id);

        Ok(Stmt::FunctionStmt(function_stmt))
    }

    pub fn function<'b, 'c>(
        &mut self,
        (program_symbol_table, ast_arena): StmtMethodArgs<'b, 'c>
    ) -> StmtMethodRetType<'b> {
        self.advance();

        let (lexeme, metadata) = current!(self, lexeme, metadata);

        program_symbol_table.set_current_symbol_table_id(
            program_symbol_table.get_current_symbol_table_id() + 1
        );

        let ssa_ident = program_symbol_table.declare_fn_ssa_ident(&lexeme);

        self.advance();

        let function_args = match self.resolve_function_args(program_symbol_table) {
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

        let body = self.block((program_symbol_table, ast_arena), Some(return_type.clone()))?;

        let function_stmt = FunctionStmt::new(
            ssa_ident,
            function_args,
            body,
            return_type,
            metadata
        );

        Ok(Stmt::FunctionStmt(function_stmt))
    }
}
