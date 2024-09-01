use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, value::ValueType },
    ir::icfg::dag::DAG,
    llvm_builder::{
        BuildLLVM,
        ConstStringPointer,
        Function,
        LLVMBuilder,
        Operand,
        RawOperand,
        TypeI1,
        TypeI32,
        Var,
    },
    traits::{ DAGNodeGenerateLLVM, DAGNodeTrait, ParseConnectedNodes },
};

#[derive(Debug)]
pub struct DAGDeclareNode {
    ssa_ident: SSAIdent,
    value_type: ValueType,
}

impl DAGNodeGenerateLLVM for DAGDeclareNode {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        match &self.value_type {
            ValueType::String => {
                llvm_builder.get_mut_mod().add_string_const("".into());
            }
            ValueType::Ptr(_) => {}
            ValueType::Array((_, _)) => {}
            ValueType::Int | ValueType::Bool | ValueType::Void => {}
        }

        let var_key = llvm_builder.req_var_ssa_key(self.ssa_ident.get_ident());
        func.add_instr(
            format!(
                "%v{} = alloca {}, align {}",
                var_key,
                self.value_type.to_llvm_type().build(),
                self.value_type.to_llvm_type().get_byte_size()
            )
        );
    }

    fn generate_llvm(
        &self,
        node_id: usize,
        ssa_var: Option<Var>,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand {
        let var_key = llvm_builder.get_var_ssa_key(self.ssa_ident.get_ident());

        let default_value_operand = match &self.value_type {
            ValueType::Int => Some(Operand::Raw(RawOperand::TypeI32(TypeI32::new(0)))),
            ValueType::Bool => Some(Operand::Raw(RawOperand::TypeI1(TypeI1::new(false)))),
            ValueType::String => {
                Some(
                    Operand::Raw(
                        RawOperand::ConstStringPointer(
                            ConstStringPointer::new(
                                llvm_builder.get_mod().get_const_string_idx(&"".into())
                            )
                        )
                    )
                )
            }
            ValueType::Array((items_type, items_count)) => { None }
            ValueType::Ptr(_) => None,
            ValueType::Void => None,
        };

        if let Some(default_value_operand) = default_value_operand {
            func.add_instr(
                format!(
                    "store {} {}, ptr %v{}, align 4",
                    self.value_type.to_llvm_type().build(),
                    default_value_operand.build(),
                    var_key
                )
            );
        }

        Operand::Var(Var::new(var_key))
    }
}

impl ParseConnectedNodes for DAGDeclareNode {
    type ConnectedNodes = ();

    fn parse_connected_nodes(&self, _: Vec<usize>) -> Self::ConnectedNodes {
        ()
    }
}

impl DAGNodeTrait for DAGDeclareNode {
    fn expected_connected_nodes(&self) -> usize {
        0
    }
}

impl DAGDeclareNode {
    pub fn new(ssa_ident: SSAIdent, value_type: ValueType) -> Self {
        Self {
            ssa_ident,
            value_type,
        }
    }
}
