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

impl Dissasemble for CFGTerminateNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
