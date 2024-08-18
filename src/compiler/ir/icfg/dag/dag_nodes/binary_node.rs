use llvm_builder::{ BuildLLVM, Function, LLVMBuilder, LLVMType, Module, Operand, Var };

use crate::compiler::{
    ds::value::ops::{ BinaryOp, ComparisonOp },
    ir::icfg::dag::DAG,
    traits::{ AllocLLVM, DAGNodeGenerateLLVM, DAGNodeTrait, OpTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGBinaryNode {
    op: BinaryOp,
}

impl DAGBinaryNode {
    pub fn new(op: BinaryOp) -> Self {
        Self { op }
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }
}

impl DAGNodeGenerateLLVM for DAGBinaryNode {
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
        let (connected_node_id_1, connected_node_id_2) = self.parse_connected_nodes(
            dag.get_connected_node_ids(node_id)
        );

        let (op1, op2) = (
            dag.generate_llvm::<T>(connected_node_id_1, func, llvm_builder),
            dag.generate_llvm::<T>(connected_node_id_2, func, llvm_builder),
        );

        let result_key = llvm_builder.req_ssa_key();

        let instr = if self.op.is_cmp() {
            format!(
                "%{} = icmp {} {} {}, {}",
                result_key,
                self.op.build_llvm(),
                T::build_type(),
                op1.build(),
                op2.build()
            )
        } else {
            format!(
                "%{} = {} nsw {} {}, {}",
                result_key,
                self.op.build_llvm(),
                T::build_type(),
                op1.build(),
                op2.build()
            )
        };

        func.add_instr(instr);

        Operand::Var(Var::new(result_key))
    }
}

impl ParseConnectedNodes for DAGBinaryNode {
    type ConnectedNodes = (usize, usize);

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        match (connected_nodes.get(0), connected_nodes.get(1)) {
            (Some(node_id_1), Some(node_id_2)) => (*node_id_1, *node_id_2),
            _ => panic!("Expected two nodes connected to DAGBinaryNode"),
        }
    }
}

impl DAGNodeTrait for DAGBinaryNode {
    fn expected_connected_nodes(&self) -> usize {
        2
    }
}

impl Dissasemble for DAGBinaryNode {
    fn dissasemble(&self) -> String {
        self.op.dissasemble()
    }
}
