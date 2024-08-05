use crate::compiler::traits::Dissasemble;

#[derive(Debug)]
pub struct CFGDropNode {}

impl Dissasemble for CFGDropNode {
    fn dissasemble(&self) -> String {
        "DROP".to_string()
    }
}
