use crate::{
    ast::{ expr::Expr, stmt::Stmt, Ast },
    operations::{ BinaryOp, Op, UnaryOp },
    vm::instructions::helper_structs::InstructionRegister,
};

use super::{ ir_graph::{ IRGraph, IRNode, IRValue }, Environment, Variable };

pub struct IRGenerator<'a> {
    ir_graph: Option<IRGraph>,
    next_register: usize,
    next_node_id: usize,
    environment: &'a mut Environment,
    scope_depth: usize,
}

impl<'a> IRGenerator<'a> {
    pub fn new(environment: &'a mut Environment) -> Self {
        Self {
            ir_graph: Some(IRGraph::new()),
            next_register: 0,
            next_node_id: 1,
            environment,
            scope_depth: 0,
        }
    }

    fn start_scope(&mut self) {
        self.environment.start_scope();
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.environment.end_scope();
        self.scope_depth -= 1;
    }

    fn generate_ir_from_stmt(&mut self, stmt: &Stmt, linked_ids: &mut Vec<usize>) {
        match stmt {
            Stmt::ScopeStmt(scope_stmt) => {
                self.start_scope();
                let start_scope_id = self.add_node(IRNode {
                    operation: Op::StartScope,
                    result: IRValue::VariableRegister(
                        InstructionRegister::new(0, self.scope_depth)
                    ),
                });
                linked_ids.push(start_scope_id);
                for stmt in &scope_stmt.stmts {
                    self.generate_ir_from_stmt(&stmt, linked_ids);
                }
                let end_scope_id = self.add_node(IRNode {
                    operation: Op::EndScope,
                    result: IRValue::VariableRegister(
                        InstructionRegister::new(0, self.scope_depth)
                    ),
                });
                linked_ids.push(end_scope_id);
                self.end_scope();
            }
            Stmt::ExprStmt(expr) => {
                let expr_id = self.compile_expr(expr);
                linked_ids.push(expr_id);
            }
            Stmt::VariableDefinition(variable_definition) => {
                let next_register = self.get_next_register();

                let (subscript, _, _) = self.environment.count_variables_of_name(
                    &variable_definition.name
                );

                let subscript = subscript + 1;

                self.environment.insert(
                    variable_definition.name.clone(),
                    Variable::new(
                        next_register,
                        variable_definition.value_type.unwrap(),
                        variable_definition.is_mutable
                    ),
                    subscript,
                    true
                );

                let definition_id = self.add_node(IRNode {
                    operation: Op::Define,
                    result: IRValue::VariableRegister(
                        InstructionRegister::new(next_register, self.scope_depth)
                    ),
                });
                if let Some(value) = &variable_definition.value {
                    let value_id = self.compile_expr(&value);
                    self.add_edge(value_id, definition_id);
                }
                linked_ids.push(definition_id);
            }
            Stmt::VariableAssignment(variable_assignment) => {
                let next_register = self.get_next_register();
                let variable_name = variable_assignment.field.get_lexeme();
                let (subscript, value_type, is_mutable) = self.environment.count_variables_of_name(
                    &variable_name
                );
                let subscript = subscript + 1;
                let value_type = value_type.unwrap();

                let assign_id = self.add_node(IRNode {
                    operation: Op::Assign,
                    result: IRValue::VariableRegister(
                        InstructionRegister::new(next_register, self.scope_depth)
                    ),
                });

                let value_id = self.compile_expr(&variable_assignment.value);
                self.add_edge(value_id, assign_id);

                // let (register, scope) = self.environment.get_latest_variable_definition(
                //     &variable_name
                // );

                // let definition_reference_id = self.add_node(IRNode {
                //     operation: Op::NoOp,
                //     result: IRValue::DefinitionReference(
                //         InstructionRegister::new(register.unwrap(), scope.unwrap())
                //     ),
                // });
                // self.add_edge(definition_reference_id, assign_id);

                self.environment.insert(
                    variable_name,
                    Variable::new(next_register, value_type, is_mutable),
                    subscript,
                    false
                );

                linked_ids.push(assign_id);
            }
        }
    }

