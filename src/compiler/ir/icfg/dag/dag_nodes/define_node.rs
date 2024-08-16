use std::fmt::format;

use llvm_builder::{ BuildLLVM, Operand, Var };

use crate::compiler::{
    ds::symbol_table::SSAKey,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGDefineNode {
    ssa_key: SSAKey,
    is_mutable: bool,
    is_initialized: bool,
}

impl DAGNodeGenerateLLVM for DAGDefineNode {
    fn generate_llvm<T>(
        &self,
        node_id: usize,
        func: &mut llvm_builder::Function,
        llvm_builder: &mut llvm_builder::LLVMBuilder,
        dag: &crate::compiler::ir::icfg::dag::DAG
    ) -> llvm_builder::Operand
        where T: llvm_builder::LLVMType
    {
        let connected_node = self
            .parse_connected_nodes(dag.get_connected_node_ids(node_id))
            .expect("Unintialized variables not supported");

        let result = dag.generate_llvm::<T>(connected_node, func, llvm_builder);

        let var_key = llvm_builder.req_var_ssa_key(self.ssa_key.get_ident());
        func.add_instr(format!("%{} = alloca {}, align 4", var_key, T::build_type()));
        func.add_instr(
            format!("store {} {}, ptr %{}, align 4", T::build_type(), result.build(), var_key)
        );

        Operand::Var(Var::new(var_key))
    }
}

impl ParseConnectedNodes for DAGDefineNode {
    type ConnectedNodes = Option<usize>;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match self.is_initialized {
            true =>
                connected_nodes
                    .get(0)
                    .copied()
                    .or_else(|| panic!("Expected one node connected to DAGDefineNode")),
            false => None,
        }
    }
}

impl DAGNodeTrait for DAGDefineNode {
    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

impl DAGDefineNode {
    pub fn new(ssa_key: SSAKey, is_mutable: bool, is_initialized: bool) -> Self {
        Self {
            ssa_key,
            is_mutable,
            is_initialized,
        }
    }

    pub fn get_ssa_key(&self) -> &SSAKey {
        &self.ssa_key
    }

    pub fn get_is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Dissasemble for DAGDefineNode {
    fn dissasemble(&self) -> String {
        ":=".to_string()
    }
}
