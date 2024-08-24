use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::{
    ir::icfg::dag::{ DAGNode, DAG },
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGAssignNode {
    result_type: ValueType,
}

impl DAGAssignNode {
    pub fn new(result_type: ValueType) -> Self {
        Self {
            result_type,
        }
    }
}

impl DAGNodeGenerateLLVM for DAGAssignNode {
    fn alloc_llvm(
        &self,
        _llvm_builder: &mut LLVMBuilder,
        _module: &mut Module,
        _func: &mut Function
    ) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let connected_node_ids = self.parse_connected_nodes(dag.get_connected_node_ids(node_id));

        let ident_node = match &dag.nodes[connected_node_ids.0] {
            DAGNode::IdentNode(ident_node) => ident_node,
            _ => panic!("Right now only identifiers is supported in assignment"),
        };

        let result = dag.generate_llvm(connected_node_ids.1, func, llvm_builder);
        let var_ssa_key = llvm_builder.get_var_ssa_key(ident_node.get_ssa_key().get_ident());

        func.add_instr(
            format!(
                "store {} {}, ptr %{}, align 4",
                self.result_type.to_llvm_type().build(),
                result.build(),
                var_ssa_key
            )
        );

        Operand::Var(Var::new(var_ssa_key))
    }
}

impl Dissasemble for DAGAssignNode {
    fn dissasemble(&self) -> String {
        "=".to_string()
    }
}

impl ParseConnectedNodes for DAGAssignNode {
    type ConnectedNodes = (usize, usize);

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match (connected_nodes.get(0), connected_nodes.get(1)) {
            (Some(node_id_1), Some(node_id_2)) => (*node_id_1, *node_id_2),
            _ => panic!("Expected two nodes connected to DAGAssignNode"),
        }
    }
}

impl DAGNodeTrait for DAGAssignNode {
    fn expected_connected_nodes(&self) -> usize {
        2
    }
}
