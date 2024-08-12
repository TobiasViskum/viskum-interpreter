use crate::{
    compiler::{
        ds::{ symbol_table::SymbolTableRef, value::ValueType },
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
        traits::SymbolTableAlloc,
    },
    macros::merge_chars_range,
};

use super::{
    parser_macros::{ current, previous },
    precedence::Precedence,
    Parser,
    TokenType::{ self, * },
};

type ReturnType<'b> = Result<Stmt<'b>, CompileError>;
type Args<'b> = (&'b AstArena<'b>, SymbolTableRef);

impl<'a> Parser<'a> {
    pub(super) fn statement<'b>(&mut self, (arena, symbol_table_ref): Args<'b>) -> ReturnType<'b> {
        let curr = current!(self, ttype);

        match curr {
            TokenLeftCurlyBrace =>
                Ok(Stmt::BlockStmt(self.block((arena, symbol_table_ref), None)?)),
            TokenMutable => self.mut_var_def((arena, symbol_table_ref)),
            TokenFunction => self.function((arena, symbol_table_ref)),
            TokenIf => Ok(Stmt::IfStmt(self.if_stmt((arena, symbol_table_ref))?)),
            TokenLoop => self.loop_stmt((arena, symbol_table_ref)),
            TokenWhile => self.while_stmt((arena, symbol_table_ref)),
            TokenBreak => self.break_stmt((arena, symbol_table_ref)),
            TokenContinue => self.continue_stmt((arena, symbol_table_ref)),
            TokenReturn => self.return_stmt((arena, symbol_table_ref)),

            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_stmt(TokenAssign) => {
                self.var_assign((arena, symbol_table_ref))
            }
            _ if curr.is(&TokenIdentifier) && self.is_ttype_in_stmt(TokenDefine) => {
                self.var_def((arena, symbol_table_ref), false)
            }

            _ => self.expression_statement((arena, symbol_table_ref)),
        }
    }

    pub(super) fn expression_statement<'b>(&mut self, (arena, _): Args<'b>) -> ReturnType<'b> {
        let expr = self.expression(Precedence::PrecAssignment.get_next(), arena)?;

        self.consume_expr_end()?;

        Ok(Stmt::ExprStmt(ExprStmt::new(expr)))
    }

    pub(super) fn return_stmt<'b>(&mut self, (arena, _): Args<'b>) -> ReturnType<'b> {
        let metadata = current!(self, metadata);

        self.advance();

        let return_expr = if !self.is_at_expr_end() {
            Some(ExprStmt::new(self.expression(Precedence::PrecAssignment.get_next(), arena)?))
        } else {
            None
        };

        self.consume_expr_end()?;

        Ok(Stmt::ReturnStmt(ReturnStmt::new(return_expr, metadata)))
    }

    pub(super) fn continue_stmt<'b>(&mut self, _: Args<'b>) -> ReturnType<'b> {
        self.advance();
        self.consume_expr_end()?;

        Ok(Stmt::ContinueStmt(ContinueStmt::new()))
    }

    pub(super) fn break_stmt<'b>(&mut self, _: Args<'b>) -> ReturnType<'b> {
        let metadata = current!(self, metadata);
        self.advance();
        self.consume_expr_end()?;

        Ok(Stmt::BreakStmt(BreakStmt::new(metadata)))
    }

    pub(super) fn while_stmt<'b>(&mut self, (arena, symbol_table_ref): Args<'b>) -> ReturnType<'b> {
        self.advance();

        let condition = ExprStmt::new(self.expression(Precedence::PrecAssignment, arena)?);

        let body = self.block((arena, symbol_table_ref), None)?;

        self.consume_expr_end()?;

        Ok(Stmt::LoopStmt(LoopStmt::new(Some(condition), body)))
    }

    pub(super) fn loop_stmt<'b>(&mut self, (arena, symbol_table_ref): Args<'b>) -> ReturnType<'b> {
        self.advance();

        let body = self.block((arena, symbol_table_ref), None)?;

        self.consume_expr_end()?;

        println!("current: {:?}", current!(self, ttype));

        Ok(Stmt::LoopStmt(LoopStmt::new(None, body)))
    }

    pub(super) fn var_assign<'b>(&mut self, (arena, symbol_table_ref): Args<'b>) -> ReturnType<'b> {
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

        let value = ExprStmt::new(self.expression(Precedence::PrecAssignment.get_next(), arena)?);

        self.consume_expr_end()?;

        Ok(
            Stmt::VarAssignStmt(
                VarAssignStmt::new(
                    IdentifierExpr::new(lexeme.take_lexeme_rc(), token_metadata),
                    value
                )
            )
        )
    }

    pub(super) fn var_def<'b>(&mut self, (arena, _): Args<'b>, is_mutable: bool) -> ReturnType<'b> {
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
            let value = self.expression(Precedence::PrecAssignment.get_next(), arena)?;

            Some(ExprStmt::new(value))
        } else {
            None
        };

        self.consume_expr_end()?;

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

    pub fn mut_var_def<'b>(&mut self, (arena, symbol_table_ref): Args<'b>) -> ReturnType<'b> {
        self.advance();

        match current!(self, ttype) {
            TokenIdentifier => self.var_def((arena, symbol_table_ref), true),
            TokenFunction => {
                panic!("Functions cannot be mutable");
            }
            _ => panic!("Unexpected: {}", current!(self, lexeme).get_lexeme_str()),
        }
    }

    pub fn block<'b>(
        &mut self,
        (arena, mut symbol_table_ref): Args<'b>,
        return_type: Option<ValueType>
    ) -> Result<BlockStmt<'b>, CompileError> {
        self.consume(
            TokenType::TokenLeftCurlyBrace,
            format!("Expected '{{' but got: {}", current!(self, lexeme).get_lexeme_str()).as_str()
        )?;

        let symbol_table_ref = symbol_table_ref.alloc_symbol_table(return_type);

        let mut scope_stmt = BlockStmt::new(symbol_table_ref, true);

        while
            !self.is_at_end() &&
            !matches!(current!(self, ttype), &TokenType::TokenRightCurlyBrace)
        {
            match self.statement((arena, symbol_table_ref)) {
                Ok(stmt) => scope_stmt.push_stmt(stmt),
                Err(e) => {
                    self.error_handler.report_compile_error(e);
                }
            }
        }

        self.consume(TokenType::TokenRightCurlyBrace, "Expected '}' at the end of block")?;

        Ok(scope_stmt)
    }

    pub fn if_stmt<'b>(
        &mut self,
        (arena, symbol_table_ref): Args<'b>
    ) -> Result<IfStmt<'b>, CompileError> {
        self.advance();

        let condition = ExprStmt::new(self.expression(Precedence::PrecAssignment, arena)?);

        let true_block = self.block((arena, symbol_table_ref), None)?;

        let false_block = if current!(self, ttype).is(&TokenType::TokenElse) {
            self.advance();
            let if_stmt = if current!(self, ttype).is(&TokenType::TokenIf) {
                self.if_stmt((arena, symbol_table_ref))?
            } else {
                let true_block = self.block((arena, symbol_table_ref), None)?;
                IfStmt::new(None, true_block, None)
            };
            Some(arena.alloc_if_stmt(if_stmt))
        } else {
            None
        };

        Ok(IfStmt::new(Some(condition), true_block, false_block))
    }

    pub fn function<'b>(&mut self, (arena, symbol_table_ref): Args<'b>) -> ReturnType<'b> {
        self.advance();

        let (lexeme, metadata) = current!(self, lexeme, metadata);

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

        let body = self.block((arena, symbol_table_ref), Some(return_type))?;

        let function_stmt = FunctionStmt::new(
            lexeme.take_lexeme_rc(),
            function_args,
            body,
            metadata
        );

        Ok(Stmt::FunctionStmt(function_stmt))
    }
}
