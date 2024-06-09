use ahash::AHashMap;

use crate::value::ValueType;

use super::{ stmt::{ Stmt, Typing, TypingValue }, Ast };

#[derive(Debug)]
pub struct SymbolTypeDef {
    pub value_type: ValueType,
}
#[derive(Debug)]
pub struct SymbolVariable {
    pub value_type: ValueType,
    pub is_mutable: bool,
    pub is_initialized: bool,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub definitions: AHashMap<String, SymbolVariable>,
    pub type_definitions: AHashMap<String, SymbolTypeDef>,
    pub forward_declarations: Vec<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            definitions: AHashMap::new(),
            type_definitions: AHashMap::new(),
            forward_declarations: vec![],
        }
    }
}
#[derive(Debug)]
pub struct AstEnvironment {
    pub scopes: Vec<SymbolTable>,
    pub scope_depth: usize,
}

impl AstEnvironment {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            scope_depth: 0,
        }
    }

    pub fn start_scope(&mut self) {
        self.scopes.push(SymbolTable::new());
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
        self.scope_depth -= 1;
    }

    pub fn start_main_scope(&mut self) {
        self.scopes.push(SymbolTable::new())
    }

    pub fn end_main_scope(&mut self) {
        self.scopes.pop();
    }
}

impl Ast {
    pub fn type_check(&self) {
        let mut ast_environment = AstEnvironment::new();

        ast_environment.start_main_scope();

        self.main_scope.forward_declare(&mut ast_environment);

        for stmt in &self.main_scope.stmts {
            match stmt {
                Stmt::TypeDefStmt(type_def_stmt) => {
                    let type_name = type_def_stmt.type_name.clone();
                    let typing_value = match &type_def_stmt.typing.typing_value {
                        TypingValue::Custom(_) => { panic!("Custom types not supported yet") }
                        TypingValue::ValueType(value_type) => { value_type.clone() }
                    };

                    ast_environment.scopes[ast_environment.scope_depth].type_definitions.insert(
                        type_name,
                        SymbolTypeDef {
                            value_type: typing_value,
                        }
                    );
                }
                Stmt::ScopeStmt(scope_stmt) => {
                    scope_stmt.forward_declare(&mut ast_environment);
                }
                _ => {}
            }
        }

        ast_environment.end_main_scope();
    }
}
