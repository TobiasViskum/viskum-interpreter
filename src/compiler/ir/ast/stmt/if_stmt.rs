use crate::compiler::{ ds::symbol_table::SymbolTableRef, error_handler::ErrorHandler, Dissasemble };

use super::{ ExprStmt, LinearControlFlow, ScopeStmt, StmtTrait };

#[derive(Debug)]
pub struct IfStmt<'ast> {
    condition: Option<ExprStmt<'ast>>,
    true_block: ScopeStmt<'ast>,
    false_block: Option<Box<IfStmt<'ast>>>,
}

impl<'ast> IfStmt<'ast> {
    pub fn new(
        condition: Option<ExprStmt<'ast>>,
        true_block: ScopeStmt<'ast>,
        false_block: Option<IfStmt<'ast>>
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

impl<'ast> Dissasemble for IfStmt<'ast> {
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

impl<'ast> StmtTrait for IfStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        if let Some(condition) = &mut self.condition {
            condition.validate_stmt(symbol_table_ref, error_handler);
        }

        self.true_block.validate_stmt(symbol_table_ref, error_handler);

        if let Some(false_block) = &mut self.false_block {
            false_block.validate_stmt(symbol_table_ref, error_handler);
        }
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}
