use std::rc::Rc;

use crate::{
    compiler::cfg::dag::{ DAGNode, DAGOp, DAG },
    error_handler::CompileError,
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value::{ Value, ValueType },
    vm::instructions::NativeCall,
};

use super::{
    ast_symbol_table::{ AstSymbol, AstSymbolTable, AstVariable },
    stmt::{ FunctionArgument, Stmt, VarAssignStmt },
};

#[derive(Debug)]
pub struct ExprBuilder {
    exprs: Vec<Expr>,
}

impl ExprBuilder {
    pub fn new() -> Self {
        Self {
            exprs: Vec::new(),
        }
    }

    pub fn get_built_expr(&mut self) -> Expr {
        self.exprs.pop().unwrap()
    }

    pub fn emit_native_call(&mut self, ast_native_call: AstNativeCall) {
        self.exprs.push(Expr::NativeFunctionCall(ast_native_call))
    }

    pub fn emit_fn_call(&mut self, ast_fn_call: AstFunctionCall) {
        self.exprs.push(Expr::FunctionCall(ast_fn_call))
    }

    pub fn emit_ident_lookup(&mut self, ast_identifier: AstIdentifier) {
        self.exprs.push(Expr::IdentifierLookup(ast_identifier))
    }

    pub fn emit_constant_literal(&mut self, ast_value: AstValue) {
        self.exprs.push(Expr::Literal(ast_value))
    }

    pub fn emit_unary_op(&mut self, unary_op: UnaryOp) -> Result<(), CompileError> {
        let rhs = self.exprs.pop().expect("Expected rhs of unary operation");

        let unary_expr = UnaryExpr {
            operator: unary_op,
            right: Box::new(rhs),
        };

        self.exprs.push(Expr::UnaryExpr(unary_expr));

        Ok(())
    }

    pub fn emit_binary_op(&mut self, binary_op: BinaryOp) -> Result<(), CompileError> {
        let popped_left = self.exprs.pop();
        let popped_right = self.exprs.pop();

        let (lhs, rhs) = match (popped_left, popped_right) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            (Some(lsh), None) => panic!("Missing right hand side of binary operation"),
            (lhs, rhs) =>
                panic!("Error in emit_binary_op: ExprBuilder: lhs: {:?}, rhs: {:?}", lhs, rhs),
        };

        let binary_expr = BinaryExpr {
            left: Box::new(lhs),
            operator: binary_op,
            right: Box::new(rhs),
        };

        self.exprs.push(Expr::BinaryExpr(binary_expr));

        Ok(())
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    Literal(AstValue),
    IdentifierLookup(AstIdentifier),
    FunctionCall(AstFunctionCall),
    NativeFunctionCall(AstNativeCall),
}

