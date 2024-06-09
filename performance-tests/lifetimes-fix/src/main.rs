fn main() {
    let instructions = &vec![Instruction::Add {
        src1: Src1::new((1, true), stack_ptr),
        src2: Src1::new((1, false), stack_ptr),
    }];

    let constant_pool = &vec![];

    let vm = VM::new(instructions, constant_pool);
}

fn stack_ptr<'a>(
    (idx, is_relative): (usize, bool),
    _: &'a [Value],
    stack: &'a Stack,
    _: &'a Registers
) -> &'a Value {
    stack.get_val_from_parts(idx)
}

pub struct VM<'a> {
    instructions: &'a [Instruction],
    constant_pool: &'a [Value],
    stack: Stack,
    registers: Registers,
}

impl<'a> VM<'a> {
    pub fn new(instructions: &'a [Instruction], constant_pool: &'a [Value]) -> Self {
        Self {
            instructions,
            constant_pool,
            stack: Stack { stack: Vec::new() },
            registers: Registers { ip: 0 },
        }
    }

    pub fn run(&'a mut self) {
        while self.registers.ip < self.instructions.len() {
            let instr = self.instructions.get(self.registers.ip).unwrap();

            match instr {
                Instruction::Add { src1, src2 } => {
                    let v1 = src1.execute(&self.constant_pool, &self.stack, &self.registers);
                    let v2 = src2.execute(&self.constant_pool, &self.stack, &self.registers);

                    let result = v1.add(v2);

                    // Do something with the result

                    self.stack.push();

                    self.registers.ip += 1;
                }
            }
        }
    }
}

pub enum Instruction {
    Add {
        src1: Src1,
        src2: Src1,
    },
}

pub struct Src1 {
    data: (usize, bool),
    func: for<'a> fn((usize, bool), &'a [Value], &'a Stack, &'a Registers) -> &'a Value,
}

impl Src1 {
    pub fn new(
        data: (usize, bool),
        func: for<'a> fn((usize, bool), &'a [Value], &'a Stack, &'a Registers) -> &'a Value
    ) -> Self {
        Self { data, func }
    }

    pub fn execute<'b>(
        &'b self,
        constants_pool: &'b [Value],
        stack: &'b Stack,
        registers: &'b Registers
    ) -> &'b Value {
        (self.func)(self.data, constants_pool, stack, registers)
    }
}

pub enum Value {
    Int32(i32),
}

impl Value {
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Int32(lhs), Self::Int32(rhs)) => Self::Int32(lhs + rhs),
        }
    }
}

pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    #[inline]
    pub fn get_val_from_parts(&self, idx: usize) -> &Value {
        self.stack.get(idx).unwrap()
    }

    pub fn push(&mut self) {
        println!("Push")
    }
}
pub struct Registers {
    pub ip: usize,
}
