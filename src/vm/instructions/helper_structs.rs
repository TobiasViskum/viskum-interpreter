#[derive(Debug, Clone, Copy)]
pub struct InstructionRegister {
    pub register: usize,
    pub scope: usize,
}

impl InstructionRegister {
    pub fn new(register: usize, scope: usize) -> Self {
        Self { register, scope }
    }

    pub fn dissassemble(&self) -> String {
        format!("S{}:R{}", self.scope, self.register)
    }
}
