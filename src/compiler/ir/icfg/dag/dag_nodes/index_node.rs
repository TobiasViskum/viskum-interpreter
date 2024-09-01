use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    ir::icfg::dag::DAG,
    llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Operand, Type, Var },
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
};

#[derive(Debug)]
pub struct DAGIndexNode {
    ssa_ident: SSAIdent,
    items_type: ValueType,
    items_count: usize,
}

impl DAGIndexNode {
    pub fn new(ssa_ident: SSAIdent, items_type: ValueType, items_count: usize) -> Self {
        Self { ssa_ident, items_type, items_count }
    }

    pub fn get_items_type(&self) -> &ValueType {
        &self.items_type
    }

    pub fn get_items_count(&self) -> usize {
        self.items_count
    }
}

impl DAGNodeGenerateLLVM for DAGIndexNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let idx_op = dag.generate_llvm(
            self.parse_connected_nodes(dag.get_connected_node_ids(node_id)),
            ssa_var,
            func,
            llvm_builder
        );

        let ssa_key = llvm_builder.req_ssa_key();
        func.add_instr(
            format!(
                "%v{} = getelementptr inbounds {}, ptr %v{}, i64 0, i64 {}",
                ssa_key,
                Type::Array((Box::new(self.items_type.to_llvm_type()), self.items_count)).build(),
                llvm_builder.get_var_ssa_key(self.ssa_ident.get_ident()),
                idx_op.build()
            )
        );

        Operand::Var(Var::new(ssa_key))
    }
}

impl ParseConnectedNodes for DAGIndexNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        *connected_nodes.get(0).expect("Expected 1 connected node to DAGIndexNode")
    }
}

impl DAGNodeTrait for DAGIndexNode {
    fn expected_connected_nodes(&self) -> usize {
        1
    }
}
