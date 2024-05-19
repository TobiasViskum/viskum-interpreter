use std::mem;

use crate::{
    error_handler::ErrorHandler,
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value::ValueType,
};

use self::{
    expr::Expr,
    stmt::{ FunctionArgument, FunctionStmt, ScopeStmt, Stmt },
    ast_symbol_table::AstSymbolTable,
};

mod generate_cfg;
pub mod expr;
pub mod stmt;
pub mod ast_symbol_table;

#[derive(Debug)]
pub struct Ast {
    pub main_scope: ScopeStmt,
    pub ast_symbol_table: AstSymbolTable,
}

impl Ast {
    pub fn new(main_scope: ScopeStmt) -> Self {
        Self {
            main_scope,
            ast_symbol_table: AstSymbolTable::new(),
        }
    }

    pub fn type_check(&mut self, error_handler: &mut ErrorHandler) {
        self.main_scope.type_check(&mut self.ast_symbol_table, error_handler);
    }
}
