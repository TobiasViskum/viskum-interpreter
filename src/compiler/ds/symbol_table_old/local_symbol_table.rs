use std::rc::Rc;

use crate::compiler::{
    ds::value::ValueType,
    error_handler::{ CompileError, ReportedError },
    ir::ast::stmt::{ FunctionStmt, VarAssignStmt, VarDefStmt },
    parser::token::TokenMetadata,
    print_todo,
    traits::{ ExprTrait, SymbolTableActions, SymbolTableAlloc },
    Dissasemble,
};

use super::{
    helper_structs::{ Symbol, SymbolFunction, SymbolTableRef, SymbolVariable, Symbols },
    GlobalSymbolTable,
    SSAIdent,
};

#[derive(Debug)]
pub struct LocalSymbolTable {
    symbols: Symbols,
    parent: Option<*mut Self>,
    global_symbol_table: *mut GlobalSymbolTable,
    fn_return_type: Option<ValueType>,
    // is_in_loop: bool,
}

impl LocalSymbolTable {
    pub fn new(
        parent: Option<*mut Self>,
        global_symbol_table: *mut GlobalSymbolTable,
        fn_return_type: Option<ValueType>
        // is_in_loop: bool
    ) -> Self {
        Self {
            symbols: Symbols::new(),
            parent,
            global_symbol_table,
            fn_return_type,
            // is_in_loop,
        }
    }

    pub fn get_all_vars(&self) -> Vec<SSAIdent> {
        self.symbols
            .iter()
            .filter(|&(_, (symbol, _))| symbol.is_var())
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub fn get_is_in_fn(&self) -> bool {
        self.fn_return_type.is_some()
    }

    pub fn get_fn_return_type(&self) -> Option<&ValueType> {
        self.fn_return_type.as_ref()
    }

    pub fn insert_var(
        &mut self,
        name: &Rc<str>,
        value_type: ValueType,
        is_mutable: bool,
        metadata: TokenMetadata
    ) {
        let symbol = Symbol::new_variable(value_type, is_mutable, metadata);

        let ssa_subscript = self.get_new_ident_subscript(name);
        self.symbols.insert(SSAIdent::new(Rc::clone(name), ssa_subscript), symbol);
    }

    pub fn get_new_ident_subscript(&mut self, ident: &Rc<str>) -> usize {
        unsafe { (*self.global_symbol_table).get_new_ident_subscript(ident) }
    }

    pub fn declare_fn(&mut self, fn_stmt: &FunctionStmt) -> Result<(), CompileError> {
        let symbol = Symbol::new_function(
            fn_stmt.get_return_type().clone(),
            fn_stmt.get_args().clone(),
            fn_stmt.get_metadata()
        );

        print_todo(
            "Check if the body returns the provided return type. Also check that if the body returns something that the correct return type is provided"
        );

        self.insert(fn_stmt.get_name(), symbol);

        Ok(())
    }

    pub fn assing_var(
        &mut self,
        var_assign_stmt: &mut VarAssignStmt
    ) -> Result<SSAIdent, CompileError> {
        let symbol_table_ref = SymbolTableRef::new(self as *mut LocalSymbolTable);

        let value_type = var_assign_stmt.get_mut_value_expr().type_check(&symbol_table_ref)?;

        var_assign_stmt.get_mut_target_expr().type_check(&symbol_table_ref)?;

        let ident_expr_lexeme = var_assign_stmt.get_target_expr().get_lexeme();

        let symbol_var = self
            .lookup_as_var(&ident_expr_lexeme)
            .or_else(|msg| {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            msg,
                            var_assign_stmt.get_target_expr().collect_metadata()
                        )
                    )
                )
            })?;

        match symbol_var.get_is_mutable() {
            true => {
                if symbol_var.get_value_type().is(&value_type) {
                    let ssa_key = self.insert(
                        ident_expr_lexeme,
                        Symbol::Variable(symbol_var.clone())
                    );
                    Ok(ssa_key)
                } else {
                    Err(
                        CompileError::new(
                            ReportedError::new(
                                format!(
                                    "Variable '{}' is of type '{}' but it is assigned to type '{}'",
                                    ident_expr_lexeme,
                                    symbol_var.get_value_type().dissasemble(),
                                    value_type.dissasemble()
                                ),
                                {
                                    let mut src_chars_range = var_assign_stmt
                                        .get_target_expr()
                                        .collect_metadata();
                                    src_chars_range.merge(
                                        &var_assign_stmt.get_value_expr().collect_metadata()
                                    );
                                    src_chars_range
                                }
                            )
                        )
                    )
                }
            }
            false =>
                Err(
                    CompileError::new_multiple(
                        vec![
                            ReportedError::new(
                                format!("Cannot assign a value to immutable variable '{}'", ident_expr_lexeme),
                                {
                                    let mut src_chars_range = var_assign_stmt
                                        .get_target_expr()
                                        .collect_metadata();
                                    src_chars_range.merge(
                                        &var_assign_stmt.get_value_expr().collect_metadata()
                                    );
                                    src_chars_range
                                }
                            ),
                            ReportedError::new(
                                format!("Consider changing this to `mut {}`", ident_expr_lexeme),
                                symbol_var.get_metadata().into()
                            )
                        ]
                    )
                ),
        }
    }

    #[must_use]
    pub fn declare_var(&mut self, var_def_stmt: &mut VarDefStmt) -> Result<SSAIdent, CompileError> {
        let mut symbol_table_ref = SymbolTableRef::new(self as *mut LocalSymbolTable);

        let value_type = var_def_stmt.get_resolved_value_type(&symbol_table_ref)?;

        let symbol = Symbol::new_variable(
            value_type,
            var_def_stmt.get_is_mutable(),
            var_def_stmt.get_metadata()
        );

        let ssa_key = symbol_table_ref.get_mut().insert(var_def_stmt.get_name(), symbol);

        Ok(ssa_key)
    }
}

