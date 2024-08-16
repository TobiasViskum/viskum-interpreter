use std::rc::Rc;

use llvm_builder::{ Operand, Var };

use crate::compiler::{
    ds::symbol_table::SSAKey,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

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

impl DAGNodeGenerateLLVM for DAGIdentNode {
    fn generate_llvm<T>(
        &self,
        node_id: usize,
        func: &mut llvm_builder::Function,
        llvm_builder: &mut llvm_builder::LLVMBuilder,
        dag: &crate::compiler::ir::icfg::dag::DAG
    ) -> llvm_builder::Operand
        where T: llvm_builder::LLVMType
    {
        let var_key = llvm_builder.get_var_ssa_key(self.get_ident());
        let result_key = llvm_builder.req_ssa_key();
        func.add_instr(format!("%{} = load {}, ptr %{}", result_key, T::build_type(), var_key));

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
