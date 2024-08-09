use crate::compiler::traits::Dissasemble;

#[derive(Debug)]
pub enum TerminationState {
    Start,
    End,
}

#[derive(Debug)]
pub struct CFGTerminateNode {
    termination_state: TerminationState,
}

impl CFGTerminateNode {
    pub fn new_start() -> Self {
        Self { termination_state: TerminationState::Start }
    }

    pub fn new_end() -> Self {
        Self { termination_state: TerminationState::End }
    }
}

impl Dissasemble for CFGTerminateNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
