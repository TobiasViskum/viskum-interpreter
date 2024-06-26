use std::{ fmt::Debug, rc::Rc };

use crate::compiler::ds::value::{ ops::{ BinaryOp, UnaryOp }, Value };

#[derive(Debug)]
pub struct DAGBinaryNode {
    binary_op: BinaryOp,
    operands: (usize, usize),
}

impl DAGBinaryNode {
    pub fn new(binary_op: BinaryOp, operands: (usize, usize)) -> Self {
        Self { binary_op, operands }
    }

    pub fn get_binary_op(&self) -> BinaryOp {
        self.binary_op
    }

    pub fn get_operands(&self) -> (usize, usize) {
        self.operands
    }
}

#[derive(Debug)]
pub struct DAGUnaryNode {
    unary_op: UnaryOp,
    operand: usize,
}

impl DAGUnaryNode {
    pub fn new(unary_op: UnaryOp, operand: usize) -> Self {
        Self { unary_op, operand }
    }

    pub fn get_unary_op(&self) -> UnaryOp {
        self.unary_op
    }

    pub fn get_operand(&self) -> usize {
        self.operand
    }
}

#[derive(Debug)]
pub struct DAGFnCallNode {
    ident: Rc<str>,
    args: Vec<DAGNode>,
}

impl DAGFnCallNode {
    pub fn new(ident: Rc<str>, args: Vec<DAGNode>) -> Self {
        Self { ident, args }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }

    pub fn get_args(&self) -> &Vec<DAGNode> {
        &self.args
    }
}

#[derive(Debug)]
pub struct DAGDefineNode {
    ident: Rc<str>,
    value_node: usize,
}

impl DAGDefineNode {
    pub fn new(ident: Rc<str>, value_node: usize) -> Self {
        Self { ident, value_node }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }

    pub fn get_value_node(&self) -> usize {
        self.value_node
    }
}

#[derive(Debug)]
pub struct DAGAssignNode {
    ident: Rc<str>,
    value_node: usize,
}

impl DAGAssignNode {
    pub fn new(ident: Rc<str>, value_node: usize) -> Self {
        Self { ident, value_node }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }

    pub fn get_value_node(&self) -> usize {
        self.value_node
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

#[derive(Debug)]
pub struct DAGIdentNode {
    ident: Rc<str>,
}

impl DAGIdentNode {
    pub fn new(ident: Rc<str>) -> Self {
        Self { ident }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }
}

#[derive(Debug)]
pub enum DAGNode {
    BinaryNode(DAGBinaryNode),
    UnaryNode(DAGUnaryNode),
    FnCallNode(DAGFnCallNode),
    DefineNode(DAGDefineNode),
    AssignNode(DAGAssignNode),
    ConstNode(DAGConstNode),
    IdentNode(DAGIdentNode),
}

pub struct DAG {
    nodes: Vec<DAGNode>,
    entry_node_id: usize,
}

impl DAG {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            entry_node_id: 0,
        }
    }

    pub fn push_node(&mut self, dag_node: DAGNode) -> usize {
        self.nodes.push(dag_node);
        self.nodes.len() - 1
    }

    pub fn set_entry_node(&mut self, entry_node_id: usize) {
        self.entry_node_id = entry_node_id;
    }
}

impl Debug for DAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
