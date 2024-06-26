use std::{ cell::RefCell, fmt::Debug, rc::Rc };

use ahash::AHashMap;

use crate::compiler::{
    error_handler::{ CompileError, ReportedError },
    ir::ast::{
        expr::{ Expr, ExprTrait },
        stmt::{ FunctionArgument, FunctionStmt, VarAssignStmt, VarDefStmt },
    },
    parser::token::TokenMetadata,
    print_todo,
    Dissasemble,
};

use super::value::ValueType;

#[derive(Debug)]
pub enum SymbolState {
    Unchanged,
    MaybeChanged,
}

#[derive(Debug, Clone)]
pub struct SymbolVariable {
    value_type: ValueType,
    is_mutable: bool,
    metadata: TokenMetadata,
}

impl SymbolVariable {
    pub fn new(value_type: ValueType, is_mutable: bool, metadata: TokenMetadata) -> Self {
        Self { value_type, is_mutable, metadata }
    }

    pub fn get_value_type(&self) -> &ValueType {
        &self.value_type
    }
    pub fn get_is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
}

#[derive(Debug)]
pub struct SymbolFunction {
    return_type: ValueType,
    args: Vec<FunctionArgument>,
    metadata: TokenMetadata,
}

impl SymbolFunction {
    pub fn new(
        return_type: ValueType,
        args: Vec<FunctionArgument>,
        metadata: TokenMetadata
    ) -> Self {
        Self { return_type, args, metadata }
    }

    pub fn get_return_type(&self) -> &ValueType {
        &self.return_type
    }

    pub fn get_args(&self) -> &Vec<FunctionArgument> {
        &self.args
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata
    }
}

#[derive(Debug)]
pub enum Symbol {
    Variable(SymbolVariable),
    Function(SymbolFunction),
}

impl Symbol {
    pub fn new_variable(value_type: ValueType, is_mutable: bool, metadata: TokenMetadata) -> Self {
        Self::Variable(SymbolVariable::new(value_type, is_mutable, metadata))
    }

    pub fn new_function(
        return_type: ValueType,
        args: Vec<FunctionArgument>,
        metadata: TokenMetadata
    ) -> Self {
        Self::Function(SymbolFunction::new(return_type, args, metadata))
    }

