use std::slice::SliceIndex;

use crate::{
    compiler::{
        ds::{
            register_allocator::RegisterAllocator,
            symbol_table::SSAKey,
            value::{ ops::{ BinaryOp, ComparisonOp, UnaryOp }, Value },
            vm_builder::VMBuilder,
        },
        traits::DAGNodeTrait,
    },
    vm::instructions::{ Instruction, Reg },
};

use super::{
    DAGAssignNode,
    DAGBinaryNode,
    DAGDefineNode,
    DAGDropNode,
    DAGGroupNode,
    DAGNode,
    DAGUnaryNode,
    DAG,
};

impl DAG {
    pub(super) fn generate_drop_instruction(
        &self,
        node_id: usize,
        _: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        drop_node: &DAGDropNode
    ) {
        let connected_node_ids = drop_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        for connected_node_id in connected_node_ids {
            register_allocator.free_from_dag_node(&self.nodes[connected_node_id]);
        }
    }

    pub(super) fn generate_group_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        group_node: &DAGGroupNode
    ) -> Reg {
        let connected_node_id = group_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        self.generate_instruction(connected_node_id, instructions, register_allocator)
    }

    pub(super) fn generate_unary_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        unary_node: &DAGUnaryNode
    ) -> Reg {
        let connected_node_id = unary_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        let src_reg = self.generate_instruction(
            connected_node_id,
            instructions,
            register_allocator
        );

        let dst_reg = register_allocator.alloc_temp_reg();

        match unary_node.get_op() {
            UnaryOp::Neg => {
                instructions.push(Instruction::NegInt { dst_reg, src_reg });
            }
            _ => unimplemented!(),
        }

        Reg::Rel(dst_reg)
    }

    pub(super) fn generate_binary_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        binary_node: &DAGBinaryNode
    ) -> Reg {
        let (connected_node_id_1, connected_node_id_2) = binary_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        let src_regs = (
            self.generate_instruction(connected_node_id_1, instructions, register_allocator),
            self.generate_instruction(connected_node_id_2, instructions, register_allocator),
        );

        let dst_reg = register_allocator.alloc_temp_reg();

        match binary_node.get_op() {
            BinaryOp::Add => {
                instructions.push(Instruction::AddInt {
                    dst_reg,
                    src1_reg: src_regs.0,
                    src2_reg: src_regs.1,
                });
            }
            BinaryOp::Sub => {
                instructions.push(Instruction::SubInt {
                    dst_reg,
                    src1_reg: src_regs.0,
                    src2_reg: src_regs.1,
                });
            }
            BinaryOp::Mul => {
                instructions.push(Instruction::MulInt {
                    dst_reg,
                    src1_reg: src_regs.0,
                    src2_reg: src_regs.1,
                });
            }
            BinaryOp::Div => {
                instructions.push(Instruction::DivInt {
                    dst_reg,
                    src1_reg: src_regs.0,
                    src2_reg: src_regs.1,
                });
            }
            BinaryOp::ComparisonOp(comparison_op) => {
                match comparison_op {
                    ComparisonOp::Eq => {
                        instructions.push(Instruction::CmpEqInt {
                            dst_reg,
                            src1_reg: src_regs.0,
                            src2_reg: src_regs.1,
                        });
                    }
                    ComparisonOp::Ne => {
                        instructions.push(Instruction::CmpNeInt {
                            dst_reg,
                            src1_reg: src_regs.0,
                            src2_reg: src_regs.1,
                        });
                    }
                    ComparisonOp::Ge => {
                        instructions.push(Instruction::CmpGeInt {
                            dst_reg,
                            src1_reg: src_regs.0,
                            src2_reg: src_regs.1,
                        });
                    }
                    ComparisonOp::Gt => {
                        instructions.push(Instruction::CmpGtInt {
                            dst_reg,
                            src1_reg: src_regs.0,
                            src2_reg: src_regs.1,
                        });
                    }
                    ComparisonOp::Le => {
                        instructions.push(Instruction::CmpLeInt {
                            dst_reg,
                            src1_reg: src_regs.0,
                            src2_reg: src_regs.1,
                        });
                    }
                    ComparisonOp::Lt => {
                        instructions.push(Instruction::CmpLtInt {
                            dst_reg,
                            src1_reg: src_regs.0,
                            src2_reg: src_regs.1,
                        });
                    }
                }
            }
        }

        register_allocator.free_temp_reg(src_regs.0);
        register_allocator.free_temp_reg(src_regs.1);

        Reg::Rel(dst_reg)
    }

    pub(super) fn generate_define_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        define_node: &DAGDefineNode
    ) -> Reg {
        let connected_node_id = define_node
            .parse_connected_nodes(self.get_connected_node_ids(node_id))
            .expect("Right now only initialized definitions are supported");

        let src_reg = self.generate_instruction(
            connected_node_id,
            instructions,
            register_allocator
        );

        let ssa_key = define_node.get_ssa_key().clone();

        match define_node.get_is_mutable() {
            true => {
                instructions.push(Instruction::Copy {
                    dst_reg: register_allocator.alloc_var_reg(ssa_key),
                    src_reg: src_reg,
                });
            }
            false => register_allocator.mark_reg_as_var(ssa_key, src_reg),
        }

        src_reg
    }

    pub(super) fn generate_assign_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        assign_node: &DAGAssignNode
    ) -> Reg {
        let connected_node_ids = assign_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        let ident_node = match &self.nodes[connected_node_ids.0] {
            DAGNode::IdentNode(ident_node) => ident_node,
            _ => panic!("Right now only identifiers is supported in assignment"),
        };

        let src_reg = self.generate_instruction(
            connected_node_ids.1,
            instructions,
            register_allocator
        );

        instructions.push(Instruction::Copy {
            dst_reg: register_allocator.alloc_var_reg(ident_node.get_ssa_key().clone()),
            src_reg: src_reg,
        });

        src_reg
    }

    pub(super) fn generate_const_instruction(
        &self,
        register_allocator: &mut RegisterAllocator,
        const_value: &Value
    ) -> Reg {
        if let Some(simple_const) = const_value.get_as_simple_const() {
            register_allocator.get_const_reg(simple_const)
        } else {
            unimplemented!("Implement complex consts")
        }
    }
}
