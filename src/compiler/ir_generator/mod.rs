use crate::{ ast::{ Ast, Expr, ExprOp, Stmt }, error_handler::ErrorHandler };

use super::ir_graph::{ IRGraph, IRNode, IROperation, IRValue };

pub struct IRGenerator<'a> {
    ir_graph: Option<IRGraph<'a>>,
    next_register: usize,
    next_node_id: usize,
}

impl<'a> IRGenerator<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        Self {
            ir_graph: Some(IRGraph::new(error_handler)),
            next_register: 0,
            next_node_id: 1,
        }
    }

    #[profiler::function_tracker]
    pub fn generate_ir_from_ast(&mut self, ast: Ast) -> IRGraph {
        for stmt in ast.stmts {
            match stmt {
                Stmt::ExprStmt(expr) => self.compile_expr(expr),
                _ => {
                    panic!("Statement not supported in compiler yet");
                }
            }
        }

        self.ir_graph.take().unwrap()
    }

    fn compile_expr(&mut self, expr: Expr) {
        self.get_expr_node_id(expr);
    }

    fn get_expr_node_id(&mut self, expr: Expr) -> usize {
        let node_id: usize;
        match expr {
            Expr::BinaryExpr(binary_expr) => {
                let register = self.get_next_register();
                let op_id: usize;
                match binary_expr.operator {
                    ExprOp::Add => {
                        let add_node = IRNode::new(IROperation::Add, IRValue::Register(register));
                        op_id = self.add_node(add_node);
                        node_id = op_id;
                    }
                    ExprOp::Mul => {
                        let mul_node = IRNode::new(IROperation::Mul, IRValue::Register(register));
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    ExprOp::Sub => {
                        let mul_node = IRNode::new(IROperation::Sub, IRValue::Register(register));
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    ExprOp::Div => {
                        let mul_node = IRNode::new(IROperation::Div, IRValue::Register(register));
                        op_id = self.add_node(mul_node);
                        node_id = op_id;
                    }
                    _ => {
                        panic!("Operator not supported in compiler yet");
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
                    ExprOp::Neg => {
                        let neg_node = IRNode::new(IROperation::Neg, IRValue::Register(register));
                        op_id = self.add_node(neg_node);
                        node_id = op_id;
                    }
                    ExprOp::Truthy => {
                        let truthy_node = IRNode::new(
                            IROperation::Truthy,
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
                let node = IRNode::new(IROperation::NoOp, IRValue::Constant(value.get_value()));
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
}
