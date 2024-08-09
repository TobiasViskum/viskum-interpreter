mod cfg_node;
mod cfg_edge;

pub use cfg_node::*;
pub use cfg_edge::*;

use crate::{
    compiler::{
        ds::{ register_allocator::{ self, RegisterAllocator }, vm_builder::VMBuilder },
        traits::{ Dissasemble, GenerateBytecode },
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

    // pub fn generate_instructions(&self, vm_builder: &mut VMBuilder) -> Vec<Instruction> {
    //     let mut instructions = vec![];
    //     let mut register_allocator = RegisterAllocator::new();

    //     match self.nodes[0].get_node_type() {
    //         CFGNodeType::ProcessNode(node) => {
    //             node.get_dag().generate_instructions(
    //                 node.get_dag().get_entry_node_id(),
    //                 &mut instructions,
    //                 &mut register_allocator
    //             );
    //         }
    //         _ => unimplemented!(),
    //     }

    //     instructions
    // }
}

impl GenerateBytecode for CFG {
    fn load_constants(&self, vm_builder: &mut VMBuilder) {
        match self.nodes[0].get_node_type() {
            CFGNodeType::ProcessNode(process_node) => {
                process_node.get_dag().load_constants(vm_builder)
            }
            _ => todo!(),
        }
    }

    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = vec![];

        match self.nodes[0].get_node_type() {
            CFGNodeType::ProcessNode(node) => {
                instructions.extend(node.get_dag().generate_instructions(register_allocator));
            }
            _ => unimplemented!(),
        }

        instructions
    }
}

impl Dissasemble for CFG {
    fn dissasemble(&self) -> String {
        self.nodes[0].dissasemble()
    }
}
