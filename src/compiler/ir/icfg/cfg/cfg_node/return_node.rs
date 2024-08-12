use crate::{
    compiler::{
        ds::register_allocator::RegisterAllocator,
        traits::{ CFGNodeTrait, Dissasemble, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

#[derive(Debug)]
pub struct CFGReturnNode;

impl CFGNodeTrait for CFGReturnNode {
    fn generate_instructions(&self, _: &mut RegisterAllocator) -> Vec<Instruction> {
        todo!()
    }
}

impl ParseConnectedNodes for CFGReturnNode {
    type ConnectedNodes = usize;

    fn parse_connected_nodes(&self, _connected_nodes: Vec<usize>) -> Self::ConnectedNodes {
        todo!()
    }
}

impl Dissasemble for CFGReturnNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
