use crate::compiler::traits::Dissasemble;

#[derive(Debug)]
pub struct CFGGotoNode {}

impl Dissasemble for CFGGotoNode {
    fn dissasemble(&self) -> String {
        todo!()
    }
}
