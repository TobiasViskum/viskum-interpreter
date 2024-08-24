use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::{
    ds::value::ops::BinaryOp,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, OpTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGBinaryNode {
    op: BinaryOp,
    result_type: ValueType,
}

impl DAGBinaryNode {
    pub fn new(op: BinaryOp, result_type: ValueType) -> Self {
        Self { op, result_type }
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }
}

impl DAGNodeGenerateLLVM for DAGBinaryNode {
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
        let (connected_node_id_1, connected_node_id_2) = self.parse_connected_nodes(
            dag.get_connected_node_ids(node_id)
        );

        let (op1, op2) = (
            dag.generate_llvm(connected_node_id_1, func, llvm_builder),
            dag.generate_llvm(connected_node_id_2, func, llvm_builder),
        );

        let result_key = llvm_builder.req_ssa_key();

        let instr = if self.op.is_cmp() {
            format!(
                "%{} = icmp {} {} {}, {}",
                result_key,
                self.op.build_llvm(),
                self.result_type.to_llvm_type().build(),
                op1.build(),
                op2.build()
            )
        } else {
            format!(
                "%{} = {} nsw {} {}, {}",
                result_key,
                self.op.build_llvm(),
                self.result_type.to_llvm_type().build(),
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