impl SymbolTableAlloc for LocalSymbolTable {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef {
        unsafe {
            let self_ptr = self as *mut LocalSymbolTable;
            let allocated_table = (*self.global_symbol_table).alloc_empty(
                Some(self_ptr),
                return_type
            );
            // Here I've created the SymbolTableRef
            SymbolTableRef::new(allocated_table)
        }
    }
}

impl SymbolTableActions for LocalSymbolTable {
    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAIdent {
        let ssa_subscript = self.get_new_ident_subscript(&ident);
        self.symbols.insert(SSAIdent::new(ident, ssa_subscript), symbol)
    }

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAIdent, &Symbol)> {
        match self.symbols.lookup(ident) {
            Some(v) => Some(v),
            None =>
                match self.parent {
                    Some(table) => unsafe { (*table).lookup(ident) }
                    None => unsafe { (*self.global_symbol_table).lookup(ident) }
                }
        }
    }

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String> {
        match self.symbols.lookup_as_fn(ident) {
            Ok(v) => Ok(v),
            Err(_) =>
                match self.parent {
                    Some(table) => unsafe { (*table).lookup_as_fn(ident) }
                    None => unsafe { (*self.global_symbol_table).lookup_as_fn(ident) }
                }
        }
    }

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String> {
        match self.symbols.lookup_as_var(ident) {
            Ok(v) => Ok(v),
            Err(_) =>
                match self.parent {
                    Some(table) => unsafe { (*table).lookup_as_var(ident) }
                    None => unsafe { (*self.global_symbol_table).lookup_as_var(ident) }
                }
        }
    }

    fn lookup_with_key(&self, ssa_key: &SSAIdent) -> Option<&Symbol> {
        match self.symbols.lookup_with_key(ssa_key) {
            Some(v) => Some(v),
            None =>
                match self.parent {
                    Some(table) => unsafe { (*table).lookup_with_key(ssa_key) }
                    None => unsafe { (*self.global_symbol_table).lookup_with_key(ssa_key) }
                }
        }
    }
}
