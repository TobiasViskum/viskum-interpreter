use crate::BuildLLVM;

#[derive(Clone, Copy)]
pub struct Var {
    ssa_ident: usize,
}

impl Var {
    pub fn new(ssa_ident: usize) -> Self {
        Self { ssa_ident }
    }
}

impl BuildLLVM for Var {
    fn build(&self) -> String {
        format!("%{}", self.ssa_ident)
    }
}
