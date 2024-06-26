use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::SymbolTableRef, value::ValueType },
    error_handler::ErrorHandler,
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::{ FunctionArgument, LinearControlFlow, ScopeStmt, StmtTrait };

#[derive(Debug)]
pub struct FunctionStmt<'ast> {
    name: Rc<str>,
    args: Vec<FunctionArgument>,
    body: ScopeStmt<'ast>,
    return_type: ValueType,
    metadata: TokenMetadata,
}

impl<'ast> FunctionStmt<'ast> {
    pub fn new(
        name: Rc<str>,
        args: Vec<FunctionArgument>,
        body: ScopeStmt<'ast>,
        metadata: TokenMetadata
    ) -> Self {
        let return_type = body
            .get_symbol_table_ref()
            .get()
            .get_fn_return_type()
            .cloned()
            .unwrap_or(ValueType::Void);

        Self {
            name,
            args,
            body,
            return_type,
            metadata,
        }
    }

    pub fn get_name(&self) -> Rc<str> {
        Rc::clone(&self.name)
    }

    pub fn get_args(&self) -> &Vec<FunctionArgument> {
        &self.args
    }

    pub fn get_return_type(&self) -> &ValueType {
        &self.return_type
    }

    pub fn get_body(&self) -> &ScopeStmt<'ast> {
        &self.body
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
}

impl<'ast> Dissasemble for FunctionStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = format!("fn {}(", self.name);
        for i in 0..self.args.len() {
            string_builder += &self.args[i].name;
            string_builder += " ";
            string_builder += &self.args[i].value_type.dissasemble();
            if i != 0 {
                string_builder += ", ";
            }
        }
        string_builder += format!(") {}", if self.return_type != ValueType::Void {
            self.return_type.dissasemble() + " "
        } else {
            "".to_string()
        }).as_str();

        string_builder += self.body.dissasemble().as_str();

        string_builder
    }
}

impl<'ast> StmtTrait for FunctionStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn validate_stmt(&mut self, _: &SymbolTableRef, error_handler: &mut ErrorHandler) {
        let symbol_table_ref = &self.body.get_symbol_table_ref();

        for arg in &self.args {
            symbol_table_ref
                .get_mut()
                .insert_var(
                    Rc::clone(&arg.name),
                    arg.value_type.clone(),
                    arg.is_mutable,
                    arg.metadata
                );
        }

        self.body.validate_stmt(symbol_table_ref, error_handler);
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }
}
