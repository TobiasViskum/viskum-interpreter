use std::rc::Rc;

use crate::{
    ast::ast_symbol_table::AstSymbolTable,
    error_handler::ErrorHandler,
    parser::token::TokenMetadata,
    value::ValueType,
    Dissasemble,
};

use super::{ ExprStmt, StmtTrait };

#[derive(Debug)]
pub struct VarDefStmt {
    pub name: Rc<str>,
    pub value_type: Option<ValueType>,
    pub is_mutable: bool,
    pub value: Option<ExprStmt>,
    pub token_metadata: TokenMetadata,
}

impl VarDefStmt {
    pub fn new(
        name: Rc<str>,
        value_type: Option<ValueType>,
        is_mutable: bool,
        value: Option<ExprStmt>,
        token_metadata: TokenMetadata
    ) -> Self {
        Self {
            name,
            value_type,
            is_mutable,
            value,
            token_metadata,
        }
    }
}

impl Dissasemble for VarDefStmt {
    fn dissasemble(&self) -> String {
        let mutable_string = match self.is_mutable {
            true => "mut ".to_string(),
            false => "".to_string(),
        };

        let value_string = match &self.value {
            Some(value) => format!(" := {}", value.dissasemble()),
            None => "".to_string(),
        };

        match &self.value_type {
            Some(value_type) => {
                format!(
                    "{}{}{}{}\n",
                    mutable_string,
                    self.name,
                    value_type.dissasemble(),
                    value_string
                )
            }
            None => { format!("{}{}{}\n", mutable_string, self.name, value_string) }
        }
    }
}

impl StmtTrait for VarDefStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        ast_symbol_table.declare_variable(self, error_handler)
    }
}
