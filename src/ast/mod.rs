use stmt::ScopeStmt;

use crate::{ error_handler::ErrorHandler, Dissasemble };
use self::ast_symbol_table::AstSymbolTable;
use crate::ast::stmt::StmtTrait;

mod generate_cfg;

pub mod expr;
pub mod stmt;
pub mod ast_symbol_table;

#[derive(Debug)]
pub struct Ast {
    main_scope: ScopeStmt,
    pub ast_symbol_table: AstSymbolTable,
}

impl Ast {
    pub fn new(main_scope: ScopeStmt) -> Self {
        Self {
            main_scope,
            ast_symbol_table: AstSymbolTable::new(),
        }
    }

    pub fn type_check_and_constant_fold(&mut self, error_handler: &mut ErrorHandler) {
        self.main_scope.validate_stmt(&mut self.ast_symbol_table, error_handler);
    }
}

impl Dissasemble for Ast {
    fn dissasemble(&self) -> String {
        let mut string_builder = "\n----- AST -----\n\n".to_string();
        string_builder += self.main_scope.dissasemble().as_str();
        string_builder += "\n---------------\n";
        string_builder
    }
}
