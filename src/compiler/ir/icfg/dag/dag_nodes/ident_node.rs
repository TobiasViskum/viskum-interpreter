use std::rc::Rc;

use llvm_builder::{ Function, LLVMBuilder, LLVMType, Module, Operand, Var };

use crate::compiler::{
    ds::symbol_table::SSAKey,
    ir::icfg::dag::DAG,
    traits::{ AllocLLVM, DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
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
