use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::SSAKey, value::{ ops::{ BinaryOp, UnaryOp }, Value } },
    traits::{ DAGNodeTrait, Dissasemble },
};

#[derive(Debug)]
pub struct DAGDropNode {
    drops_count: usize,
}

impl DAGDropNode {
    pub fn new(drops_count: usize) -> Self {
        Self { drops_count }
    }
}

impl DAGNodeTrait for DAGDropNode {
    type ConnectedNodes = Vec<usize>;

    fn parse_connected_nodes(&self, mut connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        if connected_nodes.len() > self.drops_count {
            connected_nodes.pop();
        }

        connected_nodes
    }

    fn expected_connected_nodes(&self) -> usize {
        self.drops_count
    }
}

impl Dissasemble for DAGDropNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}

#[derive(Debug)]
pub struct DAGGroupNode;

impl DAGNodeTrait for DAGGroupNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match connected_nodes.get(0) {
            Some(node_id) => *node_id,
            None => panic!("Expected one node connected to DAGGroupNode"),
        }
    }

    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

#[derive(Debug)]
pub struct DAGBinaryNode {
    op: BinaryOp,
}

impl DAGBinaryNode {
    pub fn new(op: BinaryOp) -> Self {
        Self { op }
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }
}

impl DAGNodeTrait for DAGBinaryNode {
    type ConnectedNodes = (usize, usize);

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match (connected_nodes.get(0), connected_nodes.get(1)) {
            (Some(node_id_1), Some(node_id_2)) => (*node_id_1, *node_id_2),
            _ => panic!("Expected two nodes connected to DAGBinaryNode"),
        }
    }

    fn expected_connected_nodes(&self) -> usize {
        2
    }
}

impl Dissasemble for DAGBinaryNode {
    fn dissasemble(&self) -> String {
        self.op.dissasemble()
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

impl DAGNodeTrait for DAGUnaryNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match connected_nodes.get(0) {
            Some(node_id) => *node_id,
            None => panic!("Expected one node connected to DAGUnaryNode"),
        }
    }

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

impl DAGNodeTrait for DAGFnCallNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        todo!()
    }

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
pub struct DAGDefineNode {
    ssa_key: SSAKey,
    is_mutable: bool,
    is_initialized: bool,
}

impl DAGNodeTrait for DAGDefineNode {
    type ConnectedNodes = Option<usize>;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match self.is_initialized {
            true =>
                connected_nodes
                    .get(0)
                    .copied()
                    .or_else(|| panic!("Expected one node connected to DAGDefineNode")),
            false => None,
        }
    }

    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

impl DAGDefineNode {
    pub fn new(ssa_key: SSAKey, is_mutable: bool, is_initialized: bool) -> Self {
        Self {
            ssa_key,
            is_mutable,
            is_initialized,
        }
    }

    pub fn get_ssa_key(&self) -> &SSAKey {
        &self.ssa_key
    }

    pub fn get_is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Dissasemble for DAGDefineNode {
    fn dissasemble(&self) -> String {
        ":=".to_string()
    }
}

#[derive(Debug)]
pub struct DAGAssignNode;

impl Dissasemble for DAGAssignNode {
    fn dissasemble(&self) -> String {
        "=".to_string()
    }
}

impl DAGNodeTrait for DAGAssignNode {
    type ConnectedNodes = (usize, usize);

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match (connected_nodes.get(0), connected_nodes.get(1)) {
            (Some(node_id_1), Some(node_id_2)) => (*node_id_1, *node_id_2),
            _ => panic!("Expected two nodes connected to DAGAssignNode"),
        }
    }

    fn expected_connected_nodes(&self) -> usize {
        2
    }
}

#[derive(Debug)]
pub struct DAGConstNode {
    value: Value,
}

impl DAGConstNode {
    pub fn new(value: Value) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }
}

impl DAGNodeTrait for DAGConstNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, _: Vec<usize>) -> Self::ConnectedNodes {
        ()
    }

    fn expected_connected_nodes(&self) -> usize {
        0
    }
}

impl Dissasemble for DAGConstNode {
    fn dissasemble(&self) -> String {
        self.value.dissasemble()
    }
}

#[derive(Debug)]
pub struct DAGIdentNode {
    ssa_key: SSAKey,
}

impl DAGIdentNode {
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

impl DAGNodeTrait for DAGIdentNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, _: Vec<usize>) -> Self::ConnectedNodes {
        ()
    }

    fn expected_connected_nodes(&self) -> usize {
        0
    }
}

impl Dissasemble for DAGIdentNode {
    fn dissasemble(&self) -> String {
        self.ssa_key.dissasemble()
    }
}
