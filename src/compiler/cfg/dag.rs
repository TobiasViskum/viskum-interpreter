use ahash::AHashMap;

use crate::{
    operations::{ BinaryOp, UnaryOp },
    value_v2::Value,
    vm::instructions::{ Instruction, InstructionRegister, InstructionSrc },
};

use super::{ ChangedState, DefinitionState, IREnvironment, RegistersMap };

#[derive(Debug, Clone)]
pub enum DAGOp {
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Define,
    Assign,
    Const(Value),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct DAGNode {
    pub op: DAGOp,
    pub operands: Option<Vec<usize>>,
}

impl DAGNode {
    pub fn new(op: DAGOp, operands: Option<Vec<usize>>) -> Self {
        Self {
            op,
            operands,
        }
    }
}

#[derive(Debug)]
pub struct DAG {
    pub nodes: AHashMap<usize, DAGNode>,
    entry_node_id: usize,
}

impl DAG {
    pub fn new() -> Self {
        Self {
            nodes: AHashMap::default(),
            entry_node_id: 0,
        }
    }

    pub fn set_entry_node_id(&mut self, entry_node_id: usize) {
        self.entry_node_id = entry_node_id;
    }

    pub fn add_node(&mut self, node: DAGNode) -> usize {
        let index = if self.nodes.len() == 0 { 0 } else { self.nodes.len() };

        self.nodes.insert(index, node);
        index
    }

    fn add_node_at(&mut self, node: DAGNode, index: usize) {
        self.nodes.insert(index, node);
    }

    fn remove_node(&mut self, node_id: usize) {
        self.nodes.remove(&node_id);
    }

    pub fn generate_bytecode(&self, registers_maps: &mut RegistersMap) -> Vec<Instruction> {
        let mut bytecode = vec![];

        match self.generate_node_bytecode(self.entry_node_id, registers_maps, &mut bytecode) {
            InstructionSrc::Constant(value) => {
                if bytecode.len() == 0 {
                    let (register, scope) = registers_maps.assign_register();
                    bytecode.push(Instruction::Load {
                        reg: InstructionRegister::new(register, scope, false),
                        src: InstructionSrc::Constant(value),
                    });
                }
            }
            _ => {}
        }

        bytecode
    }

