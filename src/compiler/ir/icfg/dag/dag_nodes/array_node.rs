use crate::compiler::{
    ds::value::ValueType,
    ir::icfg::dag::DAG,
    llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var },
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGArrayNode {
    items_type: ValueType,
    items_count: usize,
}

impl DAGArrayNode {
    pub fn new(items_type: ValueType, items_count: usize) -> Self {
        Self { items_type, items_count }
    }
}
impl DAGNodeGenerateLLVM for DAGArrayNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let items = self.parse_connected_nodes(dag.get_connected_node_ids(node_id));

        if let Some(ssa_var) = ssa_var {
            let array_items = items
                .iter()
                .map(|idx| dag.generate_llvm(*idx, Some(ssa_var), func, llvm_builder))
                .collect::<Vec<_>>();

            let ssa_key = llvm_builder.req_ssa_key();
            func.add_instr(
                format!(
                    "%v{} = getelementptr inbounds [{} x {}], ptr %v{}, i64 0, i64 0",
                    ssa_key,
                    self.items_count,
                    self.items_type.to_llvm_type().build(),
                    ssa_var.get_ssa_subscript()
                )
            );

            let prev_key = ssa_key;
            for array_item in array_items {
                func.add_instr(
                    format!(
                        "store {} {}, ptr %v{}, align 4",
                        self.items_type.to_llvm_type().build(),
                        array_item.build(),
                        prev_key
                    )
                );
            }

            Operand::Var(Var::new(0))
        } else {
            Operand::Var(Var::new(0))
        }
    }
}

impl Dissasemble for DAGArrayNode {
    fn dissasemble(&self) -> String {
        "[]".to_string()
    }
}

impl ParseConnectedNodes for DAGArrayNode {
    type ConnectedNodes = Vec<usize>;

    fn parse_connected_nodes(&self, mut connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        if self.items_count < connected_nodes.len() {
            connected_nodes.pop();
        }
        connected_nodes
    }
}

impl DAGNodeTrait for DAGArrayNode {
    fn expected_connected_nodes(&self) -> usize {
        self.items_count
    }
}
