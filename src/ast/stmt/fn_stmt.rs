use std::rc::Rc;

use crate::{
    ast::ast_symbol_table::AstSymbolTable,
    error_handler::ErrorHandler,
    parser::token::TokenMetadata,
    print_todo,
    value::ValueType,
    Dissasemble,
};

use super::{ FunctionArgument, ScopeStmt, StmtTrait };

#[derive(Debug)]
pub struct FunctionStmt {
    name: Rc<str>,
    args: Vec<FunctionArgument>,
    return_type: ValueType,
    body: ScopeStmt,
    metadata: TokenMetadata,
}

impl FunctionStmt {
    pub fn new(
        name: Rc<str>,
        args: Vec<FunctionArgument>,
        return_type: ValueType,
        body: ScopeStmt,
        metadata: TokenMetadata
    ) -> Self {
        Self {
            name,
            args,
            return_type,
            body,
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

    pub fn get_body(&self) -> &ScopeStmt {
        &self.body
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
}

impl Dissasemble for FunctionStmt {
    fn dissasemble(&self) -> String {
        let mut string_builder = format!("fn {}(", self.name);
        for i in 0..self.args.len() {
            string_builder += &self.args[i].name;
            string_builder += &self.args[i].value_type.dissasemble();
            if i != 0 {
                string_builder += ", ";
            }
        }
        string_builder += ") {\n";

        string_builder += self.body.dissasemble().as_str();

        string_builder += "}\n";

        string_builder
    }
}

impl StmtTrait for FunctionStmt {
    fn validate_stmt(
        &mut self,
        ast_symbol_table: &mut AstSymbolTable,
        error_handler: &mut ErrorHandler
    ) {
        ast_symbol_table.declare_function(self, error_handler);

        print_todo(
            "Dont create a new ast symbol table for each function. Because then errors cant be reported if user thinks variables are captured"
        );

        let mut ast_symbol_table = AstSymbolTable::new();
        ast_symbol_table.push_function_return_type(self.return_type.clone());
        ast_symbol_table.declare_function(self, error_handler);

        for arg in &self.args {
            ast_symbol_table.insert_variable(
                Rc::clone(&arg.name),
                arg.value_type.clone(),
                arg.is_mutable,
                arg.metadata
            );
        }

        self.body.validate_stmt(&mut ast_symbol_table, error_handler);
        ast_symbol_table.pop_function_return_type();
    }
}
