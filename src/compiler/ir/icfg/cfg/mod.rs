mod cfg_node;
mod cfg_edge;

use ahash::AHashMap;
pub use cfg_node::*;
pub use cfg_edge::*;
use llvm_builder::{ Function, LLVMBuilder, Module, Type, TypeI32 };

use crate::{
    compiler::{
        ds::{
            register_allocator::{ self, RegisterAllocator },
            value::ValueType,
            vm_builder::VMBuilder,
        },
        traits::{
            AllocLLVM,
            CFGNodeGenerateLLVM,
            CFGNodeTrait,
            Dissasemble,
            GenerateLLVM,
            LoadConstants,
            ParseConnectedNodes,
        },
    },
    vm::instructions::Instruction,
};

pub type CFGNodeId = usize;

/* 

What is true for a CFG:

- Goto nodes and decision nodes are always connected to a label node
    - These are also the only nodes, that is connected to label nodes

- Other nodes are connected to the next node, which is not a label node
    - Including label nodes

*/

#[derive(Debug)]
pub struct CFG {
    nodes: Vec<CFGNode>,
    edges: Vec<CFGEdge>,
    fn_name: String,
    ret_type: ValueType,
}

impl CFG {
    pub fn new(fn_name: String, ret_type: ValueType) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            fn_name,
            ret_type,
        }
    }

    pub fn get_mut_node(&mut self, node_id: usize) -> Option<&mut CFGNode> {
        self.nodes.get_mut(node_id)
    }

    pub fn iter_nodes(&self) -> core::slice::Iter<CFGNode> {
        self.nodes.iter()
    }

    pub fn push_node(&mut self, cfg_node: CFGNode) -> CFGNodeId {
        self.nodes.push(cfg_node);
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, origin: usize, dest: usize) {
        self.edges.push(CFGEdge::new(origin, dest));
    }

    pub fn get_connected_nodes(&self, node_id: usize) -> Vec<usize> {
        let mut connected_nodes = vec![];
        for edge in self.edges.iter() {
            if edge.get_origin() == node_id {
                connected_nodes.push(edge.get_dest());
            }
        }

        connected_nodes
    }

    pub fn last_mut_node(&mut self) -> Option<&mut CFGNode> {
        self.nodes.last_mut()
    }

    pub fn get_last_added_node_id(&self) -> usize {
        self.nodes.len() - 1
    }

    pub fn print_nodes(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            let connected_nodes_str = format!("{:?}", self.get_connected_nodes(i))
                .replace("[", "")
                .replace("]", "");

            let node_str = match node.get_node_type() {
                CFGNodeType::DecisionNode(_) => format!("DecisionNode({})", connected_nodes_str),
                CFGNodeType::DropNode(_) => format!("DropNode({})", connected_nodes_str),
                CFGNodeType::GotoNode(_) => format!("GotoNode({})", connected_nodes_str),
                CFGNodeType::ProcessNode(_) => format!("ProcessNode({})", connected_nodes_str),
                CFGNodeType::ReturnNode(_) => format!("ReturnNode({})", connected_nodes_str),
                CFGNodeType::TerminateNode(_) => format!("TerminateNode({})", connected_nodes_str),
                CFGNodeType::LabelNode(_) => format!("LabelNode({})", connected_nodes_str),
            };
            println!("{}: {}", i, node_str);
        }
    }
}

impl LoadConstants for CFG {
    fn load_constants(&self, vm_builder: &mut VMBuilder) {
        for node in self.nodes.iter().map(|node| node.get_node_type()) {
            match node {
                CFGNodeType::ProcessNode(process_node) => {
                    process_node.get_dag().load_constants(vm_builder);
                }
                CFGNodeType::DecisionNode(decision_node) => {
                    decision_node.get_condition().load_constants(vm_builder);
                }
                _ => {}
            }
        }
    }
}

impl AllocLLVM for CFG {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, module: &mut Module, func: &mut Function) {
        for cfg_node_type in self.nodes.iter().map(|node| node.get_node_type()) {
            match cfg_node_type {
                CFGNodeType::ProcessNode(process_node) => {
                    process_node.alloc_llvm(llvm_builder, module, func);
                }
                CFGNodeType::TerminateNode(terminate_node) => {
                    terminate_node.alloc_llvm(llvm_builder, module, func);
                }
                CFGNodeType::DecisionNode(decision_node) => {
                    decision_node.alloc_llvm(llvm_builder, module, func);
                }
                CFGNodeType::DropNode(drop_node) => {
                    drop_node.alloc_llvm(llvm_builder, module, func);
                }
                CFGNodeType::GotoNode(goto_node) => {
                    goto_node.alloc_llvm(llvm_builder, module, func);
                }
                CFGNodeType::ReturnNode(return_node) => {
                    return_node.alloc_llvm(llvm_builder, module, func);
                }
                CFGNodeType::LabelNode(label_node) => {
                    label_node.alloc_llvm(llvm_builder, module, func);
                }
            }
        }
    }
}