    fn generate_node_bytecode(
        &self,
        node_id: usize,
        registers_maps: &mut RegistersMap,
        bytecode: &mut Vec<Instruction>
    ) -> InstructionSrc {
        let node = self.nodes.get(&node_id).cloned().unwrap();

        match &node.op {
            DAGOp::Const(value) => { InstructionSrc::Constant(value.clone()) }
            DAGOp::Identifier(lexeme) => {
                let (register, scope) = registers_maps.get_register(lexeme).unwrap();
                InstructionSrc::Register(InstructionRegister::new(register, scope, true))
            }
            DAGOp::UnaryOp(unary_op) => {
                let operand = node.operands.unwrap()[0];
                let right = self.generate_node_bytecode(operand, registers_maps, bytecode);

                match &right {
                    InstructionSrc::Register(right_register) => {
                        if !right_register.is_variable {
                            registers_maps.free_register(
                                right_register.register,
                                right_register.scope
                            );
                        }
                    }
                    _ => {}
                }

                let (register, scope) = registers_maps.assign_register();

                let dest = InstructionRegister::new(register, scope, false);

                let instruction = match unary_op {
                    UnaryOp::Neg =>
                        Instruction::Neg {
                            dest,
                            src: right,
                        },
                    UnaryOp::Truthy =>
                        Instruction::Truthy {
                            dest,
                            src: right,
                        },
                };

                bytecode.push(instruction);
                InstructionSrc::Register(dest)
            }
            DAGOp::BinaryOp(binary_op) => {
                let operands = node.operands.unwrap();
                let left = self.generate_node_bytecode(operands[0], registers_maps, bytecode);
                let right = self.generate_node_bytecode(operands[1], registers_maps, bytecode);

                match &left {
                    InstructionSrc::Register(left_register) => {
                        if !left_register.is_variable {
                            registers_maps.free_register(
                                left_register.register,
                                left_register.scope
                            );
                        }
                    }
                    _ => {}
                }

                match &right {
                    InstructionSrc::Register(right_register) => {
                        if !right_register.is_variable {
                            registers_maps.free_register(
                                right_register.register,
                                right_register.scope
                            );
                        }
                    }
                    _ => {}
                }

                let (register, scope) = registers_maps.assign_register();

                let dest = InstructionRegister::new(register, scope, false);

                let instruction = match binary_op {
                    BinaryOp::Add =>
                        Instruction::Add {
                            dest,
                            src1: left,
                            src2: right,
                        },
                    BinaryOp::Sub =>
                        Instruction::Sub {
                            dest,
                            src1: left,
                            src2: right,
                        },
                    BinaryOp::Mul =>
                        Instruction::Mul {
                            dest,
                            src1: left,
                            src2: right,
                        },
                    BinaryOp::Div =>
                        Instruction::Div {
                            dest,
                            src1: left,
                            src2: right,
                        },
                };

                bytecode.push(instruction);
                InstructionSrc::Register(dest)
            }
            DAGOp::Define => {
                let operands = node.operands.unwrap();

                let lexeme = match &self.nodes.get(&operands[0]).unwrap().op {
                    DAGOp::Identifier(lexeme) => lexeme.clone(),
                    _ => panic!("Expected identifier"),
                };

                let value = match operands.get(1) {
                    Some(value_node_id) =>
                        self.generate_node_bytecode(*value_node_id, registers_maps, bytecode),
                    None => InstructionSrc::Constant(Value::Empty),
                };

                let (register, scope) = registers_maps.assign_variable_register(lexeme);

                let dest = InstructionRegister::new(register, scope, true);

                let instruction = Instruction::Define {
                    dest,
                    src: value,
                };

                bytecode.push(instruction);
                InstructionSrc::Register(dest)
            }
            DAGOp::Assign => {
                let operands = node.operands.unwrap();

                let lexeme = match &self.nodes.get(&operands[0]).unwrap().op {
                    DAGOp::Identifier(lexeme) => lexeme.clone(),
                    _ => panic!("Expected identifier"),
                };

                let value = match operands.get(1) {
                    Some(value_node_id) =>
                        self.generate_node_bytecode(*value_node_id, registers_maps, bytecode),
                    None => InstructionSrc::Constant(Value::Empty),
                };

                let (register, scope) = registers_maps.get_register(&lexeme).unwrap();

                let dest = InstructionRegister::new(register, scope, true);

                let instruction = Instruction::Assign {
                    dest,
                    src: value,
                };

                bytecode.push(instruction);
                InstructionSrc::Register(dest)
            }

            _ => { panic!("Unsupported operation") }
        }
    }

    pub fn constant_folding(&mut self, environment: &mut IREnvironment, scope: usize) {
        self.eval(self.nodes.len() - 1, environment, scope);
    }