    pub fn try_value_type_as_var(&self) -> Result<ValueType, String> {
        match self {
            Self::Variable(symbol_var) => Ok(symbol_var.get_value_type().clone()),
            Self::Function(symbol_fn) => {
                Err(
                    format!(
                        "Expected {} arguments but got 0. Use parentheses to call the function: '(_)'",
                        symbol_fn.get_args().len()
                    )
                )
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct SSAKey {
    ident: Rc<str>,
    subscript: usize,
}

impl SSAKey {
    pub fn new(ident: Rc<str>, subscript: usize) -> Self {
        Self { ident, subscript }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }

    pub fn get_subscript(&self) -> usize {
        self.subscript
    }
}

pub trait SymbolTableActions {
    fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol>;

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String>;

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String>;

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)>;

    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAKey;
}

pub trait SymbolTableAlloc {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef;
}

#[derive(Debug)]
struct Symbols {
    symbols: AHashMap<SSAKey, (Symbol, SymbolState)>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            symbols: AHashMap::new(),
        }
    }

    fn count_ident_occurences(&self, ident: &Rc<str>) -> usize {
        self.symbols
            .iter()
            .filter(|&(key, _)| key.get_ident() == *ident)
            .count()
    }
}

impl SymbolTableActions for Symbols {
    fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol> {
        match self.symbols.get(ssa_key) {
            Some((symbol, _)) => Some(symbol),
            None => None,
        }
    }

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String> {
        match self.lookup(ident) {
            Some((_, symbol)) => {
                match symbol {
                    Symbol::Function(symbol_fn) => Ok(symbol_fn),
                    Symbol::Variable(_) =>
                        Err(
                            format!("Undefined function: '{}'. A variable with a similar name exists.", ident)
                        ),
                }
            }
            None => { Err(format!("Undefined function: '{}'", ident)) }
        }
    }

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String> {
        match self.lookup(ident) {
            Some((_, symbol)) => {
                match symbol {
                    Symbol::Variable(symbol_var) => Ok(symbol_var),
                    Symbol::Function(_) =>
                        Err(
                            format!("Undefined variable: '{}'. A function with a similar name exists.", ident)
                        ),
                }
            }
            None => { Err(format!("Undefined variable: '{}'", ident)) }
        }
    }

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.0.ident == *ident)
            .max_by_key(|(key, _)| key.subscript)
            .and_then(|(ssa_key, (symbol, _))| Some((ssa_key, symbol)))
    }

    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAKey {
        let ssa_subscript = self.count_ident_occurences(&ident) + 1;
        let ssa_key = SSAKey::new(ident, ssa_subscript);
        self.symbols.insert(ssa_key.clone(), (symbol, SymbolState::Unchanged));
        ssa_key
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolTableRef {
    raw_ptr: *mut LocalSymbolTable,
}

impl SymbolTableRef {
    pub(in super::symbol_table) fn new(raw_ptr: *mut LocalSymbolTable) -> Self {
        Self { raw_ptr }
    }

    pub fn get(&self) -> &LocalSymbolTable {
        unsafe { &*self.raw_ptr }
    }

    pub fn get_mut(&self) -> &mut LocalSymbolTable {
        unsafe { &mut *self.raw_ptr }
    }
}

impl SymbolTableAlloc for SymbolTableRef {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef {
        unsafe { (*self.raw_ptr).alloc_symbol_table(return_type) }
    }
}

#[derive(Debug)]
pub struct LocalSymbolTable {
    symbols: Symbols,
    parent: Option<*mut Self>,
    global_symbol_table: *mut GlobalSymbolTable,
    fn_return_type: Option<ValueType>,
}

impl LocalSymbolTable {
    pub fn new(
        parent: Option<*mut Self>,
        global_symbol_table: *mut GlobalSymbolTable,
        fn_return_type: Option<ValueType>
    ) -> Self {
        Self {
            symbols: Symbols::new(),
            parent,
            global_symbol_table,
            fn_return_type,
        }
    }

    pub fn get_is_in_fn(&self) -> bool {
        self.fn_return_type.is_some()
    }

    pub fn get_fn_return_type(&self) -> Option<&ValueType> {
        self.fn_return_type.as_ref()
    }

    pub fn insert_var(
        &mut self,
        name: Rc<str>,
        value_type: ValueType,
        is_mutable: bool,
        metadata: TokenMetadata
    ) {
        let symbol = Symbol::new_variable(value_type, is_mutable, metadata);

        self.symbols.insert(name, symbol);
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

        self.symbols.insert(fn_stmt.get_name(), symbol);

        Ok(())
    }

    pub fn assing_var(&mut self, var_assign_stmt: &mut VarAssignStmt) -> Result<(), CompileError> {
        let symbol_table_ref = SymbolTableRef::new(self as *mut LocalSymbolTable);

        let value_type = var_assign_stmt.get_mut_value_expr().type_check(&symbol_table_ref)?;

        let ident_expr = match (unsafe { &*var_assign_stmt.get_target_expr().get_expr() }) {
            Expr::IdentifierExpr(ident_expr) => ident_expr,
            _ => panic!("Only assignment to lexemes is currently supported"),
        };
        let ident_expr_lexeme = ident_expr.get_lexeme();

        let symbol_var = self
            .lookup_as_var(&ident_expr_lexeme)
            .or_else(|msg|
                Err(CompileError::new(ReportedError::new(msg, ident_expr.collect_metadata())))
            )?;

        match symbol_var.get_is_mutable() {
            true => {
                if symbol_var.get_value_type().is(&value_type) {
                    self.symbols.insert(ident_expr_lexeme, Symbol::Variable(symbol_var.clone()));
                } else {
                    return Err(
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
                    );
                }
            }
            false => {
                return Err(
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
                );
            }
        }

        Ok(())
    }

    pub fn declare_var(&mut self, var_def_stmt: &mut VarDefStmt) -> Result<(), CompileError> {
        let symbol_table_ref = SymbolTableRef::new(self as *mut LocalSymbolTable);

        let value_type = var_def_stmt.get_resolved_value_type(&symbol_table_ref)?;

        let symbol = Symbol::new_variable(
            value_type,
            var_def_stmt.get_is_mutable(),
            var_def_stmt.get_metadata()
        );

        self.symbols.insert(var_def_stmt.get_name(), symbol);

        Ok(())
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
            SymbolTableRef::new(allocated_table)
        }
    }
}

impl SymbolTableActions for LocalSymbolTable {
    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAKey {
        self.symbols.insert(ident, symbol)
    }

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)> {
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

    fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol> {
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

#[derive(Debug)]
pub struct GlobalSymbolTable {
    allocated_symbol_tables: Vec<LocalSymbolTable>,
    symbols: Symbols,
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self {
            allocated_symbol_tables: Vec::new(),
            symbols: Symbols::new(),
        }
    }

    fn alloc_empty(
        &mut self,
        parent: Option<*mut LocalSymbolTable>,
        return_type: Option<ValueType>
    ) -> *mut LocalSymbolTable {
        let global_ptr = self as *mut GlobalSymbolTable;
        self.allocated_symbol_tables.push(LocalSymbolTable::new(parent, global_ptr, return_type));
        let idx = self.allocated_symbol_tables.len() - 1;
        self.allocated_symbol_tables.get_mut(idx).unwrap() as *mut LocalSymbolTable
    }
}

impl SymbolTableAlloc for GlobalSymbolTable {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef {
        let allocated_symbol_table = self.alloc_empty(None, return_type);
        SymbolTableRef::new(allocated_symbol_table)
    }
}

impl SymbolTableActions for GlobalSymbolTable {
    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAKey {
        self.symbols.insert(ident, symbol)
    }

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)> {
        self.symbols.lookup(ident)
    }

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String> {
        self.symbols.lookup_as_fn(ident)
    }

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String> {
        self.symbols.lookup_as_var(ident)
    }

    fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol> {
        self.symbols.lookup_with_key(ssa_key)
    }
}

pub struct Compiler {
    symbol_table: GlobalSymbolTable,
    // register_allocator: RegisterAllocator
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            symbol_table: GlobalSymbolTable::new(),
        }
    }

    pub fn get_mut_symbol_table(&mut self) -> &mut GlobalSymbolTable {
        &mut self.symbol_table
    }
}
