use llvm_builder::{ Function, LLVMBuilder, LLVMType, Module, Operand };

use crate::compiler::{
    ds::value::ops::UnaryOp,
    ir::icfg::dag::DAG,
    traits::{ AllocLLVM, DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGUnaryNode {
    op: UnaryOp,
}

impl DAGNodeGenerateLLVM for DAGUnaryNode {
    fn alloc_llvm<T>(
        &self,
        _llvm_builder: &mut LLVMBuilder,
        _module: &mut Module,
        _func: &mut Function
    )
        where T: LLVMType {}

    fn generate_llvm<T>(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand
        where T: LLVMType
    {
        unimplemented!()
    }
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
