use crate::compiler::{
    ds::value::Value,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

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

impl DAGNodeGenerateLLVM for DAGConstNode {
    fn generate_llvm<T>(
        &self,
        node_id: usize,
        func: &mut llvm_builder::Function,
        llvm_builder: &mut llvm_builder::LLVMBuilder,
        dag: &DAG
    ) -> llvm_builder::Operand
        where T: llvm_builder::LLVMType
    {
        self.value.get_llvm_operand()
    }
}

impl ParseConnectedNodes for DAGConstNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, _: Vec<usize>) -> Self::ConnectedNodes {
        ()
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