    #[profiler::function_tracker]
    fn eval(
        &mut self,
        node_id: usize,
        environment: &mut IREnvironment,
        scope: usize
    ) -> Option<Value> {
        let node = self.nodes.get(&node_id).cloned().unwrap();

        match &node.op {
            DAGOp::Const(value) => { Some(value.clone()) }
            DAGOp::Identifier(lexeme) => {
                let (value, changed_state, _) = match environment.get(lexeme) {
                    Some(value) => value,
                    None => panic!("Variable not found"),
                };

                if changed_state == &ChangedState::Unchanged {
                    return value.clone();
                } else {
                    return None;
                }
            }
            DAGOp::BinaryOp(op) => {
                if let Some(operands) = &node.operands {
                    self.evaluate_binary_op(op, operands, node_id, environment, scope)
                } else {
                    None
                }
            }
            DAGOp::UnaryOp(op) => {
                if let Some(operands) = &node.operands {
                    self.evaluate_unary_op(op, operands[0], node_id, environment, scope)
                } else {
                    None
                }
            }
            DAGOp::Define | DAGOp::Assign => {
                if let Some(operands) = node.operands {
                    let is_definition = match &node.op {
                        DAGOp::Define => DefinitionState::IsDefinition,
                        _ => DefinitionState::IsAssignment,
                    };

                    let evaluated_value = match operands.get(1) {
                        Some(value_node_id) => { self.eval(*value_node_id, environment, scope) }
                        None => None,
                    };

                    let lexeme = match operands.get(0) {
                        Some(lexeme_node_id) => {
                            match &self.nodes.get(lexeme_node_id).unwrap().op {
                                DAGOp::Identifier(lexeme) => lexeme,
                                _ => panic!("Expected identifier"),
                            }
                        }
                        None => panic!("No lexeme node found"),
                    };

                    if is_definition == DefinitionState::IsDefinition {
                        environment.push(lexeme, evaluated_value.clone(), is_definition, scope);
                    } else {
                        environment.overwrite(&lexeme, evaluated_value.clone());
                    }

                    evaluated_value
                } else {
                    None
                }
            }
            _ => { panic!("Unsupported operation") }
        }
    }

    fn evaluate_unary_op(
        &mut self,
        op: &UnaryOp,
        operand: usize,
        node_id: usize,
        environment: &mut IREnvironment,
        scope: usize
    ) -> Option<Value> {
        let right = self.eval(operand, environment, scope);

        match right {
            Some(rhs) => {
                match op {
                    UnaryOp::Neg => {
                        let evaluated = rhs.neg().unwrap();
                        self.remove_node(operand);
                        self.add_node_at(
                            DAGNode::new(DAGOp::Const(evaluated.clone()), None),
                            node_id
                        );
                        Some(evaluated)
                    }
                    UnaryOp::Truthy => {
                        let evaluated = rhs.not().unwrap();
                        self.remove_node(operand);
                        self.add_node_at(
                            DAGNode::new(DAGOp::Const(evaluated.clone()), None),
                            node_id
                        );
                        Some(evaluated)
                    }
                }
            }
            _ => None,
        }
    }

    fn evaluate_binary_op(
        &mut self,
        op: &BinaryOp,
        operands: &Vec<usize>,
        node_id: usize,
        environment: &mut IREnvironment,
        scope: usize
    ) -> Option<Value> {
        let left = self.eval(operands[0], environment, scope);
        let right = self.eval(operands[1], environment, scope);

        match (left, right) {
            (Some(lhs), Some(rhs)) => {
                match op {
                    BinaryOp::Add => {
                        let evaluated = lhs.add(&rhs).unwrap();

                        self.remove_node(operands[0]);
                        self.remove_node(operands[1]);

                        self.add_node_at(
                            DAGNode::new(DAGOp::Const(evaluated.clone()), None),
                            node_id
                        );

                        Some(evaluated)
                    }
                    BinaryOp::Sub => {
                        let evaluated = lhs.sub(&rhs).unwrap();
                        self.remove_node(operands[0]);
                        self.remove_node(operands[1]);
                        self.add_node_at(
                            DAGNode::new(DAGOp::Const(evaluated.clone()), None),
                            node_id
                        );
                        Some(evaluated)
                    }
                    BinaryOp::Mul => {
                        let evaluated = lhs.mul(&rhs).unwrap();
                        self.remove_node(operands[0]);
                        self.remove_node(operands[1]);
                        self.add_node_at(
                            DAGNode::new(DAGOp::Const(evaluated.clone()), None),
                            node_id
                        );
                        Some(evaluated)
                    }
                    BinaryOp::Div => {
                        let evaluated = lhs.div(&rhs).unwrap();
                        self.remove_node(operands[0]);
                        self.remove_node(operands[1]);
                        self.add_node_at(
                            DAGNode::new(DAGOp::Const(evaluated.clone()), None),
                            node_id
                        );
                        Some(evaluated)
                    }
                }
            }
            _ => None,
        }
    }
}
