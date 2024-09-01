use crate::compiler::{
    ds::value::ValueType,
    ir::icfg::dag::DAG,
    llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Operand, Var },
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGDerefNode {
    result_type: ValueType,
}

impl DAGDerefNode {
    pub fn new(result_type: ValueType) -> Self {
        Self {
            result_type,
        }
    }
}

impl DAGNodeGenerateLLVM for DAGDerefNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let result_op = dag.generate_llvm(
            self.parse_connected_nodes(dag.get_connected_node_ids(node_id)),
            ssa_var,
            func,
            llvm_builder
        );

        let ssa_key = llvm_builder.req_ssa_key();
        func.add_instr(
            format!(
                "%v{} = load {}, ptr {}",
                ssa_key,
                self.result_type.to_llvm_type().build(),
                result_op.build()
            )
        );

        Operand::Var(Var::new(ssa_key))
    }
}

impl ParseConnectedNodes for DAGDerefNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        *connected_nodes.get(0).expect("Expected one node conneceted to DAGDerefNode")
    }
}

impl DAGNodeTrait for DAGDerefNode {
    fn expected_connected_nodes(&self) -> usize {
        1
    }
}

impl Dissasemble for DAGDerefNode {
    fn dissasemble(&self) -> String {
        "DAGDEREF".to_string()
    }
}
