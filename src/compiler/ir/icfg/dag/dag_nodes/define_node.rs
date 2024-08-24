use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::{
    ds::ssa_ident::SSAIdent,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGDefineNode {
    ssa_key: SSAIdent,
    is_mutable: bool,
    is_initialized: bool,
    result_type: ValueType,
}

impl DAGNodeGenerateLLVM for DAGDefineNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, module: &mut Module, func: &mut Function) {
        let var_key = llvm_builder.req_var_ssa_key(self.ssa_key.get_ident());
        func.add_instr(
            format!("%{} = alloca {}, align 4", var_key, self.result_type.to_llvm_type().build())
        );
    }

    fn generate_llvm(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let connected_node = self
            .parse_connected_nodes(dag.get_connected_node_ids(node_id))
            .expect("Unintialized variables not supported");

        let result = dag.generate_llvm(connected_node, func, llvm_builder);

        let var_key = llvm_builder.get_var_ssa_key(self.ssa_key.get_ident());

        func.add_instr(
            format!(
                "store {} {}, ptr %{}, align 4",
                self.result_type.to_llvm_type().build(),
                result.build(),
                var_key
            )
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
    pub fn new(
        ssa_key: SSAIdent,
        is_mutable: bool,
        is_initialized: bool,
        result_type: ValueType
    ) -> Self {
        Self {
            ssa_key,
            is_mutable,
            is_initialized,
            result_type,
        }
    }

    pub fn get_ssa_key(&self) -> &SSAIdent {
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
