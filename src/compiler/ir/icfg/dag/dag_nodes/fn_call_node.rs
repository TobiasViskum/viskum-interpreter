use std::rc::Rc;

use crate::compiler::ds::value::ValueType;
use crate::compiler::llvm_builder::{ BuildLLVM, Function, LLVMBuilder, Module, Operand, Type, Var };

use crate::compiler::{
    ds::ssa_ident::SSAIdent,
    ir::icfg::dag::DAG,
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
    Dissasemble,
};

#[derive(Debug)]
pub struct DAGFnCallNode {
    args_types: Vec<ValueType>,
    ssa_ident: SSAIdent,
    result_type: ValueType,
}

impl DAGFnCallNode {
    pub fn new(args_types: Vec<ValueType>, ssa_ident: SSAIdent, result_type: ValueType) -> Self {
        Self { args_types, ssa_ident, result_type }
    }

    pub fn get_ident(&self) -> Rc<str> {
        self.ssa_ident.get_ident()
    }

    pub fn get_ssa_ident(&self) -> &SSAIdent {
        &self.ssa_ident
    }
}

impl DAGNodeGenerateLLVM for DAGFnCallNode {
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
        let ssa_key = match self.result_type.to_llvm_type() {
            v if v.is(&Type::Void) => {
                func.add_instr(format!("call {} @{}()", v.build(), self.ssa_ident.build()));
                0
            }
            v => {
                let _ = llvm_builder.req_ssa_key();
                let _ = llvm_builder.req_ssa_key();
                let _ = llvm_builder.req_ssa_key();
                let _ = llvm_builder.req_ssa_key();
                let ssa_key = llvm_builder.req_ssa_key();
                func.add_instr(
                    format!("%{} = call {} @{}()", ssa_key, v.build(), self.ssa_ident.build())
                );
                ssa_key
            }
        };

        Operand::Var(Var::new(ssa_key))
    }
}

impl ParseConnectedNodes for DAGFnCallNode {
    type ConnectedNodes = Vec<usize>;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        todo!()
    }
}

impl DAGNodeTrait for DAGFnCallNode {
    fn expected_connected_nodes(&self) -> usize {
        self.args_types.len()
    }
}

impl Dissasemble for DAGFnCallNode {
    fn dissasemble(&self) -> String {
        format!("{}(", self.ssa_ident.dissasemble())
    }
}
