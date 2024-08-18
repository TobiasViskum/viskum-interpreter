mod decision_node;
mod goto_node;
mod process_node;
mod return_node;
mod terminate_node;
mod connect_node;
mod drop_node;
mod label_node;

use colored::Colorize;
pub use decision_node::CFGDecisionNode;
pub use drop_node::CFGDropNode;
pub use goto_node::CFGGotoNode;
pub use process_node::CFGProcessNode;
pub use return_node::CFGReturnNode;
pub use terminate_node::CFGTerminateNode;
pub use label_node::CFGLabelNode;

use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ CFGNodeTrait, Dissasemble, GenerateLLVM, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub enum CFGNodeType {
    ProcessNode(CFGProcessNode),
    DecisionNode(CFGDecisionNode),
    GotoNode(CFGGotoNode),
    ReturnNode(CFGReturnNode),
    TerminateNode(CFGTerminateNode),
    LabelNode(CFGLabelNode),
    DropNode(CFGDropNode),
}

impl Dissasemble for CFGNodeType {
    fn dissasemble(&self) -> String {
        match self {
            Self::ProcessNode(node) => node.dissasemble(),
            Self::DecisionNode(node) => node.dissasemble(),
            Self::GotoNode(node) => node.dissasemble(),
            Self::ReturnNode(node) => node.dissasemble(),
            Self::LabelNode(node) => node.dissasemble(),
            Self::TerminateNode(node) => node.dissasemble(),
            Self::DropNode(node) => node.dissasemble(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CFGNodeState {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct CFGNode {
    node_type: CFGNodeType,
    node_state: CFGNodeState,
}

impl CFGNode {
    pub fn new(node_type: CFGNodeType) -> Self {
        Self {
            node_type,
            node_state: CFGNodeState::Dead,
        }
    }

    pub fn get_node_state(&self) -> CFGNodeState {
        self.node_state
    }

    pub fn get_mut_node_type(&mut self) -> &mut CFGNodeType {
        &mut self.node_type
    }

    pub fn get_node_type(&self) -> &CFGNodeType {
        &self.node_type
    }

    pub fn mark_alive(&mut self) {
        self.node_state = CFGNodeState::Alive;
    }

    pub fn is_alive(&self) -> bool {
        self.node_state == CFGNodeState::Alive
    }
}

impl Dissasemble for CFGNode {
    fn dissasemble(&self) -> String {
        let node_string = self.node_type.dissasemble();
        format!("{}", {
            if self.is_alive() { node_string } else { node_string.as_str().dimmed().to_string() }
        })
    }
}
