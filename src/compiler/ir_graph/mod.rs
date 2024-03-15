use std::collections::HashMap;

use indexmap::IndexSet;

use crate::{ operations::Op, value::Value, vm::instructions::IRInstructionSrc };

#[derive(Debug, Clone)]
pub enum IRValue {
    Register(usize),
    Constant(Value),
    VariableRegister(usize),
}

impl IRValue {
    pub fn to_ir_instruction_src(&self) -> IRInstructionSrc {
        match self {
            IRValue::Register(register) => IRInstructionSrc::Register(*register),
            IRValue::Constant(value) => IRInstructionSrc::Constant(*value),
            IRValue::VariableRegister(register) => IRInstructionSrc::VariableRegister(*register),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRNode {
    pub operation: Op,
    pub result: IRValue,
}

impl IRNode {
    pub fn new(operation: Op, result: IRValue) -> Self {
        Self { operation, result }
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

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct IRControlFlowEdge {
    pub src: usize,
    pub dest: usize,
}
impl IRControlFlowEdge {
    pub fn new(src: usize, dest: usize) -> Self {
        Self { src, dest }
    }
}

#[derive(Debug)]
pub struct IRGraph {
    nodes: HashMap<usize, IRNode>,
    edges: IndexSet<IREdge>,
    control_flow_edges: IndexSet<IRControlFlowEdge>,
}

impl IRGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: IndexSet::new(),
            control_flow_edges: IndexSet::new(),
        }
    }

    pub fn _get_nodes(&self) -> &HashMap<usize, IRNode> {
        &self.nodes
    }

    pub fn _get_edges(&self) -> &IndexSet<IREdge> {
        &self.edges
    }

    pub fn get_node(&self, id: usize) -> Option<&IRNode> {
        self.nodes.get(&id)
    }

    pub fn add_node(&mut self, id: usize, node: IRNode) {
        self.nodes.insert(id, node);
    }

    pub fn _remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
        self.edges.retain(|edge| edge.src != id && edge.dest != id);
    }

    pub fn add_edge(&mut self, src: usize, dest: usize) {
        let edge = IREdge::new(src, dest);
        self.edges.insert(edge);
    }

    pub fn add_control_flow_edge(&mut self, src: usize, dest: usize) {
        let edge = IRControlFlowEdge::new(src, dest);
        self.control_flow_edges.insert(edge);
    }

    pub fn _remove_control_flow_edge(&mut self, src: usize, dest: usize) {
        let edge = IRControlFlowEdge::new(src, dest);
        self.control_flow_edges.swap_remove(&edge);
    }

    pub fn _remove_edge(&mut self, src: usize, dest: usize) {
        let edge = IREdge::new(src, dest);
        self.edges.swap_remove(&edge);
    }

    pub fn _update_node(&mut self, id: usize, node: IRNode) {
        if self.nodes.contains_key(&id) {
            self.nodes.insert(id, node);
        } else {
            println!("Node with id {} does not exist.", id);
        }
    }

    pub fn _update_edge(&mut self, src: usize, dest: usize, new_dest: usize) {
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

    pub fn get_control_flow_edge_to_node(&self, node_id: usize) -> Option<&IRControlFlowEdge> {
        self.control_flow_edges.iter().find(|&edge| edge.dest == node_id)
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
