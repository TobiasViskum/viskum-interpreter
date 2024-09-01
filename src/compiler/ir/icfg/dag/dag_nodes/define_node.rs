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
    result_type: ValueType,
}

impl DAGNodeGenerateLLVM for DAGDefineNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        let var_key = llvm_builder.req_var_ssa_key(self.ssa_key.get_ident());
        func.add_instr(
            format!(
                "%v{} = alloca {}, align {}",
                var_key,
                self.result_type.to_llvm_type().build(),
                self.result_type.to_llvm_type().get_byte_size()
            )
        );
    }

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let connected_node = self.parse_connected_nodes(dag.get_connected_node_ids(node_id));

        let var_key = llvm_builder.get_var_ssa_key(self.ssa_key.get_ident());

        let result = dag.generate_llvm(connected_node, Some(Var::new(var_key)), func, llvm_builder);

        func.add_instr(
            format!(
                "store {} {}, ptr %v{}, align 4",
                self.result_type.to_llvm_type().build(),
                result.build(),
                var_key
            )
        );

        Operand::Var(Var::new(var_key))
    }
}

impl ParseConnectedNodes for DAGDefineNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        *connected_nodes.get(0).expect("Expected node connected to define node")
    }
}

impl DAGNodeTrait for DAGDefineNode {
    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

impl DAGDefineNode {
    pub fn new(ssa_key: SSAIdent, result_type: ValueType) -> Self {
        Self {
            ssa_key,
            result_type,
        }
    }

    pub fn get_ssa_key(&self) -> &SSAIdent {
        &self.ssa_key
    }
}

impl Dissasemble for DAGDefineNode {
    fn dissasemble(&self) -> String {
        ":=".to_string()
    }
}
