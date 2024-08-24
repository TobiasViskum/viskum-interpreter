use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module, Operand };

use crate::compiler::{
    ir::icfg::dag::DAG,
    traits::{ AllocLLVM, DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
};

#[derive(Debug)]
pub struct DAGGroupNode {
    result_type: ValueType,
}

impl DAGGroupNode {
    pub fn new(result_type: ValueType) -> Self {
        Self { result_type }
    }
}

impl DAGNodeGenerateLLVM for DAGGroupNode {
    fn alloc_llvm(
        &self,
        _llvm_builder: &mut LLVMBuilder,
        _module: &mut Module,
        _func: &mut Function
    ) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let connected_node_id = self.parse_connected_nodes(dag.get_connected_node_ids(node_id));
        dag.generate_llvm(connected_node_id, func, llvm_builder)
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
