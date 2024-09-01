use std::rc::Rc;

use crate::compiler::ds::symbol_table::{ NativeSymbolFn, UserNativeSymbolFn };
use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{
    BuildLLVM,
    Function,
    LLVMBuilder,
    Module,
    Operand,
    RawOperand,
    Type,
    TypeI32,
    Var,
};

use crate::compiler::traits::NativeFnTrait;
use crate::compiler::{
    ds::ssa_ident::SSAIdent,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGNativeFnCallNode {
    native_fn: UserNativeSymbolFn,
    args_types: Vec<ValueType>,
}

impl DAGNativeFnCallNode {
    pub fn new(native_fn: UserNativeSymbolFn, args_types: Vec<ValueType>) -> Self {
        Self { native_fn, args_types }
    }

    pub fn get_native_fn(&self) -> UserNativeSymbolFn {
        self.native_fn
    }
}

impl DAGNodeGenerateLLVM for DAGNativeFnCallNode {
    fn alloc_llvm(
        &self,
        llvm_builder: &mut LLVMBuilder,

        _func: &mut Function
    ) {
        llvm_builder.get_mut_mod().push_native_fn(self.native_fn.to_native_symbol_fn());
    }

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let connected_node_ids = self.parse_connected_nodes(dag.get_connected_node_ids(node_id));

        let mut args_str = String::new();

        for i in 0..self.args_types.len() {
            let operand = dag.generate_llvm(connected_node_ids[i], ssa_var, func, llvm_builder);

            args_str += format!(
                "{} {}",
                self.args_types[i].to_llvm_type().build_as_arg(),
                operand.build()
            ).as_str();

            if i != self.args_types.len() - 1 {
                args_str += ", ";
            }
        }

        let ssa_key = match self.native_fn.get_llvm_ret_type() {
            v if v.is(&Type::Void) => {
                func.add_instr(
                    format!(
                        "call {} @{}({})",
                        self.native_fn.build_call_type(),
                        self.native_fn.get_llvm_ident(),
                        args_str
                    )
                );
                0
            }
            v => {
                let ssa_key = llvm_builder.req_ssa_key();

                func.add_instr(
                    format!(
                        "%v{} = call {} @{}({})",
                        ssa_key,
                        self.native_fn.build_call_type(),
                        self.native_fn.get_llvm_ident(),
                        args_str
                    )
                );
                ssa_key
            }
        };

        Operand::Var(Var::new(ssa_key))
    }
}

impl ParseConnectedNodes for DAGNativeFnCallNode {
    type ConnectedNodes = Vec<usize>;

    fn parse_connected_nodes(&self, mut connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        if self.args_types.len() < connected_nodes.len() {
            connected_nodes.pop();
        }
        connected_nodes
    }
}

impl DAGNodeTrait for DAGNativeFnCallNode {
    fn expected_connected_nodes(&self) -> usize {
        self.args_types.len()
    }
}

impl Dissasemble for DAGNativeFnCallNode {
    fn dissasemble(&self) -> String {
        format!("{}(", self.native_fn.get_lang_ident().unwrap())
    }
}
