mod cfg_node;
mod cfg_edge;

use ahash::AHashMap;
pub use cfg_node::*;
pub use cfg_edge::*;

use crate::{
    compiler::{
        ds::{ register_allocator::{ self, RegisterAllocator }, vm_builder::VMBuilder },
        traits::{ CFGNodeTrait, Dissasemble, LoadConstants, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

pub type CFGNodeId = usize;

#[derive(Debug)]
pub struct CFG {
    nodes: Vec<CFGNode>,
    edges: Vec<CFGEdge>,
}

impl CFG {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
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

impl CFG {
    pub fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = vec![];
        for cfg_node_type in self.nodes.iter().map(|node| node.get_node_type()) {
            let node_instructions = match cfg_node_type {
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

    // fn generate_instruction(
    //     &self,
    //     instructions: &mut Vec<Instruction>,
    //     register_allocator: &mut RegisterAllocator,
    //     cfg_node_id_to_instr_id: &mut AHashMap<usize, usize>,
    //     node_id: usize
    // ) {
    //     let node = &self.nodes[node_id];

    //     // if node.get_node_state() == CFGNodeState::Dead {
    //     //     return;
    //     // }

    //     // if let Some(goto_instr_ids) = cfg_node_id_to_goto_instr_id.get(&node_id) {
    //     //     let instr_len = instructions.len();

    //     //     for goto_instr_id in goto_instr_ids.iter() {
    //     //         if let Instruction::Goto { jmp_pos } = &mut instructions[*goto_instr_id] {
    //     //             *jmp_pos = instr_len;
    //     //         }
    //     //     }

    //     //     cfg_node_id_to_goto_instr_id.remove(&node_id);
    //     // }

    //     println!("Node id: {}", node_id);
    //     cfg_node_id_to_instr_id.insert(node_id, instructions.len());

    //     let connected_nodes = self.get_connected_nodes(node_id);

    //     match node.get_node_type() {
    //         CFGNodeType::ProcessNode(process_node) => {
    //             process_node.generate_instructions(register_allocator);

    //             process_node.get_dag().generate_instructions(instructions, register_allocator);
    //             let connected_node_id = process_node.parse_connected_nodes(connected_nodes);

    //             self.generate_instruction(
    //                 instructions,
    //                 register_allocator,
    //                 cfg_node_id_to_instr_id,
    //                 connected_node_id
    //             )
    //         }
    //         CFGNodeType::GotoNode(_) => {
    //             instructions.push(Instruction::Goto { jmp_pos: 0 });
    //         }
    //         CFGNodeType::TerminateNode(terminate_node) => {
    //             if terminate_node.is_start() {
    //                 let connected_node_id = terminate_node.parse_connected_nodes(connected_nodes);
    //                 self.generate_instruction(
    //                     instructions,
    //                     register_allocator,
    //                     cfg_node_id_to_instr_id,
    //                     connected_node_id
    //                 )
    //             }
    //         }
    //         CFGNodeType::DecisionNode(decision_node) => {
    //             let (true_node_id, false_node_id) = {
    //                 decision_node.parse_connected_nodes(connected_nodes)
    //             };

    //             let has_condition = decision_node.get_condition().is_some();

    //             dag.generate_instructions_as_condition(instructions, register_allocator);

    //             let jmp_cmp_instr_id = instructions.len() - 1;

    //             let true_instr_id = instructions.len();

    //             self.generate_instruction(
    //                 instructions,
    //                 register_allocator,
    //                 cfg_node_id_to_instr_id,
    //                 true_node_id
    //             );

    //             let false_instr_id = instructions.len();

    //             self.generate_instruction(
    //                 instructions,
    //                 register_allocator,
    //                 cfg_node_id_to_instr_id,
    //                 false_node_id
    //             );

    //             if has_condition {
    //                 match &mut instructions[jmp_cmp_instr_id] {
    //                     | Instruction::JmpCmpEqInt { true_jmp_pos, false_jmp_pos, .. }
    //                     | Instruction::JmpCmpNeInt { true_jmp_pos, false_jmp_pos, .. }
    //                     | Instruction::JmpCmpGeInt { true_jmp_pos, false_jmp_pos, .. }
    //                     | Instruction::JmpCmpGtInt { true_jmp_pos, false_jmp_pos, .. }
    //                     | Instruction::JmpCmpLeInt { true_jmp_pos, false_jmp_pos, .. }
    //                     | Instruction::JmpCmpLtInt { true_jmp_pos, false_jmp_pos, .. } => {
    //                         *true_jmp_pos = true_instr_id;
    //                         *false_jmp_pos = false_instr_id;
    //                     }
    //                     instr => panic!("Expected JmpCmp instruction. Found {:?}", instr),
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}

impl Dissasemble for CFG {
    fn dissasemble(&self) -> String {
        self.nodes[1].dissasemble()
    }
}
