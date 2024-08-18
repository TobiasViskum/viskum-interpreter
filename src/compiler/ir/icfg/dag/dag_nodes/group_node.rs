use llvm_builder::{ Function, LLVMBuilder, LLVMType, Module, Operand };

use crate::compiler::{
    ir::icfg::dag::DAG,
    traits::{ AllocLLVM, DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
};

#[derive(Debug)]
pub struct DAGGroupNode;

impl DAGNodeGenerateLLVM for DAGGroupNode {
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
