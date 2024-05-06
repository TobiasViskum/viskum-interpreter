use std::collections::HashMap;

use crate::{
    ast::{
        expr::{ AstIdentifier, AstValue, BinaryExpr, Expr, UnaryExpr },
        stmt::{
            FunctionArgument,
            ScopeStmt,
            Stmt,
            TypeDefStmt,
            VariableAssignmentStmt,
            VariableDefinitionStmt,
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

#[derive(Debug)]
pub struct AstGenerator {
    ast: Option<Ast>,
    stmts: Vec<Stmt>,
    exprs: Vec<Expr>,
    panic_mode: bool,
    ast_environment: AstEnvironment,
}

impl AstGenerator {
    pub fn new() -> Self {
        Self {
            ast: Some(Ast::new()),
            stmts: Vec::new(),
            exprs: Vec::new(),
            panic_mode: false,
            ast_environment: AstEnvironment::new(),
        }
    }

    fn get_scope_depth(&self) -> usize {
        self.ast_environment.get_scope_depth()
    }

    pub fn start_function(
        &mut self,
        name: String,
        args: Vec<FunctionArgument>,
        return_type: Option<ValueType>
    ) {
        self.ast.as_mut().unwrap().start_function(name, args, return_type);
    }

    pub fn end_function(&mut self) {
        self.ast.as_mut().unwrap().end_function();
    }

    pub fn start_scope(&mut self) {
        self.ast_environment.start_scope();
        self.ast.as_mut().unwrap().start_scope();
    }

    pub fn end_scope(&mut self) {
        self.ast_environment.end_scope();
        self.ast.as_mut().unwrap().end_scope()
    }

    pub fn free(&mut self) {
        self.ast = None;
    }

    pub fn enter_panic_mode(&mut self) {
        self.panic_mode = true;
    }

    pub fn exit_panic_mode(&mut self) {
        self.panic_mode = false;
    }

    pub fn emit_constant_literal(&mut self, value: AstValue) {
        self.exprs.push(Expr::Literal(value));
    }

    pub fn emit_identifier_lookup(&mut self, variable: AstIdentifier) {
        self.exprs.push(Expr::IdentifierLookup(variable));
    }

    pub fn emit_type_definition(
        &mut self,
        type_def: TypeDefStmt
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        self.push_stmt(Stmt::TypeDefStmt(type_def))
    }

    pub fn emit_variable_assignment(
        &mut self,
        token_metadata: TokenMetadata
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        if self.panic_mode {
            return Ok(());
        }
        let value = self.exprs.pop().unwrap();
        let identifier = match self.exprs.pop().unwrap() {
            Expr::IdentifierLookup(v) => v,
            _ => {
                return Err(("Expected identifier in assignment".to_string(), vec![token_metadata]));
            }
        };

        let variable_assignment = Stmt::VariableAssignment(
            VariableAssignmentStmt::new(None, identifier, value)
        );

        self.push_stmt(variable_assignment)
    }

    pub fn emit_variable_definition(
        &mut self,
        lexeme: String,
        identifier_metadata: TokenMetadata,
        value_type: Option<ValueType>,
        is_mutable: bool,
        last_token_in_definition: TokenMetadata
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        let value = self.exprs.pop();

        if
            !matches!(
                identifier_metadata.get_ttype(),
                TokenType::TokenIdentifier | TokenType::TokenInt32 | TokenType::TokenBool
            )
        {
            return Err((
                format!("Invalid variable definition target: '{}'", lexeme),
                vec![identifier_metadata],
            ));
        }

        let variable_definition = Stmt::VariableDefinition(
            VariableDefinitionStmt::new(
                lexeme,
                value_type,
                is_mutable,
                value,
                last_token_in_definition
            )
        );

        self.push_stmt(variable_definition)
    }

    pub fn emit_binary_op(
        &mut self,
        expr_op: BinaryOp
    ) -> Result<(), (String, Vec<TokenMetadata>)> {
        let popped_left = self.exprs.pop();
        let popped_right = self.exprs.pop();

        let (left, right) = match (popped_left, popped_right) {
            (Some(left), Some(right)) => (left, right),
            (Some(left), None) => {
                let mut metadata = match left {
                    Expr::Literal(v) => v.get_token_metadata(),
                    _ => panic!("This is weird..."),
                };
                metadata.increment_length();
                metadata.increment_length();

                return Err((
                    "Expected right-hand side of binary operation".to_string(),
                    vec![metadata],
                ));
            }
            (x, y) => {
                println!("{:?} {:?}", x, y);
                panic!("This is also weird...")
            }
        };

        let binary_expr = BinaryExpr {
            left: Box::new(left),
            operator: expr_op,
            right: Box::new(right),
        };

        // let expr = match binary_expr.type_check_and_get_eval() {
        //     Ok(Some(expr)) => expr,
        //     Ok(None) => Expr::BinaryExpr(binary_expr),
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        let expr = Expr::BinaryExpr(binary_expr);

        self.exprs.push(expr);

        Ok(())
    }

    pub fn emit_unary_op(&mut self, expr_op: UnaryOp) -> Result<(), (String, Vec<TokenMetadata>)> {
        if self.panic_mode {
            return Ok(());
        }

        let right = self.exprs.pop().unwrap();

        let unary_expr = UnaryExpr {
            operator: expr_op,
            right: Box::new(right),
        };

        // let expr = match unary_expr.type_check_and_get_eval() {
        //     Ok(Some(expr)) => expr,
        //     Ok(None) => Expr::UnaryExpr(unary_expr),
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };

        let expr = Expr::UnaryExpr(unary_expr);

        self.exprs.push(expr);

        Ok(())
    }

    pub fn push_expr(&mut self) -> Result<(), (String, Vec<TokenMetadata>)> {
        if self.panic_mode {
            self.exit_panic_mode();
            Ok(())
        } else {
            match self.exprs.pop() {
                Some(expr) => {
                    match self.push_stmt(Stmt::ExprStmt(expr)) {
                        Ok(_) => Ok(()),
                        Err((e, token_vec)) => Err((e, token_vec)),
                    }
                }
                None => Ok(()), // Change this to an error
            }
        }
    }

    pub fn push_stmt(&mut self, stmt: Stmt) -> Result<(), (String, Vec<TokenMetadata>)> {
        let mut token_vec = Vec::new();
        let mut stmt = stmt;
        match stmt.type_check(&mut self.ast_environment, &mut token_vec) {
            Ok(_) => {
                self.ast.as_mut().unwrap().push_stmt(stmt);
                Ok(())
            }
            Err(e) => {
                return Err((e, token_vec));
            }
        }
    }

    pub fn pop_expr(&mut self) -> Option<Expr> {
        self.exprs.pop()
    }

    pub fn get_ast(&mut self) -> Ast {
        self.ast.take().unwrap()
    }
}
