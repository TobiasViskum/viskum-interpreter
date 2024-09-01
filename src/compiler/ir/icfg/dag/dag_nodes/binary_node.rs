use crate::compiler::ds::symbol_table::{ NativeSymbolFn, StrCmp };
use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Var };

use crate::compiler::traits::NativeFnTrait;
use crate::compiler::{
    ds::value::ops::BinaryOp,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, OpTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGBinaryNode {
    op: BinaryOp,
    op_types: ValueType,
}

impl DAGBinaryNode {
    pub fn new(op: BinaryOp, op_types: ValueType) -> Self {
        Self { op, op_types }
    }

    pub fn get_op(&self) -> BinaryOp {
        self.op
    }
}

impl DAGNodeGenerateLLVM for DAGBinaryNode {
    fn alloc_llvm(
        &self,
        llvm_builder: &mut LLVMBuilder,

        _func: &mut Function
    ) {
        match (self.op, &self.op_types) {
            (BinaryOp::ComparisonOp(_), ValueType::String) => {
                llvm_builder.get_mut_mod().push_native_fn(NativeSymbolFn::StrCmp(StrCmp))
            }
            _ => {}
        }
    }

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let (connected_node_id_1, connected_node_id_2) = self.parse_connected_nodes(
            dag.get_connected_node_ids(node_id)
        );

        let (op1, op2) = (
            dag.generate_llvm(connected_node_id_1, ssa_var, func, llvm_builder),
            dag.generate_llvm(connected_node_id_2, ssa_var, func, llvm_builder),
        );

        let mut result_key = llvm_builder.req_ssa_key();

        let instr = if self.op.is_cmp() {
            match (&self.op_types, self.op) {
                (ValueType::String, BinaryOp::ComparisonOp(op)) => {
                    let mut call_str = format!(
                        "%v{} = call {} @{}(ptr noundef {}, ptr noundef {})\n",
                        result_key,
                        StrCmp.build_call_type(),
                        StrCmp.get_llvm_ident(),
                        op1.build(),
                        op2.build()
                    );

                    let new_result_key = llvm_builder.req_ssa_key();

                    call_str += format!(
                        "    %v{} = icmp {} i32 %v{}, 0",
                        new_result_key,
                        op.build_llvm(),
                        result_key
                    ).as_str();

                    result_key = new_result_key;

                    call_str
                }
                (ValueType::String, BinaryOp::Add) => {
                    panic!("String addition not implemented yet")
                }
                (ValueType::Bool, _) | (ValueType::Int, _) => {
                    format!(
                        "%v{} = icmp {} {} {}, {}",
                        result_key,
                        self.op.build_llvm(),
                        self.op_types.to_llvm_type().build(),
                        op1.build(),
                        op2.build()
                    )
                }

                (ValueType::Array(_) | ValueType::String | ValueType::Ptr(_), _) =>
                    panic!("Undefined op"),
                (ValueType::Void, _) => panic!("Unexepcted void type"),
            }
        } else {
            format!(
                "%v{} = {} nsw {} {}, {}",
                result_key,
                self.op.build_llvm(),
                self.op_types.to_llvm_type().build(),
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
