mod optimize_registers;

use std::{ cell::RefCell, collections::HashMap };

use crate::{ constants::REGISTERS, vm::instructions::Instruction };

use super::ir_graph::{ IREdge, IRGraph, IRNode, IROperation, IRValue };

pub struct BytecodeGenerator<'a> {
    ir_graph: IRGraph<'a>,
    instructions: Vec<Instruction>,
    virtual_registers: RefCell<HashMap<usize, usize>>,
    available_registers: RefCell<Vec<usize>>,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(ir_graph: IRGraph<'a>) -> Self {
        let mut available_registers = Vec::new();
        for i in (1..REGISTERS).rev() {
            available_registers.push(i - 1);
        }

        Self {
            ir_graph: ir_graph,
            instructions: Vec::with_capacity(256),
            virtual_registers: RefCell::new(HashMap::new()),
            available_registers: RefCell::new(available_registers),
        }
    }

    #[profiler::function_tracker]
    pub fn generate_bytecode(&mut self) {
        let root_node_id = 1;

        self.new_instruction_from_node(root_node_id);

        self.instructions.push(Instruction::Halt);
    }

    pub fn dissassemble(&self) {
        for instruction in &self.instructions {
            println!("{}", instruction.dissassemble());
        }
    }

    pub fn get_instructions(&mut self) -> Vec<Instruction> {
        self.instructions.clone()
    }

    #[profiler::function_tracker]
    fn new_instruction_from_node(&mut self, node_id: usize) {
        let node = self.get_node(node_id).cloned();
        if let Some(node) = node {
            match node.operation {
                IROperation::Add | IROperation::Sub | IROperation::Mul | IROperation::Div => {
                    self.generate_binary_instruction(&node, node_id);
                }
                IROperation::Neg | IROperation::Truthy => {
                    self.generate_unary_instruction(&node, node_id);
                }
                IROperation::NoOp => {
                    match node.result {
                        IRValue::Constant(value) => {
                            let instruction = Instruction::new_load(IRValue::Register(0), value);
                            self.instructions.push(instruction);
                        }
                        _ => {
                            panic!("Unsupported operation.");
                        }
                    }
                }
            }
        }
    }

    fn generate_unary_instruction(&mut self, node: &IRNode, node_id: usize) {
        let dest = match node.result {
            IRValue::Register(register) => register,
            _ => {
                panic!("Unsupported operation.");
            }
        };

        let src: IRValue;

        let edge_to_this_node = self.get_edge_to_node(node_id);
        let edge_src = edge_to_this_node.src.clone();

        let adj_node = self.get_node(edge_src).unwrap().clone();

        match adj_node.result {
            IRValue::Constant(value) => {
                src = IRValue::Constant(value);
            }
            IRValue::Register(register) => {
                src = IRValue::Register(register);
                self.new_instruction_from_node(edge_src);
            }
        }

        let instruction = Instruction::new_unary(node.operation.clone(), dest, src);
        self.instructions.push(instruction);
    }

    fn generate_binary_instruction(&mut self, node: &IRNode, node_id: usize) {
        let operation = node.operation.clone();
        let (src1, src2) = self.get_binary_sources(node_id);

        let dest = match node.result {
            IRValue::Register(register) => register,
            _ => {
                panic!("Unsupported operation.");
            }
        };

        let instruction = Instruction::new_binary(operation, dest, src1, src2);
        self.instructions.push(instruction);
    }

    fn get_binary_sources(&mut self, node_id: usize) -> (IRValue, IRValue) {
        let mut src1: Option<IRValue> = None;
        let mut src2: Option<IRValue> = None;

        for edge in self.get_edges_to_node(node_id) {
            let adj_node = self.get_node(edge.src.clone()).unwrap();

            match adj_node.result {
                IRValue::Constant(value) => {
                    if src1.is_none() {
                        src1 = Some(IRValue::Constant(value));
                    } else {
                        src2 = Some(IRValue::Constant(value));
                    }
                }
                IRValue::Register(register) => {
                    if src1.is_none() {
                        src1 = Some(IRValue::Register(register));
                    } else {
                        src2 = Some(IRValue::Register(register));
                    }
                    self.new_instruction_from_node(edge.src.clone());
                }
            }
        }

        (src1.unwrap(), src2.unwrap())
    }

    fn get_node(&self, node_id: usize) -> Option<&IRNode> {
        self.ir_graph.get_node(node_id)
    }

    fn get_edges_to_node(&self, node_id: usize) -> Vec<IREdge> {
        self.ir_graph.get_edges_to_node(node_id)
    }

    fn get_edge_to_node(&self, node_id: usize) -> &IREdge {
        self.ir_graph.get_edge_to_node(node_id)
    }
}
