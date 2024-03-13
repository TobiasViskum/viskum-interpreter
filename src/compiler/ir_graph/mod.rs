use std::collections::{ HashMap, HashSet };

use indexmap::IndexSet;

use crate::{
    constants::REGISTERS,
    error_handler::ErrorHandler,
    value::Value,
    vm::instructions::InstructionSrc,
};

#[derive(Debug, Clone)]
pub enum IROperation {
    // Binary
    Add,
    Sub,
    Mul,
    Div,

    // Unary
    Neg,
    Truthy,

    // No-op
    NoOp,
}

#[derive(Debug, Clone)]
pub enum IRValue {
    Register(usize),
    Constant(Value),
}

impl IRValue {
    pub fn to_instruction_src(&self) -> InstructionSrc {
        match self {
            IRValue::Register(register) => InstructionSrc::Register(*register),
            IRValue::Constant(value) => InstructionSrc::Constant(*value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRNode {
    pub operation: IROperation,
    pub result: IRValue,
}

impl IRNode {
    pub fn new(operation: IROperation, result: IRValue) -> Self {
        Self { operation, result }
    }

    pub fn get_result(&self) -> &IRValue {
        &self.result
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct IREdge {
    pub src: usize,
    pub dest: usize,
}

impl IREdge {
    pub fn new(src: usize, dest: usize) -> Self {
        Self { src, dest }
    }
}

#[derive(Debug)]
pub struct IRGraph<'a> {
    nodes: HashMap<usize, IRNode>,
    edges: IndexSet<IREdge>,
    available_registers: Vec<usize>,
    error_handler: &'a ErrorHandler,
}

impl<'a> IRGraph<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        let mut available_registers = Vec::new();
        for i in (1..REGISTERS).rev() {
            available_registers.push(i - 1);
        }

        Self {
            nodes: HashMap::new(),
            edges: IndexSet::new(),
            available_registers,
            error_handler,
        }
    }

    pub fn get_nodes(&self) -> &HashMap<usize, IRNode> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &IndexSet<IREdge> {
        &self.edges
    }

    pub fn get_node(&self, id: usize) -> Option<&IRNode> {
        self.nodes.get(&id)
    }

    pub fn add_node(&mut self, id: usize, node: IRNode) {
        self.nodes.insert(id, node);
    }

    pub fn remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
        self.edges.retain(|edge| edge.src != id && edge.dest != id);
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        let edge = IREdge::new(src, dest);
        self.edges.insert(edge);
    }

    pub fn remove_edge(&mut self, src: usize, dest: usize) {
        let edge = IREdge::new(src, dest);
        self.edges.swap_remove(&edge);
    }

    pub fn update_node(&mut self, id: usize, node: IRNode) {
        if self.nodes.contains_key(&id) {
            self.nodes.insert(id, node);
        } else {
            println!("Node with id {} does not exist.", id);
        }
    }

    pub fn update_edge(&mut self, src: usize, dest: usize, new_dest: usize) {
        let old_edge = IREdge::new(src, dest);
        let new_edge = IREdge::new(src, new_dest);
        if self.edges.contains(&old_edge) {
            self.edges.swap_remove(&old_edge);
            self.edges.insert(new_edge);
        } else {
            println!("Edge from {} to {} does not exist.", src, dest);
        }
    }

    pub fn get_edges_to_node(&self, node_id: usize) -> Vec<IREdge> {
        self.edges
            .iter()
            .filter(|&edge| edge.dest == node_id)
            .cloned()
            .collect::<Vec<IREdge>>()
    }

    pub fn get_edge_to_node(&self, node_id: usize) -> &IREdge {
        self.edges
            .iter()
            .find(|&edge| edge.dest == node_id)
            .unwrap()
    }

    // pub(super) fn get_evaluated_node_if_constant(
    //     &self,
    //     node_id: usize
    // ) -> Result<Option<Value>, ()> {
    //     if let Some(node) = self.nodes.get(&node_id) {
    //         // println!("Evaluating node: {:?}, {:?}", node.result, node.operation);
    //         match &node.result {
    //             IRValue::Constant(value) => Ok(Some(*value)),
    //             IRValue::Register(register) => {
    //                 let mut result: Option<Value> = None;
    //                 let mut edges_to_this_node = self.edges
    //                     .iter()
    //                     .filter(|&edge| edge.dest == node_id);
    //                 for edge in &mut edges_to_this_node {
    //                     if let Ok(Some(value)) = self.get_evaluated_node_if_constant(edge.src) {
    //                         match node.operation {
    //                             IROperation::Add => {
    //                                 result = result.map(|r: Value| r.add(&value)).or(Some(value));
    //                             }
    //                             IROperation::Mul => {
    //                                 result = result.map(|r: Value| r.mul(&value)).or(Some(value));
    //                             }
    //                             IROperation::Div => {
    //                                 result = result.map(|r: Value| r.div(&value)).or(Some(value));
    //                             }
    //                             IROperation::Sub => {
    //                                 result = result.map(|r: Value| r.sub(&value)).or(Some(value));
    //                             }
    //                             IROperation::Neg => {
    //                                 // value = -value;
    //                             }
    //                             _ => {
    //                                 println!("Cannot evaluate: Unsupported operation.");
    //                                 return Err(());
    //                             }
    //                         }
    //                     } else {
    //                         return Ok(None);
    //                     }
    //                 }

    //                 Ok(result)
    //             }
    //         }
    //     } else {
    //         println!("Node with id {} not found.", node_id);
    //         Ok(None)
    //     }
    // }
}
