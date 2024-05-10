use std::collections::HashMap;

use crate::{
    ast::{
        expr::{ AstIdentifier, AstValue, BinaryExpr, Expr, UnaryExpr },
        stmt::{
            FunctionArgument,
            ScopeStmt,
            Stmt,
            TypeDefStmt,
            VarAssignStmt,
            VarDefStmt,
        },
        Ast,
    },
    operations::{ BinaryOp, UnaryOp },
    value::ValueType,
};
use super::token::{ token_type::TokenType, Token, TokenMetadata };

#[derive(Debug)]
struct AstVariableValue {
    value_type: ValueType,
    is_mutable: bool,
    is_initialized: bool,
}

impl AstVariableValue {
    pub fn to_tuple(&self) -> (ValueType, bool, bool) {
        (self.value_type.clone(), self.is_mutable, self.is_initialized)
    }
}

#[derive(Debug)]
pub struct AstScope {
    definitions: HashMap<String, AstVariableValue>,
}

impl AstScope {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        lexeme: String,
        value_type: ValueType,
        is_mutable: bool,
        is_initialized: bool
    ) {
        self.definitions.insert(lexeme, AstVariableValue {
            value_type,
            is_mutable,
            is_initialized,
        });
    }

    pub fn get(&self, lexeme: &String) -> Option<(ValueType, bool, bool)> {
        match self.definitions.get(lexeme) {
            Some(v) => Some(v.to_tuple()),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct AstEnvironment {
    scopes: Vec<AstScope>,
    scope_depth: usize,
}

impl AstEnvironment {
    pub fn new() -> Self {
        Self {
            scopes: vec![AstScope::new()],
            scope_depth: 0,
        }
    }

    pub fn get_scope_depth(&self) -> usize {
        self.scope_depth
    }

    pub fn start_scope(&mut self) {
        self.scope_depth += 1;
        self.scopes.push(AstScope::new());
    }

    pub fn end_scope(&mut self) {
        if self.scope_depth > 0 {
            self.scope_depth -= 1;
        }
        self.scopes.pop();
    }

    pub fn insert(
        &mut self,
        lexeme: String,
        value_type: ValueType,
        is_mutable: bool,
        is_initialized: bool
    ) {
        self.scopes[self.scope_depth].insert(lexeme, value_type, is_mutable, is_initialized);
    }

    pub fn get(&self, lexeme: &String) -> Option<(ValueType, bool, bool)> {
        for i in (0..self.scope_depth + 1).rev() {
            match self.scopes[i].get(lexeme) {
                Some(v) => {
                    return Some(v);
                }
                None => {
                    continue;
                }
            }
        }

        None
    }
}