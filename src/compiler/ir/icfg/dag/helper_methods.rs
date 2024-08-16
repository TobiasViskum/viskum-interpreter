use std::slice::SliceIndex;

use crate::{
    compiler::{
        ds::{
            register_allocator::RegisterAllocator,
            symbol_table::SSAKey,
            value::{ ops::{ BinaryOp, ComparisonOp, UnaryOp }, Value },
            vm_builder::VMBuilder,
        },
        traits::{ DAGNodeTrait, ParseConnectedNodes },
    },
    vm::instructions::{ Instruction, Reg },
};

use super::{
    dag_nodes::DAGBinaryNode,
    DAGAssignNode,
    DAGDefineNode,
    DAGGroupNode,
    DAGNode,
    DAGUnaryNode,
    DAG,
};

impl DAG {
    pub(super) fn generate_group_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        group_node: &DAGGroupNode,
        is_condition: bool
    ) -> Reg {
        let connected_node_id = group_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        self.generate_instruction(connected_node_id, instructions, register_allocator, is_condition)
    }

    pub(super) fn generate_unary_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        unary_node: &DAGUnaryNode,
        is_condition: bool
    ) -> Reg {
        let connected_node_id = unary_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        let src_reg = self.generate_instruction(
            connected_node_id,
            instructions,
            register_allocator,
            is_condition
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
        binary_node: &DAGBinaryNode,
        is_condition: bool
    ) -> Reg {
        let (connected_node_id_1, connected_node_id_2) = binary_node.parse_connected_nodes(
            self.get_connected_node_ids(node_id)
        );

        let src_regs = (
            self.generate_instruction(
                connected_node_id_1,
                instructions,
                register_allocator,
                is_condition
            ),
            self.generate_instruction(
                connected_node_id_2,
                instructions,
                register_allocator,
                is_condition
            ),
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
                if is_condition {
                    register_allocator.free_temp_reg(Reg::Rel(dst_reg));
                }

                if is_condition {
                    match comparison_op {
                        ComparisonOp::Eq => {
                            instructions.push(Instruction::JmpCmpEqInt {
                                true_jmp_pos: 0,
                                false_jmp_pos: 0,
                                src1_reg: src_regs.0,
                                src2_reg: src_regs.1,
                            });
                        }
                        ComparisonOp::Ne => {
                            instructions.push(Instruction::JmpCmpNeInt {
                                true_jmp_pos: 0,
                                false_jmp_pos: 0,
                                src1_reg: src_regs.0,
                                src2_reg: src_regs.1,
                            });
                        }
                        ComparisonOp::Ge => {
                            instructions.push(Instruction::JmpCmpGeInt {
                                true_jmp_pos: 0,
                                false_jmp_pos: 0,
                                src1_reg: src_regs.0,
                                src2_reg: src_regs.1,
                            });
                        }
                        ComparisonOp::Gt => {
                            instructions.push(Instruction::JmpCmpGtInt {
                                true_jmp_pos: 0,
                                false_jmp_pos: 0,
                                src1_reg: src_regs.0,
                                src2_reg: src_regs.1,
                            });
                        }
                        ComparisonOp::Le => {
                            instructions.push(Instruction::JmpCmpLeInt {
                                true_jmp_pos: 0,
                                false_jmp_pos: 0,
                                src1_reg: src_regs.0,
                                src2_reg: src_regs.1,
                            });
                        }
                        ComparisonOp::Lt => {
                            instructions.push(Instruction::JmpCmpLtInt {
                                true_jmp_pos: 0,
                                false_jmp_pos: 0,
                                src1_reg: src_regs.0,
                                src2_reg: src_regs.1,
                            });
                        }
                    }
                    register_allocator.free_temp_reg(src_regs.0);
                    register_allocator.free_temp_reg(src_regs.1);
                } else {
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
        define_node: &DAGDefineNode,
        is_condition: bool
    ) -> Reg {
        let connected_node_id = define_node
            .parse_connected_nodes(self.get_connected_node_ids(node_id))
            .expect("Right now only initialized definitions are supported");

        let src_reg = self.generate_instruction(
            connected_node_id,
            instructions,
            register_allocator,
            is_condition
        );

        let ssa_key = define_node.get_ssa_key().get_ident();

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
        assign_node: &DAGAssignNode,
        is_condition: bool
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
            register_allocator,
            is_condition
        );

        let var_reg = register_allocator.get_var_reg(ident_node.get_ssa_key().get_ident());

        instructions.push(Instruction::Copy {
            dst_reg: var_reg.get_idx(),
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
