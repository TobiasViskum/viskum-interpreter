use std::rc::Rc;

use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::{
    ds::ssa_ident::SSAIdent,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGIdentNode {
    ssa_key: SSAIdent,
    result_type: ValueType,
}

impl DAGIdentNode {
    pub fn new(ssa_key: SSAIdent, result_type: ValueType) -> Self {
        Self { ssa_key, result_type }
    }

    pub fn get_ident(&self) -> Rc<str> {
        self.ssa_key.get_ident()
    }

    pub fn get_ssa_key(&self) -> &SSAIdent {
        &self.ssa_key
    }
}

impl DAGNodeGenerateLLVM for DAGIdentNode {
    fn alloc_llvm(&self, _llvm_builder: &mut LLVMBuilder, _func: &mut Function) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let var_key = llvm_builder.get_var_ssa_key(self.get_ident());
        let result_key = llvm_builder.req_ssa_key();
        func.add_instr(
            format!(
                "%v{} = load {}, ptr %v{}",
                result_key,
                self.result_type.to_llvm_type().build(),
                var_key
            )
        );

        Operand::Var(Var::new(result_key))
    }
}

impl ParseConnectedNodes for DAGIdentNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, _: Vec<usize>) -> Self::ConnectedNodes {
        ()
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
