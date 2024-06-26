mod optimize_registers;

use std::cell::RefCell;

use ahash::AHashMap;

use crate::{
    constants::REGISTERS,
    operations::{ BinaryOp, Op, UnaryOp },
    value::Value,
    vm::instructions::{ helper_structs::InstructionRegister, IRInstruction },
};

use super::ir_graph::{ IRControlFlowEdge, IREdge, IRGraph, IRNode, IRValue };

#[derive(Debug)]
pub struct BytecodeScopeRegisters {
    virtual_registers: RefCell<AHashMap<usize, usize>>,
    available_registers: RefCell<Vec<usize>>,
}

impl BytecodeScopeRegisters {
    #[profiler::function_tracker]
    pub fn new() -> Self {
        let available_registers: Vec<usize> = (0..REGISTERS).rev().collect();

        Self {
            virtual_registers: RefCell::new(AHashMap::default()),
            available_registers: RefCell::new(available_registers),
        }
    }
}

pub struct BytecodeGenerator<'a> {
    ir_graph: &'a IRGraph,
    instructions: Vec<IRInstruction>,
    bytecode_scope_registers: Vec<BytecodeScopeRegisters>,
    scope_depth: usize,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(ir_graph: &'a IRGraph) -> Self {
        Self {
            ir_graph: ir_graph,
            instructions: Vec::with_capacity(256),
            bytecode_scope_registers: vec![BytecodeScopeRegisters::new()],
            scope_depth: 0,
        }
    }

    #[profiler::function_tracker]
    pub fn generate_bytecode(&mut self) {
        let mut node_id = 1;

        loop {
            self.generate_instruction_from_node(node_id);

            let control_flow_edge = self.get_control_flow_edge_to_node(node_id);

            if let Some(control_flow_edge) = control_flow_edge {
                node_id = control_flow_edge.src;
            } else {
                break;
            }
        }

        self.instructions.push(IRInstruction::Halt);
    }

    pub fn dissassemble(&self) {
        for instruction in &self.instructions {
            println!("{}", instruction.dissassemble());
        }
    }

    #[profiler::function_tracker]
    fn generate_instruction_from_node(&mut self, node_id: usize) {
        let node = self.get_node(node_id).cloned();

        if let Some(node) = node {
            match &node.operation {
                Op::StartScope => {
                    let instruction = IRInstruction::StartScope;
                    self.scope_depth += 1;
                    self.instructions.push(instruction);
                }
                Op::EndScope => {
                    let instruction = IRInstruction::EndScope;
                    self.scope_depth -= 1;
                    self.instructions.push(instruction);
                }
                Op::BinaryOp(binary_op) =>
                    self.generate_binary_instruction(&node, node_id, binary_op),
                Op::UnaryOp(unary_op) => self.generate_unary_instruction(&node, node_id, unary_op),
                Op::Assign => self.generate_assignment_instruction(&node, node_id),
                Op::Define => self.generate_definement_instruction(&node, node_id),
                Op::NoOp => {
                    match node.result {
                        IRValue::Constant(value) => {
                            let edge = self.get_edge_to_node(node_id).unwrap();
                            let adj_node = self.get_node(edge.src).unwrap();

                            match &adj_node.result {
                                IRValue::Register(instruction_register) => {
                                    let instruction = IRInstruction::new_load(
                                        *instruction_register,
                                        IRValue::Constant(value)
                                    );
                                    self.instructions.push(instruction);
                                }
                                _ => panic!("Could not generate LOAD instruction."),
                            }
                        }
                        _ => {
                            panic!("Unsupported operation.");
                        }
                    }
                }
            }
        }
    }

    fn generate_assignment_instruction(&mut self, node: &IRNode, node_id: usize) {
        let mut dest = match &node.result {
            IRValue::VariableRegister(register) => *register,
            IRValue::Register(_) | IRValue::Constant(_) | IRValue::DefinitionReference(_) => {
                panic!("Unsupported operation.");
            }
        };

        let mut instruction_value: IRValue = IRValue::Constant(Value::Empty);

        let edges_to_this_node = self.get_edges_to_node(node_id);
        for edge_to_this_node in &edges_to_this_node {
            let edge_src = edge_to_this_node.src;

            let adj_node = self.get_node(edge_src).unwrap().clone();

            match &adj_node.result {
                IRValue::Constant(value) => {
                    instruction_value = IRValue::Constant(*value);
                    // let instruction = IRInstruction::new_assign(*dest, IRValue::Constant(*value));
                    // self.instructions.push(instruction);
                }
                IRValue::Register(register) => {
                    self.generate_instruction_from_node(edge_src);
                    instruction_value = IRValue::Register(*register);
                    // let instruction = IRInstruction::new_assign(
                    //     *dest,
                    //     IRValue::Register(*register)
                    // );
                    // self.instructions.push(instruction);
                }
                IRValue::VariableRegister(register) => {
                    instruction_value = IRValue::Register(*register);
                    // let instruction = IRInstruction::new_assign(
                    //     *dest,
                    //     IRValue::Register(*register)
                    // );
                    // self.instructions.push(instruction);
                }
                IRValue::DefinitionReference(register) => {
                    // dest = *register;
                }
            }
        }

        let instruction = IRInstruction::new_assign(dest, instruction_value);
        self.instructions.push(instruction);
    }

    fn generate_definement_instruction(&mut self, node: &IRNode, node_id: usize) {
        let dest = match &node.result {
            IRValue::VariableRegister(register) => register,
            IRValue::Register(_) | IRValue::Constant(_) | IRValue::DefinitionReference(_) => {
                panic!("Unsupported operation.");
            }
        };

        let edge_to_this_node = self.get_edge_to_node(node_id);

        if let Some(edge_to_this_node) = edge_to_this_node {
            let edge_src = edge_to_this_node.src;

            let adj_node = self.get_node(edge_src).unwrap().clone();

            match &adj_node.result {
                IRValue::Constant(value) => {
                    let instruction = IRInstruction::new_define(*dest, IRValue::Constant(*value));
                    self.instructions.push(instruction);
                }
                IRValue::Register(register) => {
                    self.generate_instruction_from_node(edge_src);
                    let instruction = IRInstruction::new_define(
                        *dest,
                        IRValue::Register(*register)
                    );
                    self.instructions.push(instruction);
                }
                IRValue::VariableRegister(register) => {
                    let instruction = IRInstruction::new_define(
                        *dest,
                        IRValue::Register(*register)
                    );
                    self.instructions.push(instruction);
                }
                IRValue::DefinitionReference(_) => {
                    panic!("Unsupported operation.");
                }
            }
        } else {
            let instruction = IRInstruction::new_define(*dest, IRValue::Constant(Value::Empty));
            self.instructions.push(instruction);
        }
    }

    fn generate_unary_instruction(&mut self, node: &IRNode, node_id: usize, unary_op: &UnaryOp) {
        let dest = match &node.result {
            IRValue::Register(register) => register,
            | IRValue::VariableRegister(_)
            | IRValue::Constant(_)
            | IRValue::DefinitionReference(_) => {
                panic!("Unsupported operation.");
            }
        };

        let src: IRValue;

        let edge_to_this_node = self.get_edge_to_node(node_id).unwrap();
        let edge_src = edge_to_this_node.src;

        let adj_node = self.get_node(edge_src).unwrap();

        match &adj_node.result {
            IRValue::Constant(value) => {
                src = IRValue::Constant(*value);
            }
            IRValue::Register(register) => {
                src = IRValue::Register(*register);
                self.generate_instruction_from_node(edge_src);
            }
            IRValue::VariableRegister(register) => {
                src = IRValue::Register(*register);
                self.generate_instruction_from_node(edge_src);
            }
            IRValue::DefinitionReference(_) => {
                panic!("Unsupported operation.");
            }
        }

        let instruction = IRInstruction::new_unary(unary_op, *dest, src);
        self.instructions.push(instruction);
    }

    fn generate_binary_instruction(&mut self, node: &IRNode, node_id: usize, binary_op: &BinaryOp) {
        let (src1, src2) = self.get_binary_sources(node_id);

        let dest = match &node.result {
            IRValue::Register(register) =>
                InstructionRegister::new(register.register, register.scope),
            | IRValue::VariableRegister(_)
            | IRValue::Constant(_)
            | IRValue::DefinitionReference(_) => {
                panic!("Unsupported operation.");
            }
        };

        let instruction = IRInstruction::new_binary(binary_op, dest, src1, src2);
        self.instructions.push(instruction);
    }

    fn get_binary_sources(&mut self, node_id: usize) -> (IRValue, IRValue) {
        let mut src1: Option<IRValue> = None;
        let mut src2: Option<IRValue> = None;

        for edge in self.get_edges_to_node(node_id) {
            let adj_node = self.get_node(edge.src).unwrap();

            match &adj_node.result {
                IRValue::Constant(value) => {
                    if src1.is_none() {
                        src1 = Some(IRValue::Constant(*value));
                    } else {
                        src2 = Some(IRValue::Constant(*value));
                    }
                }
                IRValue::Register(register) => {
                    if src1.is_none() {
                        src1 = Some(
                            IRValue::Register(
                                InstructionRegister::new(register.register, register.scope)
                            )
                        );
                    } else {
                        src2 = Some(
                            IRValue::Register(
                                InstructionRegister::new(register.register, register.scope)
                            )
                        );
                    }
                    self.generate_instruction_from_node(edge.src);
                }
                IRValue::VariableRegister(register) => {
                    if src1.is_none() {
                        src1 = Some(
                            IRValue::VariableRegister(
                                InstructionRegister::new(register.register, register.scope)
                            )
                        );
                    } else {
                        src2 = Some(
                            IRValue::VariableRegister(
                                InstructionRegister::new(register.register, register.scope)
                            )
                        );
                    }
                }
                IRValue::DefinitionReference(_) => {
                    panic!("Unsupported operation.");
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

    fn get_edge_to_node(&self, node_id: usize) -> Option<&IREdge> {
        self.ir_graph.get_edge_to_node(node_id)
    }

    fn get_control_flow_edge_to_node(&self, node_id: usize) -> Option<&IRControlFlowEdge> {
        self.ir_graph.get_control_flow_edge_to_node(node_id)
    }
}
