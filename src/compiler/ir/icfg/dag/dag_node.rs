use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::SSAKey, value::{ ops::{ BinaryOp, UnaryOp }, Value } },
    traits::{ DAGNodeTrait, Dissasemble },
};

#[derive(Debug)]
pub struct DAGGroupNode;

impl DAGNodeTrait for DAGGroupNode {
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
pub struct DAGDefineNode;

impl Dissasemble for DAGDefineNode {
    fn dissasemble(&self) -> String {
        ":=".to_string()
    }
}

impl DAGNodeTrait for DAGDefineNode {
    fn expected_connected_nodes(&self) -> usize {
        2
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
    fn expected_connected_nodes(&self) -> usize {
        0
    }
}

impl Dissasemble for DAGIdentNode {
    fn dissasemble(&self) -> String {
        self.ssa_key.dissasemble()
    }
}