    #[profiler::function_tracker]
    pub fn generate_ir_from_ast(&mut self, ast: Ast) -> IRGraph {
        let mut linked_ids = Vec::new();

        for stmt in &ast.main_scope.stmts {
            self.generate_ir_from_stmt(stmt, &mut linked_ids);
        }

        if linked_ids.len() > 0 {
            for i in 0..linked_ids.len() - 1 {
                let this_id = linked_ids.get(i).unwrap();
                if let Some(next_id) = linked_ids.get(i + 1) {
                    self.add_control_flow_edge(*next_id, *this_id);
                }
            }
        }

        self.ir_graph.take().unwrap()
    }

    fn compile_expr(&mut self, expr: &Expr) -> usize {
        self.get_expr_node_id(expr)
    }

    fn get_expr_node_id(&mut self, expr: &Expr) -> usize {
        let node_id: usize;
        match expr {
            Expr::BinaryExpr(binary_expr) => {
                let register = self.get_next_register();
                let op_id: usize;

                match binary_expr.operator {
                    BinaryOp::Add => {
                        let add_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Add),
                            IRValue::Register(InstructionRegister::new(register, self.scope_depth))
                        );
                        op_id = self.add_node(add_node);
                        node_id = op_id;
                    }
                    BinaryOp::Mul => {
                        let mul_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Mul),
                            IRValue::Register(InstructionRegister::new(register, self.scope_depth))
                        );
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    BinaryOp::Sub => {
                        let mul_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Sub),
                            IRValue::Register(InstructionRegister::new(register, self.scope_depth))
                        );
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    BinaryOp::Div => {
                        let mul_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Div),
                            IRValue::Register(InstructionRegister::new(register, self.scope_depth))
                        );
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                }

                let (left_id, right_id) = (
                    self.get_expr_node_id(&binary_expr.left) as usize,
                    self.get_expr_node_id(&binary_expr.right) as usize,
                );

                self.add_edge(right_id, op_id);
                self.add_edge(left_id, op_id);
            }
            Expr::UnaryExpr(unary_expr) => {
                let register = self.get_next_register();
                let op_id: usize;
                match unary_expr.operator {
                    UnaryOp::Neg => {
                        let neg_node = IRNode::new(
                            Op::UnaryOp(UnaryOp::Neg),
                            IRValue::Register(InstructionRegister::new(register, self.scope_depth))
                        );
                        op_id = self.add_node(neg_node);
                        node_id = op_id;
                    }
                    UnaryOp::Truthy => {
                        let truthy_node = IRNode::new(
                            Op::UnaryOp(UnaryOp::Truthy),
                            IRValue::Register(InstructionRegister::new(register, self.scope_depth))
                        );
                        op_id = self.add_node(truthy_node);
                        node_id = op_id;
                    }
                }

                let expr_id = self.get_expr_node_id(&unary_expr.right) as usize;

                self.add_edge(expr_id, op_id);
            }
            Expr::Literal(value) => {
                let node = IRNode::new(Op::NoOp, IRValue::Constant(value.get_value()));
                node_id = self.add_node(node);
                let next_register = self.get_next_register();
                let node_register = self.add_node(
                    IRNode::new(
                        Op::NoOp,
                        IRValue::Register(InstructionRegister::new(next_register, self.scope_depth))
                    )
                );

                self.add_edge(node_register, node_id);
            }
            Expr::IdentifierLookup(ast_identifier) => {
                let found_variable = self.environment.get_variable_with_highest_subscript(
                    &ast_identifier.get_lexeme()
                );
                let (variable, scope) = (found_variable.0.unwrap(), found_variable.1.unwrap());

                let node = IRNode::new(
                    Op::NoOp,
                    IRValue::VariableRegister(InstructionRegister::new(variable.location, scope))
                );
                node_id = self.add_node(node);
            }
        }

        node_id
    }

    fn get_next_register(&mut self) -> usize {
        let register = self.next_register;
        self.next_register += 1;
        register
    }

    fn add_node(&mut self, node: IRNode) -> usize {
        let node_id = self.next_node_id;
        self.ir_graph.as_mut().unwrap().add_node(node_id, node);
        self.next_node_id += 1;
        node_id
    }

    fn _remove_node(&mut self, node_id: usize) {
        self.next_node_id -= 1;
        self.ir_graph.as_mut().unwrap()._remove_node(node_id);
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.ir_graph.as_mut().unwrap().add_edge(src, dest);
    }

    fn add_control_flow_edge(&mut self, src: usize, dest: usize) {
        self.ir_graph.as_mut().unwrap().add_control_flow_edge(src, dest);
    }
}
