use std::rc::Rc;

use ahash::AHashMap;
use symbols::Symbols;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    error_handler::{ CompileError, ReportedError },
    ir::ast::{ stmt::VarAssignStmt, AST_DISSASEMBLE_INDENTATION },
    parser::Lexeme,
    traits::ExprTrait,
    Dissasemble,
};

use super::{
    NativeSymbolFn,
    NativeSymbolVar,
    NativeSymbolsHandler,
    SymbolFn,
    SymbolTable,
    SymbolVar,
    UserNativeSymbolFn,
    UserSymbolFn,
    UserSymbolVar,
};

pub mod symbol_table;
pub mod symbols;
pub mod util_structs;

#[derive(PartialEq)]
enum SymbolType {
    Fn,
    Var,
}
impl SymbolType {
    pub fn is_fn(&self) -> bool {
        match self {
            Self::Fn => true,
            _ => false,
        }
    }

    pub fn is_var(&self) -> bool {
        match self {
            Self::Var => true,
            _ => false,
        }
    }

    pub fn is(&self, other: &Self) -> bool {
        self == other
    }
}

struct SSADeclaration {
    pub ssa_ident: SSAIdent,
    pub symbol_type: SymbolType,
    pub scoped_table_id: usize,
}

pub struct ProgramSymbolTablePhase1 {
    symbol_tables: Vec<SymbolTable>,
    ident_occurences: AHashMap<Rc<str>, usize>,
    ssa_declarations: Vec<SSADeclaration>,
    native_symbols_handler: NativeSymbolsHandler,
    current_symbol_table_id: usize,
}

impl Dissasemble for ProgramSymbolTablePhase1 {
    fn dissasemble(&self) -> String {
        self.dissasemble_table(0, 0)
    }
}
// Helper dissasemble methods
impl ProgramSymbolTablePhase1 {
    fn dissasemble_table(&self, table_id: usize, scope_depth: usize) -> String {
        let table = self.get_table(table_id);
        let child_table_ids = self.get_child_table_ids(table_id);

        let mut string_builder = format!(
            "{}[{}]: {}\n",
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            table_id,
            table.get_symbols().dissasemble()
        );
        for child_table_id in child_table_ids.iter() {
            string_builder += self.dissasemble_table(*child_table_id, scope_depth + 1).as_str();
            string_builder += "\n";
        }

        string_builder
    }

    fn get_child_table_ids(&self, table_id: usize) -> Vec<usize> {
        self.symbol_tables
            .iter()
            .enumerate()
            .filter_map(|(i, symbol_table)| {
                if let Some(parent_symbol_table_id) = symbol_table.get_parent_symbol_table_id() {
                    if parent_symbol_table_id == table_id {
                        return Some(i);
                    }
                }
                None
            })
            .collect::<Vec<_>>()
    }
}

impl ProgramSymbolTablePhase1 {
    pub fn new() -> Self {
        Self {
            symbol_tables: vec![],
            ident_occurences: AHashMap::new(),
            ssa_declarations: Vec::new(),
            native_symbols_handler: NativeSymbolsHandler,
            current_symbol_table_id: 0,
        }
    }

    pub fn expect_main_function(&mut self) -> Result<(), CompileError> {
        self.get_mut_table(0).get_mut_symbols().lookup_main()
    }

    pub(super) fn get_table(&self, table_id: usize) -> &SymbolTable {
        self.symbol_tables.get(table_id).expect("Expected symbol table")
    }

    pub(super) fn get_mut_table(&mut self, table_id: usize) -> &mut SymbolTable {
        self.symbol_tables.get_mut(table_id).expect("Expected symbol table")
    }

    pub fn set_current_symbol_table_id(&mut self, new_symbol_table_id: usize) {
        self.current_symbol_table_id = new_symbol_table_id;
    }

    pub fn get_current_symbol_table_id(&self) -> usize {
        self.current_symbol_table_id
    }
    pub fn get_next_symbol_table_id(&self) -> usize {
        self.symbol_tables.len()
    }

