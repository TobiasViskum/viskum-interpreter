use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ CFGNodeTrait, Dissasemble, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug, PartialEq)]
pub enum TerminationState {
    Start,
    End,
}

#[derive(Debug)]
pub struct CFGTerminateNode {
    termination_state: TerminationState,
}

impl CFGNodeTrait for CFGTerminateNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        if !self.is_start() { vec![Instruction::Halt] } else { vec![] }
    }
}

impl CFGTerminateNode {
    pub fn new_start() -> Self {
        Self { termination_state: TerminationState::Start }
    }

    pub fn new_end() -> Self {
        Self { termination_state: TerminationState::End }
    }

    pub fn is_start(&self) -> bool {
        self.termination_state == TerminationState::Start
    }
}

impl ParseConnectedNodes for CFGTerminateNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        *connected_nodes.get(0).expect("Expected one node connected to CFGTerminateNode")
    }
}

impl Dissasemble for CFGTerminateNode {
    fn dissasemble(&self) -> String {
        "".to_string()
    }
}
