use crate::compiler::ds::ssa_ident::SSAIdent;
use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::{
    ir::icfg::dag::{ DAGNode, DAG },
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGAssignNode {
    ssa_ident: SSAIdent,
    result_type: ValueType,
    has_field_expr: bool,
}

impl DAGAssignNode {
    pub fn new(ssa_ident: SSAIdent, result_type: ValueType, has_field_expr: bool) -> Self {
        Self {
            ssa_ident,
            result_type,
            has_field_expr,
        }
    }
}

impl DAGNodeGenerateLLVM for DAGAssignNode {
    fn alloc_llvm(
        &self,
        _llvm_builder: &mut LLVMBuilder,

        _func: &mut Function
    ) {}

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let connected_node_ids = self.parse_connected_nodes(dag.get_connected_node_ids(node_id));

        let result = dag.generate_llvm(connected_node_ids.1, ssa_var, func, llvm_builder);
        let var_ssa_key = llvm_builder.get_var_ssa_key(self.ssa_ident.get_ident());

        if self.has_field_expr {
            let setter = dag.generate_llvm(connected_node_ids.0, ssa_var, func, llvm_builder);
            func.add_instr(
                format!(
                    "store {} {}, ptr {}, align 4",
                    self.result_type.to_llvm_type().build(),
                    result.build(),
                    setter.build()
                )
            );
        } else {
            func.add_instr(
                format!(
                    "store {} {}, ptr %v{}, align 4",
                    self.result_type.to_llvm_type().build(),
                    result.build(),
                    var_ssa_key
                )
            );
        }

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
