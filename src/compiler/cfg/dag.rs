use ahash::AHashMap;

use crate::{
    operations::{ BinaryOp, ComparisonOp, Op, UnaryOp },
    value::Value,
    vm::{ self, instructions::{ Instruction, InstructionPos, InstructionSrc } },
};

use super::{ ChangedState, DefinitionState, RegistersMap, VMSymbolTable };

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

    pub fn get_comparison_op(&self) -> ComparisonOp {
        match self.nodes.get(&self.entry_node_id).unwrap().op {
            DAGOp::BinaryOp(BinaryOp::ComparisonOp(op)) => op,
            _ => panic!("Expected comparison op"),
        }
    }

    pub fn ensure_comparison_to_entry_node(&mut self) -> ComparisonOp {
        let comparison_op = match self.nodes.get(&self.entry_node_id).unwrap().op {
            DAGOp::BinaryOp(BinaryOp::ComparisonOp(op)) => Some(op),
            _ => None,
        };

        if let Some(comparison_op) = comparison_op {
            comparison_op
        } else {
            let comparison_op = ComparisonOp::Equal;
            let falsy_node = self.add_node(
                DAGNode::new(DAGOp::UnaryOp(UnaryOp::Truthy), Some(vec![self.entry_node_id]))
            );
            let false_node = self.add_node(DAGNode::new(DAGOp::Const(Value::Bool(false)), None));
            let equal_node = self.add_node(
                DAGNode::new(
                    DAGOp::BinaryOp(BinaryOp::ComparisonOp(comparison_op)),
                    Some(vec![false_node, falsy_node])
                )
            );
            self.set_entry_node_id(equal_node);

            comparison_op
        }
    }

    pub fn get_entry_node_id(&self) -> usize {
        self.entry_node_id
    }

    pub fn get_entry_node_binary_op(&self) -> Option<BinaryOp> {
        match self.nodes.get(&self.entry_node_id).unwrap().op {
            DAGOp::BinaryOp(op) => Some(op),
            _ => None,
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

    pub fn generate_dag_bytecode(&self, vm_symbol_table: &mut VMSymbolTable) -> Vec<Instruction> {
        let mut bytecode = vec![];

        self.generate_node_bytecode(self.entry_node_id, vm_symbol_table, &mut bytecode);

        bytecode
    }

    fn generate_node_bytecode(
        &self,
        node_id: usize,
        vm_symbol_table: &mut VMSymbolTable,
        bytecode: &mut Vec<Instruction>
    ) -> InstructionSrc {
        let node = self.nodes.get(&node_id).cloned().unwrap();

        match &node.op {
            DAGOp::Const(value) => { InstructionSrc::Constant(value.clone()) }
            DAGOp::Identifier(lexeme) => {
                let stack_pos = vm_symbol_table.get_variable_stack_pos(lexeme);

                InstructionSrc::Stack(stack_pos)
            }
            DAGOp::UnaryOp(unary_op) => {
                let operand = node.operands.unwrap()[0];
                let right = self.generate_node_bytecode(operand, vm_symbol_table, bytecode);

                let register = vm_symbol_table.assign_register();

                if let InstructionSrc::Register(reg) = &right {
                    vm_symbol_table.free_register(*reg);
                }

                let instruction = match unary_op {
                    UnaryOp::Neg =>
                        Instruction::Neg {
                            reg: register,
                            src: right,
                        },
                    UnaryOp::Truthy =>
                        Instruction::Truthy {
                            reg: register,
                            src: right,
                        },
                };

                bytecode.push(instruction);
                InstructionSrc::Register(register)
            }
            DAGOp::BinaryOp(binary_op) => {
                let operands = node.operands.unwrap();
                let right = self.generate_node_bytecode(operands[0], vm_symbol_table, bytecode);
                let left = self.generate_node_bytecode(operands[1], vm_symbol_table, bytecode);

                let register = vm_symbol_table.assign_register();

                if let InstructionSrc::Register(reg) = &left {
                    vm_symbol_table.free_register(*reg);
                }
                if let InstructionSrc::Register(reg) = &right {
                    vm_symbol_table.free_register(*reg);
                }

                let instruction = match binary_op {
                    BinaryOp::Add =>
                        Instruction::Add {
                            reg: register,
                            src1: left,
                            src2: right,
                        },
                    BinaryOp::Sub =>
                        Instruction::Sub {
                            reg: register,
                            src1: left,
                            src2: right,
                        },
                    BinaryOp::Mul =>
                        Instruction::Mul {
                            reg: register,
                            src1: left,
                            src2: right,
                        },
                    BinaryOp::Div =>
                        Instruction::Div {
                            reg: register,
                            src1: left,
                            src2: right,
                        },

                    | BinaryOp::ComparisonOp(ComparisonOp::Equal)
                    | BinaryOp::ComparisonOp(ComparisonOp::NotEqual)
                    | BinaryOp::ComparisonOp(ComparisonOp::Greater)
                    | BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual)
                    | BinaryOp::ComparisonOp(ComparisonOp::Less)
                    | BinaryOp::ComparisonOp(ComparisonOp::LessEqual) =>
                        Instruction::Cmp { src1: left, src2: right },
                };

                bytecode.push(instruction);
                InstructionSrc::Register(register)
            }
            DAGOp::Define => {
                let operands = node.operands.unwrap();

                let lexeme = match &self.nodes.get(&operands[0]).unwrap().op {
                    DAGOp::Identifier(lexeme) => lexeme.clone(),
                    _ => panic!("Expected identifier"),
                };

                let value = match operands.get(1) {
                    Some(value_node_id) =>
                        self.generate_node_bytecode(*value_node_id, vm_symbol_table, bytecode),
                    None => InstructionSrc::Constant(Value::Empty),
                };

                let stack_pos = vm_symbol_table.insert_variable(&lexeme);

                let instruction = Instruction::Define {
                    stack_pos,
                    src: value,
                };

                bytecode.push(instruction);

                InstructionSrc::Stack(stack_pos)
            }
            DAGOp::Assign => {
                let operands = node.operands.unwrap();

                let lexeme = match &self.nodes.get(&operands[0]).unwrap().op {
                    DAGOp::Identifier(lexeme) => lexeme.clone(),
                    _ => panic!("Expected identifier"),
                };

                let value = match operands.get(1) {
                    Some(value_node_id) =>
                        self.generate_node_bytecode(*value_node_id, vm_symbol_table, bytecode),
                    None => InstructionSrc::Constant(Value::Empty),
                };

                let stack_pos = vm_symbol_table.get_variable_stack_pos(&lexeme);

                let instruction = Instruction::Assign {
                    stack_pos,
                    src: value,
                };

                bytecode.push(instruction);
                InstructionSrc::Stack(stack_pos)
            }
        }
    }

    // pub fn constant_folding(&mut self, environment: &mut IREnvironment, scope: usize) {
    //     self.eval(self.nodes.len() - 1, environment, scope);
    // }

    // #[profiler::function_tracker]
    // fn eval(
    //     &mut self,
    //     node_id: usize,
    //     environment: &mut IREnvironment,
    //     scope: usize
    // ) -> Option<Value> {
    //     let node = self.nodes.get(&node_id).cloned().unwrap();

    //     match &node.op {
    //         DAGOp::Const(value) => { Some(value.clone()) }
    //         DAGOp::Identifier(lexeme) => {
    //             let (value, changed_state, _) = match environment.get(lexeme) {
    //                 Some(value) => value,
    //                 None => panic!("Variable not found: {}", lexeme),
    //             };

    //             println!("{:?} {:?}", value, changed_state);
    //             // if changed_state == &ChangedState::Unchanged {
    //             return value.clone();
    //             // } else {
    //             //     return None;
    //             // }
    //         }
    //         DAGOp::BinaryOp(op) => {
    //             if let Some(operands) = &node.operands {
    //                 self.evaluate_binary_op(op, operands, node_id, environment, scope)
    //             } else {
    //                 None
    //             }
    //         }
    //         DAGOp::UnaryOp(op) => {
    //             if let Some(operands) = &node.operands {
    //                 self.evaluate_unary_op(op, operands[0], node_id, environment, scope)
    //             } else {
    //                 None
    //             }
    //         }
    //         DAGOp::Define | DAGOp::Assign => {
    //             if let Some(operands) = node.operands {
    //                 let is_definition = match &node.op {
    //                     DAGOp::Define => DefinitionState::IsDefinition,
    //                     _ => DefinitionState::IsAssignment,
    //                 };

    //                 let evaluated_value = match operands.get(1) {
    //                     Some(value_node_id) => { self.eval(*value_node_id, environment, scope) }
    //                     None => None,
    //                 };

    //                 let lexeme = match operands.get(0) {
    //                     Some(lexeme_node_id) => {
    //                         match &self.nodes.get(lexeme_node_id).unwrap().op {
    //                             DAGOp::Identifier(lexeme) => lexeme,
    //                             _ => panic!("Expected identifier"),
    //                         }
    //                     }
    //                     None => panic!("No lexeme node found"),
    //                 };

    //                 if is_definition == DefinitionState::IsDefinition {
    //                     environment.push(lexeme, evaluated_value.clone(), is_definition, scope);
    //                 } else {
    //                     environment.overwrite(&lexeme, evaluated_value.clone());
    //                 }

    //                 evaluated_value
    //             } else {
    //                 None
    //             }
    //         }
    //         _ => { panic!("Unsupported operation") }
    //     }
    // }

    // fn evaluate_unary_op(
    //     &mut self,
    //     op: &UnaryOp,
    //     operand: usize,
    //     node_id: usize,
    //     environment: &mut IREnvironment,
    //     scope: usize
    // ) -> Option<Value> {
    //     let right = self.eval(operand, environment, scope);

    //     match right {
    //         Some(rhs) => {
    //             match op {
    //                 UnaryOp::Neg => {
    //                     let evaluated = rhs.neg().unwrap();
    //                     self.remove_node(operand);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 UnaryOp::Truthy => {
    //                     let evaluated = rhs.not();
    //                     self.remove_node(operand);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //             }
    //         }
    //         _ => None,
    //     }
    // }

    // fn evaluate_binary_op(
    //     &mut self,
    //     op: &BinaryOp,
    //     operands: &Vec<usize>,
    //     node_id: usize,
    //     environment: &mut IREnvironment,
    //     scope: usize
    // ) -> Option<Value> {
    //     let left = self.eval(operands[0], environment, scope);
    //     let right = self.eval(operands[1], environment, scope);

    //     match (left, right) {
    //         (Some(lhs), Some(rhs)) => {
    //             match op {
    //                 BinaryOp::Add => {
    //                     let evaluated = lhs.add(&rhs).unwrap();

    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);

    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );

    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::Sub => {
    //                     let evaluated = lhs.sub(&rhs).unwrap();
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::Mul => {
    //                     let evaluated = lhs.mul(&rhs).unwrap();
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::Div => {
    //                     let evaluated = lhs.div(&rhs).unwrap();
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::ComparisonOp(ComparisonOp::Equal) => {
    //                     let evaluated = Value::Bool(lhs.cmp_e(&rhs).unwrap());
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::ComparisonOp(ComparisonOp::NotEqual) => {
    //                     let evaluated = Value::Bool(lhs.cmp_ne(&rhs).unwrap());
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::ComparisonOp(ComparisonOp::Greater) => {
    //                     let evaluated = Value::Bool(lhs.cmp_g(&rhs).unwrap());
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::ComparisonOp(ComparisonOp::GreaterEqual) => {
    //                     let evaluated = Value::Bool(lhs.cmp_ge(&rhs).unwrap());
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::ComparisonOp(ComparisonOp::Less) => {
    //                     let evaluated = Value::Bool(lhs.cmp_l(&rhs).unwrap());
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //                 BinaryOp::ComparisonOp(ComparisonOp::LessEqual) => {
    //                     let evaluated = Value::Bool(lhs.cmp_le(&rhs).unwrap());
    //                     self.remove_node(operands[0]);
    //                     self.remove_node(operands[1]);
    //                     self.add_node_at(
    //                         DAGNode::new(DAGOp::Const(evaluated.clone()), None),
    //                         node_id
    //                     );
    //                     Some(evaluated)
    //                 }
    //             }
    //         }
    //         _ => None,
    //     }
    // }
}
