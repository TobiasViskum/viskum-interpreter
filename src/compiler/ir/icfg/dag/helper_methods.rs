use std::slice::SliceIndex;

use crate::{
    compiler::ds::{
        register_allocator::RegisterAllocator,
        value::{ ops::{ BinaryOp, ComparisonOp, UnaryOp }, Value },
        vm_builder::VMBuilder,
    },
    vm::instructions::{ Instruction, Reg },
};

use super::{ DAGBinaryNode, DAG };

impl DAG {
    pub(super) fn generate_group_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator
    ) -> Reg {
        let connected_node_id = *self
            .get_connected_node_ids(node_id)
            .get(0)
            .expect(format!("Expected one node connected to group node, found {}", 0).as_str());

        self.generate_instruction(connected_node_id, instructions, register_allocator)
    }

    pub(super) fn generate_unary_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        unary_op: UnaryOp
    ) -> Reg {
        let connected_node_id = self.get_connected_node_ids(node_id);

        let src_reg = self.generate_instruction(
            connected_node_id[0],
            instructions,
            register_allocator
        );

        let dst_reg = register_allocator.alloc_reg();

        match unary_op {
            UnaryOp::Neg => {
                instructions.push(Instruction::NegInt { dst_reg, src_reg: src_reg });
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
        binary_op: BinaryOp
    ) -> Reg {
        let connected_node_ids = self.get_connected_node_ids(node_id);

        let src_regs = (
            self.generate_instruction(connected_node_ids[0], instructions, register_allocator),
            self.generate_instruction(connected_node_ids[1], instructions, register_allocator),
        );

        let dst_reg = register_allocator.alloc_reg();

        match binary_op {
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
                    _ => todo!("handle the rest of the comparison ops"),
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
        register_allocator: &mut RegisterAllocator
    ) -> Reg {
        let connected_node_ids = self.get_connected_node_ids(node_id);

        let src_reg: &dyn std::any::Any;

        todo!()
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
