use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::SSAKey, value::{ ops::{ BinaryOp, UnaryOp }, Value } },
    traits::{ DAGNodeTrait, Dissasemble, ParseConnectedNodes },
};

#[derive(Debug)]
pub struct DAGGroupNode;

impl ParseConnectedNodes for DAGGroupNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match connected_nodes.get(0) {
            Some(node_id) => *node_id,
            None => panic!("Expected one node connected to DAGGroupNode"),
        }
    }
}

impl DAGNodeTrait for DAGGroupNode {
    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

#[derive(Debug)]
pub struct DAGUnaryNode {
    op: UnaryOp,
}

impl DAGUnaryNode {
    pub fn new(op: UnaryOp) -> Self {
        Self { op }
    }

    pub fn get_op(&self) -> UnaryOp {
        self.op
    }
}

impl ParseConnectedNodes for DAGUnaryNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match connected_nodes.get(0) {
            Some(node_id) => *node_id,
            None => panic!("Expected one node connected to DAGUnaryNode"),
        }
    }
}

impl DAGNodeTrait for DAGUnaryNode {
    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

impl Dissasemble for DAGUnaryNode {
    fn dissasemble(&self) -> String {
        self.op.dissasemble()
    }
}

#[derive(Debug)]
pub struct DAGFnCallNode {
    ssa_key: SSAKey,
}

impl DAGFnCallNode {
    pub fn new(ssa_key: SSAKey) -> Self {
        Self { ssa_key }
    }

    pub fn get_ident(&self) -> Rc<str> {
        self.ssa_key.get_ident()
    }

    pub fn get_ssa_key(&self) -> &SSAKey {
        &self.ssa_key
    }
}

impl ParseConnectedNodes for DAGFnCallNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        todo!()
    }
}

impl DAGNodeTrait for DAGFnCallNode {
    fn expected_connected_nodes(&self) -> usize {
        todo!()
    }
}

impl Dissasemble for DAGFnCallNode {
    fn dissasemble(&self) -> String {
        format!("{}(", self.ssa_key.dissasemble())
    }
}

#[derive(Debug)]
pub struct DAGAssignNode;

impl Dissasemble for DAGAssignNode {
    fn dissasemble(&self) -> String {
        "=".to_string()
    }
}

impl ParseConnectedNodes for DAGAssignNode {
    type ConnectedNodes = (usize, usize);

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match (connected_nodes.get(0), connected_nodes.get(1)) {
            (Some(node_id_1), Some(node_id_2)) => (*node_id_1, *node_id_2),
            _ => panic!("Expected two nodes connected to DAGAssignNode"),
        }
    }
}

impl DAGNodeTrait for DAGAssignNode {
    fn expected_connected_nodes(&self) -> usize {
        2
    }
}
