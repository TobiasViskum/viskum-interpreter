use crate::{
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value::{ Value, ValueType },
};

use self::{ expr::Expr, stmt::{ ScopeStmt, Stmt } };

pub mod expr;
pub mod stmt;

#[derive(Debug, Clone)]
pub struct AstValue {
    value: Value,
    token_metadata: TokenMetadata,
}

impl AstValue {
    pub fn new(value: Value, token_metadata: TokenMetadata) -> Self {
        Self {
            value,
            token_metadata,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value
    }

    pub fn get_token_metadata(&self) -> TokenMetadata {
        self.token_metadata
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata);
    }
}

#[derive(Debug, Clone)]
pub struct AstIdentifier {
    lexeme: String,
    token_metadata: TokenMetadata,
}

impl AstIdentifier {
    pub fn new(lexeme: String, token_metadata: TokenMetadata) -> Self {
        Self {
            lexeme,
            token_metadata,
        }
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn get_token_metadata(&self) -> TokenMetadata {
        self.token_metadata
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata);
    }
}

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
