use std::rc::Rc;

use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module, Operand };

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
    fn alloc_llvm(
        &self,
        llvm_builder: &mut LLVMBuilder,
        module: &mut Module,
        _func: &mut Function
    ) {
        match self.get_value() {
            Value::String(string) => {
                let const_string_idx = module.add_string_const(string.to_string());
                llvm_builder.insert_const_string_idx(Rc::clone(string), const_string_idx);
            }
            _ => {}
        }
    }

    fn generate_llvm(
        &self,
        _node_id: usize,
        _func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        _dag: &DAG
    ) -> Operand {
        self.value.get_llvm_operand(llvm_builder)
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
