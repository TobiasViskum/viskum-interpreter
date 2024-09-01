use super::BuildLLVM;

#[derive(Clone, Copy)]
pub struct Var {
    ssa_subscript: usize,
}

impl Var {
    pub fn new(ssa_subscript: usize) -> Self {
        Self { ssa_subscript }
    }

    pub fn get_ssa_subscript(&self) -> usize {
        self.ssa_subscript
    }
}

impl BuildLLVM for Var {
    fn build(&self) -> String {
        format!("%v{}", self.ssa_subscript)
    }
}
