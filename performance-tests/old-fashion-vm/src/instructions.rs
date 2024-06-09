use ahash::AHashMap;

pub struct Register {
    reg: usize,
    is_relative: bool,
}

impl Register {
    pub fn get_reg(&self) -> usize {
        self.reg
    }

    pub fn get_is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn new(reg: usize) -> Self {
        Self {
            reg,
            is_relative: true,
        }
    }
}

pub enum Instruction {
    Halt,
    StoreFnPtr {
        instructions_amount: usize,
    },
    LoadConstOne {
        reg: Register,
        const_pos: usize,
    },
    LoadConstTwo {
        reg1: Register,
        const_pos1: usize,
        reg2: Register,
        const_pos2: usize,
    },
    Add {
        dst: Register,
        reg1: Register,
        reg2: Register,
    },
    Sub {
        dst: Register,
        reg1: Register,
        reg2: Register,
    },
    JmpLt {
        reg1: Register,
        reg2: Register,
        true_pos: usize,
        false_pos: usize,
    },
    Call {
        dst: Register,
        args_count: usize,
    },
    Ret {
        reg: Register,
    },
}

impl Instruction {
    pub fn compile(&self) {}
}
