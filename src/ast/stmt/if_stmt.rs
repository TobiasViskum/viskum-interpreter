use crate::{ ast::ast_symbol_table::AstSymbolTable, error_handler::ErrorHandler, Dissasemble };

use super::{ ExprStmt, ScopeStmt, StmtTrait };

#[derive(Debug)]
pub struct IfStmt {
    condition: Option<ExprStmt>,
    true_block: ScopeStmt,
    false_block: Option<Box<IfStmt>>,
}

impl IfStmt {
    pub fn new(
        condition: Option<ExprStmt>,
        true_block: ScopeStmt,
        false_block: Option<IfStmt>
    ) -> Self {
        let false_block = match false_block {
            Some(stmt) => Some(Box::new(stmt)),
            None => None,
        };

        Self {
            condition,
            true_block,
            false_block,
        }
    }
}

impl Dissasemble for IfStmt {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("");
        if let Some(condition) = &self.condition {
            string_builder += format!("if {} {{\n", condition.dissasemble()).as_str();

            string_builder += self.true_block.dissasemble().as_str();

            string_builder += "}\n";

            if let Some(false_block) = &self.false_block {
                string_builder += false_block.dissasemble().as_str();
            }
        } else {
            string_builder += "else {\n";
        }

        string_builder
    }
}

impl StmtTrait for IfStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        if let Some(condition) = &mut self.condition {
            condition.validate_stmt(ast_symbol_table, error_handler);
        }

        self.true_block.validate_stmt(ast_symbol_table, error_handler);

        if let Some(false_block) = &mut self.false_block {
            false_block.validate_stmt(ast_symbol_table, error_handler);
        }
    }
}
