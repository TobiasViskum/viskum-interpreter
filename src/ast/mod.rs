use crate::{
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value_v2::ValueType,
};

use self::{ expr::Expr, stmt::{ FunctionArgument, FunctionStmt, ScopeStmt, Stmt } };

mod generate_cfg;
pub mod expr;
pub mod stmt;
mod type_check;

#[derive(Debug)]
pub struct Ast {
    pub main_scope: ScopeStmt,
    path_to_parent_scope: Vec<*mut ScopeStmt>,
    current_scope_ptr: Option<*mut ScopeStmt>,
}

impl Ast {
    pub fn push_stmt(&mut self, stmt: Stmt) {
        if self.current_scope_ptr.is_none() {
            self.current_scope_ptr = Some(&mut self.main_scope);
        }

        if let Some(scope) = self.current_scope_ptr {
            unsafe {
                let current_scope = &mut *scope;
                current_scope.push_stmt(stmt);
            }
        } else {
            panic!("No scope to push to");
        }
    }

    pub fn start_function(
        &mut self,
        name: String,
        args: Vec<FunctionArgument>,
        return_type: ValueType
    ) {}

    pub fn end_function(&mut self) {}

    pub fn start_scope(&mut self) {
        if self.current_scope_ptr.is_none() {
            self.current_scope_ptr = Some(&mut self.main_scope);
        } else {
            let current_scope = self.current_scope_ptr.unwrap();
            unsafe {
                self.path_to_parent_scope.push(current_scope);
                let new_index = (*current_scope).stmts.len();
                (*current_scope).stmts.push(Stmt::ScopeStmt(ScopeStmt::new()));

                let new_scope = match (*current_scope).stmts[new_index] {
                    Stmt::ScopeStmt(ref mut scope) => scope,
                    _ => panic!("Expected scope stmt: {:?}", (*current_scope).stmts[new_index]),
                };
                self.current_scope_ptr = Some(new_scope);
            }
        }
    }

    pub fn end_scope(&mut self) {
        if self.current_scope_ptr.is_none() {
            println!("No scope to end, so must be main scope")
        } else {
            if let Some(parent_scope) = self.path_to_parent_scope.pop() {
                self.current_scope_ptr = Some(parent_scope);
            } else {
                self.current_scope_ptr = None;
            }
        }
    }

    pub fn new() -> Self {
        let main_scope = ScopeStmt::new();

        Self {
            main_scope,
            current_scope_ptr: None,
            path_to_parent_scope: Vec::new(),
        }
    }
}
