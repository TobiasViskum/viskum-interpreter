use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::{
    ds::value::ops::UnaryOp,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGUnaryNode {
    op: UnaryOp,
    result_type: ValueType,
}

impl DAGNodeGenerateLLVM for DAGUnaryNode {
    fn alloc_llvm(&self, _llvm_builder: &mut LLVMBuilder, _func: &mut Function) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        unimplemented!()
    }
}

impl DAGUnaryNode {
    pub fn new(op: UnaryOp, result_type: ValueType) -> Self {
        Self { op, result_type }
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
