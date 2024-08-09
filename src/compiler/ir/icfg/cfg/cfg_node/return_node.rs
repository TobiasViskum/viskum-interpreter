use crate::compiler::traits::Dissasemble;

#[derive(Debug)]
pub struct CFGReturnNode;

impl Dissasemble for CFGReturnNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