impl Expr {
    pub fn compile_to_dag(&self) -> DAG {
        let mut dag = DAG::new();

        let entry_node_id = self.compile_to_dag_node(&mut dag);

        dag.set_entry_node_id(entry_node_id);

        dag
    }

    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        match self {
            Expr::BinaryExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::UnaryExpr(expr) => expr.compile_to_dag_node(dag),
            Expr::Literal(ast_value) => ast_value.compile_to_dag_node(dag),
            Expr::IdentifierLookup(ast_identifier) => ast_identifier.compile_to_dag_node(dag),
            Expr::FunctionCall(ast_fn_call) => ast_fn_call.compile_to_dag_node(dag),
            Expr::NativeFunctionCall(ast_native_call) => ast_native_call.compile_to_dag_node(dag),
        }
    }

    pub fn type_check(
        &mut self,
        ast_symbol_table: &AstSymbolTable,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        match self {
            Expr::NativeFunctionCall(func_call) => {
                let (min_args, max_args) = func_call.native_call.get_args_count();

                if func_call.args.len() < min_args || func_call.args.len() > max_args {
                    token_vec.push(func_call.metadata);

                    if min_args == max_args {
                        return Err(
                            format!(
                                "Function '{}' takes {} arguments but received {} arguments",
                                func_call.native_call.get_lexeme(),
                                min_args,
                                func_call.args.len()
                            )
                        );
                    } else {
                        return Err(
                            format!(
                                "Function '{}' takes between {} and {} arguments but received {} arguments",
                                func_call.native_call.get_lexeme(),
                                min_args,
                                max_args,
                                func_call.args.len()
                            )
                        );
                    }
                }

                for arg in &mut func_call.args {
                    match arg.type_check(ast_symbol_table, token_vec) {
                        Ok(v) => {
                            // Compare if the arg types matches the arg types of the function
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                Ok(func_call.native_call.get_return_type())
            }
            Expr::FunctionCall(func_call) => {
                let function = match ast_symbol_table.get(&func_call.lexeme) {
                    Some(symbol) =>
                        match symbol {
                            AstSymbol::Function(f) => f,
                            _ => {
                                token_vec.push(func_call.metadata);
                                return Err(
                                    format!(
                                        "Undefined function '{}'. A variable with the same name exists",
                                        func_call.lexeme
                                    )
                                );
                            }
                        }
                    None => {
                        token_vec.push(func_call.metadata);
                        return Err(format!("Undefined function '{}'", func_call.lexeme));
                    }
                };

                for arg in &mut func_call.args {
                    match arg.type_check(ast_symbol_table, token_vec) {
                        Ok(v) => {
                            // Compare if the arg types matches the arg types of the function
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }

                for func_arg in &function.args {
                    (*func_call).real_function_args.push(Rc::clone(&func_arg.name));
                }

                (*func_call).return_type = function.return_type;

                Ok(function.return_type)
            } // return type
            Expr::BinaryExpr(expr) => expr.type_check(ast_symbol_table, token_vec),
            Expr::UnaryExpr(expr) => expr.type_check(ast_symbol_table, token_vec),
            Expr::Literal(ast_value) => Ok(ast_value.value.to_value_type()),
            Expr::IdentifierLookup(ast_identifier) => {
                if let Some(symbol) = ast_symbol_table.get(&ast_identifier.lexeme) {
                    let value_type = match symbol {
                        AstSymbol::Error => {
                            panic!("Couldn't look up identifier: {}", &ast_identifier.lexeme)
                        }
                        AstSymbol::Variable(var) => var.value_type,
                        AstSymbol::Function(_) =>
                            panic!("Uncalled functions cannot be used to arithmetic"),
                    };
                    Ok(value_type)
                } else {
                    token_vec.push(ast_identifier.token_metadata);
                    return Err(format!("Undefined variable: '{}'", ast_identifier.lexeme));
                }
            }
        }
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        match self {
            Expr::NativeFunctionCall(_) => panic!("error type check 2"),
            Expr::FunctionCall(_) => panic!("error type check 2"),
            Expr::BinaryExpr(expr) => expr.push_to_token_vec(token_vec),
            Expr::UnaryExpr(expr) => expr.push_to_token_vec(token_vec),
            Expr::Literal(ast_value) => ast_value.push_to_token_vec(token_vec),
            Expr::IdentifierLookup(ast_identifier) => ast_identifier.push_to_token_vec(token_vec),
        }
    }
}

#[derive(Debug)]
pub struct AstNativeCall {
    metadata: TokenMetadata,
    args: Vec<Expr>,
    native_call: NativeCall,
}

impl AstNativeCall {
    pub fn new(metadata: TokenMetadata, args: Vec<Expr>, native_call: NativeCall) -> Self {
        Self {
            metadata,
            args,
            native_call,
        }
    }

    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let mut operand_ids = vec![];

        for arg in &self.args {
            let value_node = arg.compile_to_dag_node(dag);
            operand_ids.push(value_node);
        }

        let op = DAGOp::NativeCall(self.native_call);

        let dag_node = DAGNode::new(op, Some(operand_ids));

        dag.add_node(dag_node)
    }
}

#[derive(Debug)]
pub struct AstFunctionCall {
    lexeme: Rc<str>,
    metadata: TokenMetadata,
    args: Vec<Expr>,
    real_function_args: Vec<Rc<str>>,
    return_type: ValueType,
}

impl AstFunctionCall {
    pub fn new(
        lexeme: Rc<str>,
        metadata: TokenMetadata,
        args: Vec<Expr>,
        real_function_args: Vec<Rc<str>>,
        return_type: ValueType
    ) -> Self {
        Self {
            lexeme,
            metadata,
            args,
            real_function_args,
            return_type,
        }
    }

    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let mut operand_ids = vec![];

        for arg in &self.args {
            let value_node = arg.compile_to_dag_node(dag);
            operand_ids.push(value_node);
        }

        let op = DAGOp::FnCall(self.lexeme.clone());

        let dag_node = DAGNode::new(op, Some(operand_ids));

        dag.add_node(dag_node)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: BinaryOp,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let left_dag = self.left.compile_to_dag_node(dag);
        let right_dag = self.right.compile_to_dag_node(dag);

        let op = DAGOp::BinaryOp(self.operator);

        let dag_node = DAGNode::new(op, Some(vec![left_dag, right_dag]));

        dag.add_node(dag_node)
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        self.left.push_to_token_vec(token_vec);
        self.right.push_to_token_vec(token_vec);
    }

    pub fn type_check(
        &mut self,
        ast_symbol_table: &AstSymbolTable,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        let left_type = self.left.type_check(ast_symbol_table, token_vec)?;
        let right_type = self.right.type_check(ast_symbol_table, token_vec)?;

        match left_type.type_check_binary(&right_type, self.operator) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.left.push_to_token_vec(token_vec);
                self.right.push_to_token_vec(token_vec);
                Err(e)
            }
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let right_dag = self.right.compile_to_dag_node(dag);

        let op = DAGOp::UnaryOp(self.operator);

        let mut dag_node = DAGNode::new(op, Some(vec![right_dag]));

        dag.add_node(dag_node)
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        self.right.push_to_token_vec(token_vec);
    }

    pub fn type_check(
        &mut self,
        ast_symbol_table: &AstSymbolTable,
        token_vec: &mut Vec<TokenMetadata>
    ) -> Result<ValueType, String> {
        let right_type = self.right.type_check(ast_symbol_table, token_vec)?;

        match right_type.type_check_unary(self.operator) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.right.push_to_token_vec(token_vec);
                if token_vec.len() > 0 {
                    let mutable_right = token_vec.get_mut(0).unwrap();
                    for _ in 0..self.operator.get_op_len() {
                        mutable_right.decrement_start();
                        mutable_right.increment_length();
                    }
                }
                Err(e)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstValue {
    pub value: Value,
    pub token_metadata: TokenMetadata,
}

impl AstValue {
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let op = DAGOp::Const(self.value.clone());

        let mut dag_node = DAGNode::new(op, None);

        dag.add_node(dag_node)
    }

    pub fn new(value: Value, token_metadata: TokenMetadata) -> Self {
        Self {
            value,
            token_metadata,
        }
    }

    pub fn get_value(&self) -> Value {
        self.value.clone()
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
    pub lexeme: Rc<str>,
    pub token_metadata: TokenMetadata,
}

impl AstIdentifier {
    pub fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        let op = DAGOp::Identifier(self.lexeme.clone());

        let mut dag_node = DAGNode::new(op, None);

        dag.add_node(dag_node)
    }

    pub fn new(lexeme: Rc<str>, token_metadata: TokenMetadata) -> Self {
        Self {
            lexeme,
            token_metadata,
        }
    }

    pub fn get_lexeme(&self) -> Rc<str> {
        self.lexeme.clone()
    }

    pub fn get_token_metadata(&self) -> TokenMetadata {
        self.token_metadata
    }

    pub fn push_to_token_vec(&self, token_vec: &mut Vec<TokenMetadata>) {
        token_vec.push(self.token_metadata);
    }
}
