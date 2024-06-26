use crate::compiler::{ ds::symbol_table::SymbolTableRef, error_handler::ErrorHandler, Dissasemble };

use super::{ LinearControlFlow, Stmt, StmtTrait, Stmts };

#[derive(Debug)]
pub struct ScopeStmt<'ast> {
    stmts: Stmts<'ast>,
    symbol_table_ref: SymbolTableRef,
    // forwards_declarations: Stmts, // TypeDefStmt, FnStmt, (ClassStmt)
}

impl<'ast> ScopeStmt<'ast> {
    pub fn new(symbol_table_ref: SymbolTableRef) -> Self {
        Self {
            stmts: Stmts::new(),
            symbol_table_ref,
            // symbol_table_ref,
        }
    }

    pub fn get_symbol_table_ref(&self) -> SymbolTableRef {
        self.symbol_table_ref
    }

    pub fn push_stmt(&mut self, stmt: Stmt<'ast>) {
        self.stmts.push(stmt, &self.symbol_table_ref);
    }

    pub fn get_stmts(&self) -> &Stmts<'ast> {
        &self.stmts
    }
}

impl<'ast> Dissasemble for ScopeStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("{\n");
        string_builder += self.stmts.dissasemble().as_str();
        string_builder += "}\n";
        string_builder
    }
}

impl<'ast> StmtTrait for ScopeStmt<'ast> {
    fn is_linear_control_flow(&self) -> bool {
        self.stmts.is_linear_control_flow()
    }

    fn validate_stmt(&mut self, _: &SymbolTableRef, error_handler: &mut ErrorHandler) {
        let symbol_table_ref = &self.get_symbol_table_ref();

        self.stmts.validate_stmt(symbol_table_ref, error_handler);
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        todo!()
    }
}