    pub fn get_fn_ret_type(&self) -> Option<&ValueType> {
        self.get_table(self.current_symbol_table_id).get_ret_type()
    }

    fn get_prev_symbol_var_for_assignment(
        &self,
        var_assign_stmt: &VarAssignStmt,
        ssa_ident: &SSAIdent
    ) -> Result<&UserSymbolVar, CompileError> {
        let prev_symbol_var = self
            .get_table(self.current_symbol_table_id)
            .lookup_var_by_name(&ssa_ident.get_ident(), self);

        let prev_symbol_var = match prev_symbol_var {
            Ok(symbol_var) => {
                match symbol_var {
                    SymbolVar::UserSymbolVar(user_symbol_var) => user_symbol_var,
                    SymbolVar::NativeSymbolVar(_) => {
                        return Err(
                            CompileError::new(
                                ReportedError::new(
                                    format!(
                                        "Cannot assign a value to global variable: '{}'",
                                        ssa_ident.get_ident()
                                    ),
                                    var_assign_stmt.get_target_expr().collect_metadata()
                                )
                            )
                        );
                    }
                }
            }
            Err(msg) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            msg,
                            var_assign_stmt.get_target_expr().collect_metadata()
                        )
                    )
                );
            }
        };

        Ok(prev_symbol_var)
    }

    pub fn assign_var(
        &mut self,
        var_assign_stmt: &mut VarAssignStmt
    ) -> Result<ValueType, CompileError> {
        let (result_value, ssa_ident) = var_assign_stmt.get_mut_target_expr().type_check(self)?;
        let ssa_ident = ssa_ident.expect("Expected it to be there (or else invalid assignment)");

        var_assign_stmt.set_ssa_ident(ssa_ident.clone());

        let prev_symbol_var = self
            .get_prev_symbol_var_for_assignment(var_assign_stmt, &ssa_ident)?
            .clone();

        self.insert_var(
            ssa_ident.clone(),
            UserSymbolVar::new(
                prev_symbol_var.get_value_type().clone(),
                var_assign_stmt.get_target_expr().collect_metadata().into(),
                prev_symbol_var.get_mut_keyword_metadata()
            )
        );

        let (value_type, _) = var_assign_stmt.get_mut_value_expr().type_check(self)?;

        if prev_symbol_var.get_is_mutable() {
            if result_value.is_assignable_to(&value_type) {
                Ok(value_type)
            } else {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Variable '{}' is of type '{}' but it is assigned to type '{}'",
                                ssa_ident.get_ident(),
                                prev_symbol_var.get_value_type().dissasemble(),
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
        } else {
            Err(
                CompileError::new_multiple(
                    vec![
                        ReportedError::new(
                            format!(
                                "Cannot assign a value to immutable variable '{}'",
                                ssa_ident.get_ident()
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
                        ),
                        ReportedError::new(
                            format!("Consider changing this to `mut {}`", ssa_ident.get_ident()),
                            prev_symbol_var.get_ident_metadata().into()
                        )
                    ]
                )
            )
        }
    }

    pub fn insert_var(&mut self, ssa_ident: SSAIdent, symbol_var: UserSymbolVar) {
        self.get_mut_table(self.current_symbol_table_id).insert_var(ssa_ident, symbol_var);
    }

    pub fn insert_fn(
        &mut self,
        ssa_ident: SSAIdent,
        symbol_fn: UserSymbolFn
    ) -> Result<(), CompileError> {
        self.get_mut_table(self.current_symbol_table_id).insert_fn(ssa_ident, symbol_fn)
    }

    pub fn lookup_var<'a>(&'a self, ssa_ident: &'a SSAIdent) -> Result<SymbolVar<'a>, String> {
        let table = self.get_table(self.current_symbol_table_id);
        table.lookup_var(ssa_ident, self)
    }

    pub fn lookup_var_ident_by_name<'a>(
        &'a self,
        ssa_ident: &Rc<str>
    ) -> Result<&SSAIdent, String> {
        let table = self.get_table(self.current_symbol_table_id);
        table.lookup_var_ident_by_name(ssa_ident, self)
    }

    pub fn lookup_fn<'a>(&'a self, ssa_ident: &'a SSAIdent) -> Result<SymbolFn<'a>, String> {
        let table = self.get_table(self.current_symbol_table_id);
        table.lookup_fn(ssa_ident, self)
    }

    pub(super) fn lookup_global_var(&self, ident: &Rc<str>) -> Result<NativeSymbolVar, String> {
        self.native_symbols_handler.lookup_var(ident)
    }

    pub(super) fn lookup_global_fn(&self, ident: &Rc<str>) -> Result<UserNativeSymbolFn, String> {
        self.native_symbols_handler.lookup_fn(ident)
    }

    fn search_ident(&self, lexeme: &Lexeme, searching_symbol_type: SymbolType) -> SSAIdent {
        let mut all_accesable_scope_ids = vec![];

        let mut curr_scope_id = Some(self.current_symbol_table_id);
        while let Some(scope_id) = curr_scope_id {
            all_accesable_scope_ids.push(scope_id);
            curr_scope_id = self.get_table(scope_id).get_parent_symbol_table_id();
        }

        for ssa_declaration in self.ssa_declarations.iter().rev() {
            let ssa_ident = &ssa_declaration.ssa_ident;
            let symbol_type = &ssa_declaration.symbol_type;
            if
                symbol_type.is(&searching_symbol_type) &&
                ssa_declaration.ssa_ident.get_ident() == *lexeme.borrow_ident() &&
                all_accesable_scope_ids.contains(&ssa_declaration.scoped_table_id)
            {
                return ssa_ident.clone();
            }
        }

        SSAIdent::new(lexeme.get_ident(), 1)
    }

    pub fn search_fn_ssa_ident(&self, lexeme: &Lexeme) -> SSAIdent {
        self.search_ident(lexeme, SymbolType::Fn)
    }

    pub fn search_var_ssa_ident(&self, lexeme: &Lexeme) -> SSAIdent {
        self.search_ident(lexeme, SymbolType::Var)
    }

    pub fn declare_fn_ssa_ident(&mut self, lexeme: &Lexeme) -> SSAIdent {
        let new_subscript = self.ident_occurences
            .get(lexeme.borrow_ident())
            .cloned()
            .map(|subscript| subscript + 1)
            .unwrap_or(1);

        self.ident_occurences.insert(lexeme.get_ident(), new_subscript);
        let ssa_ident = SSAIdent::new(lexeme.get_ident(), new_subscript);
        self.ssa_declarations.push(SSADeclaration {
            ssa_ident: ssa_ident.clone(),
            symbol_type: SymbolType::Fn,
            scoped_table_id: self.current_symbol_table_id,
        });
        ssa_ident
    }

    pub fn declare_var_ssa_ident(&mut self, lexeme: &Lexeme) -> SSAIdent {
        let new_subscript = self.ident_occurences
            .get(lexeme.borrow_ident())
            .cloned()
            .map(|subscript| subscript + 1)
            .unwrap_or(1);
        self.ident_occurences.insert(lexeme.get_ident(), new_subscript);
        let ssa_ident = SSAIdent::new(lexeme.get_ident(), new_subscript);
        self.ssa_declarations.push(SSADeclaration {
            ssa_ident: ssa_ident.clone(),
            symbol_type: SymbolType::Var,
            scoped_table_id: self.current_symbol_table_id,
        });
        ssa_ident
    }

    pub fn make_stage_2(self) {}

    pub fn new_symbol_table(
        &mut self,
        parent_symbol_table_id: Option<usize>,
        fn_return_type: Option<ValueType>
    ) -> usize {
        self.symbol_tables.push(SymbolTable::new(parent_symbol_table_id, fn_return_type));
        let new_id = self.symbol_tables.len() - 1;
        self.current_symbol_table_id = new_id;
        new_id
    }
}
