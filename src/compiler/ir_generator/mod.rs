use crate::{
    ast::{ Ast, Expr, Stmt },
    error_handler::ErrorHandler,
    operations::{ BinaryOp, Op, UnaryOp },
};

use super::{ ir_graph::{ IRGraph, IRNode, IRValue }, Environment, Variable };

pub struct IRGenerator<'a> {
    ir_graph: Option<IRGraph<'a>>,
    next_register: usize,
    next_node_id: usize,
    environment: &'a mut Environment,
}

impl<'a> IRGenerator<'a> {
    pub fn new(error_handler: &'a ErrorHandler, environment: &'a mut Environment) -> Self {
        Self {
            ir_graph: Some(IRGraph::new(error_handler)),
            next_register: 0,
            next_node_id: 1,
            environment,
        }
    }

    #[profiler::function_tracker]
    pub fn generate_ir_from_ast(&mut self, ast: Ast) -> IRGraph {
        let mut linked_ids = Vec::new();

        for stmt in ast.stmts {
            match stmt {
                Stmt::ExprStmt(expr) => {
                    let expr_id = self.compile_expr(expr);
                    linked_ids.push(expr_id);
                }
                Stmt::VariableDefinition(variable_definition) => {
                    let next_register = self.get_next_register();

                    self.environment.insert(
                        variable_definition.name,
                        Variable::new(
                            next_register,
                            variable_definition.value_type,
                            variable_definition.is_mutable
                        )
                    );

                    let definition_id = self.add_node(IRNode {
                        operation: Op::Define,
                        result: IRValue::VariableRegister(next_register),
                    });
                    let value_id = self.compile_expr(variable_definition.value);
                    self.add_edge(value_id, definition_id);
                    linked_ids.push(definition_id);
                }
            }
        }

        for i in 0..linked_ids.len() - 1 {
            let this_id = linked_ids.get(i).unwrap();
            if let Some(next_id) = linked_ids.get(i + 1) {
                self.add_control_flow_edge(*next_id, *this_id);
            }
        }

        self.ir_graph.take().unwrap()
    }

    fn compile_expr(&mut self, expr: Expr) -> usize {
        self.get_expr_node_id(expr)
    }

    fn get_expr_node_id(&mut self, expr: Expr) -> usize {
        let node_id: usize;
        match expr {
            Expr::BinaryExpr(binary_expr) => {
                let register = self.get_next_register();
                let op_id: usize;
                match binary_expr.operator {
                    BinaryOp::Add => {
                        let add_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Add),
                            IRValue::Register(register)
                        );
                        op_id = self.add_node(add_node);
                        node_id = op_id;
                    }
                    BinaryOp::Mul => {
                        let mul_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Mul),
                            IRValue::Register(register)
                        );
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    BinaryOp::Sub => {
                        let mul_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Sub),
                            IRValue::Register(register)
                        );
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    BinaryOp::Div => {
                        let mul_node = IRNode::new(
                            Op::BinaryOp(BinaryOp::Div),
                            IRValue::Register(register)
                        );
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                }

                let (rigth_id, left_id) = (
                    self.get_expr_node_id(*binary_expr.left) as usize,
                    self.get_expr_node_id(*binary_expr.right) as usize,
                );

                self.add_edge(left_id, op_id);
                self.add_edge(rigth_id, op_id);

                // let evaluated_node = self.ir_graph
                //     .as_mut()
                //     .unwrap()
                //     .get_evaluated_node_if_constant(op_id);

                // if let Ok(Some(value)) = evaluated_node {
                //     self.remove_node(left_id);
                //     self.remove_node(rigth_id);
                //     self.remove_node(op_id);
                //     node_id = self.add_node(
                //         IRNode::new(IROperation::NoOp, IRValue::Constant(value))
                //     );
                // } else {
                //     match self.ir_graph.as_ref().unwrap().get_node(rigth_id).unwrap().get_result() {
                //         IRValue::Register(register) => {
                //             if !self.has_register(*register) {
                //                 self.available_registers.push(*register);
                //             }
                //         }
                //         _ => {}
                //     }

                //     match self.ir_graph.as_ref().unwrap().get_node(left_id).unwrap().get_result() {
                //         IRValue::Register(register) => {
                //             if !self.has_register(*register) {
                //                 self.available_registers.push(*register);
                //             }
                //         }
                //         _ => {}
                //     }
                // }
            }
            Expr::UnaryExpr(unary_expr) => {
                let register = self.get_next_register();
                let op_id: usize;
                match unary_expr.operator {
                    UnaryOp::Neg => {
                        let neg_node = IRNode::new(
                            Op::UnaryOp(UnaryOp::Neg),
                            IRValue::Register(register)
                        );
                        op_id = self.add_node(neg_node);
                        node_id = op_id;
                    }
                    UnaryOp::Truthy => {
                        let truthy_node = IRNode::new(
                            Op::UnaryOp(UnaryOp::Truthy),
                            IRValue::Register(register)
                        );
                        op_id = self.add_node(truthy_node);
                        node_id = op_id;
                    }
                    _ => {
                        panic!("Operator not supported in compiler yet");
                    }
                }

                let expr_id = self.get_expr_node_id(*unary_expr.right) as usize;

                self.add_edge(expr_id, op_id);
            }
            Expr::Literal(value) => {
                let node = IRNode::new(Op::NoOp, IRValue::Constant(value.get_value()));
                node_id = self.add_node(node);
            }
            Expr::VariableLookup(variable) => {
                let variable = self.environment.get(&variable.get_lexeme()).unwrap();
                let node = IRNode::new(Op::NoOp, IRValue::VariableRegister(variable.0.location));
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

    fn remove_node(&mut self, node_id: usize) {
        self.next_node_id -= 1;
        self.ir_graph.as_mut().unwrap().remove_node(node_id);
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.ir_graph.as_mut().unwrap().add_edge(src, dest);
    }

    fn add_control_flow_edge(&mut self, src: usize, dest: usize) {
        self.ir_graph.as_mut().unwrap().add_control_flow_edge(src, dest);
    }
}