impl GenerateLLVM for CFG {
    fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) {
        for (i, cfg_node_type) in self.nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (i, node.get_node_type())) {
            match cfg_node_type {
                CFGNodeType::ProcessNode(process_node) => {
                    process_node.build_llvm(llvm_builder, func);
                }
                CFGNodeType::DecisionNode(decision_node) => {
                    decision_node.build_llvm(i, llvm_builder, func, self);
                }
                CFGNodeType::LabelNode(label_node) => {
                    label_node.build_llvm(i, llvm_builder, func, self);
                }
                CFGNodeType::GotoNode(goto_node) => {
                    goto_node.build_llvm(i, llvm_builder, func, self);
                }
                CFGNodeType::TerminateNode(_) => {}
                _ => {
                    unimplemented!("CFGNode not supported (build_llvm)");
                }
            }
        }
    }
}

impl CFG {
    pub fn get_fn_info(&self) -> (&String, ValueType) {
        (&self.fn_name, self.ret_type.clone())
    }

    pub fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = vec![];
        for cfg_node_type in self.nodes.iter().map(|node| node.get_node_type()) {
            let node_instructions = match cfg_node_type {
                CFGNodeType::LabelNode(label_node) => {
                    label_node.generate_instructions(register_allocator)
                }
                CFGNodeType::ProcessNode(process_node) => {
                    process_node.generate_instructions(register_allocator)
                }
                CFGNodeType::DecisionNode(decision_node) => {
                    decision_node.generate_instructions(register_allocator)
                }
                CFGNodeType::GotoNode(goto_node) => {
                    goto_node.generate_instructions(register_allocator)
                }
                CFGNodeType::ReturnNode(return_node) => {
                    return_node.generate_instructions(register_allocator)
                }
                CFGNodeType::DropNode(drop_node) => {
                    drop_node.generate_instructions(register_allocator)
                }
                CFGNodeType::TerminateNode(terminate_node) => {
                    terminate_node.generate_instructions(register_allocator)
                }
            };
            instructions.push(node_instructions);
        }

        let instructions_lengths = instructions
            .iter()
            .map(|instructions| instructions.len())
            .collect::<Vec<_>>();

        let collect_instr_ids_until_id = |node_id: usize| -> usize {
            instructions_lengths
                .iter()
                .enumerate()
                .map_while(|(i, instrs_len)| if i <= node_id { Some(*instrs_len) } else { None })
                .sum::<usize>()
        };

        for (cfg_node_id, instructions) in instructions.iter_mut().enumerate() {
            for instruction in instructions.iter_mut() {
                match instruction {
                    Instruction::Goto { jmp_pos } => {
                        let instr_id = collect_instr_ids_until_id(
                            self.get_connected_nodes(cfg_node_id)[0] - 1
                        );
                        *jmp_pos = instr_id;
                    }
                    | Instruction::JmpCmpEqInt { true_jmp_pos, false_jmp_pos, .. }
                    | Instruction::JmpCmpNeInt { true_jmp_pos, false_jmp_pos, .. }
                    | Instruction::JmpCmpGeInt { true_jmp_pos, false_jmp_pos, .. }
                    | Instruction::JmpCmpGtInt { true_jmp_pos, false_jmp_pos, .. }
                    | Instruction::JmpCmpLeInt { true_jmp_pos, false_jmp_pos, .. }
                    | Instruction::JmpCmpLtInt { true_jmp_pos, false_jmp_pos, .. } => {
                        let true_instr_id = collect_instr_ids_until_id(
                            self.get_connected_nodes(cfg_node_id)[0] - 1
                        );
                        *true_jmp_pos = true_instr_id;

                        let false_instr_id = collect_instr_ids_until_id(
                            self.get_connected_nodes(cfg_node_id)[1] - 1
                        );
                        *false_jmp_pos = false_instr_id;
                    }
                    _ => {}
                }
            }
        }

        let instructions = instructions.into_iter().fold(vec![], |mut acc, instrs| {
            acc.extend(instrs);
            acc
        });

        instructions
    }
}

impl Dissasemble for CFG {
    fn dissasemble(&self) -> String {
        self.nodes[1].dissasemble()
    }
}
