use ahash::AHashMap;

use crate::vm::instructions::Instruction;

use super::OptimizedInstruction;

pub fn optimize_instructions(instructions: Vec<Instruction>) -> Vec<OptimizedInstruction> {
    let mut instruction_links = AHashMap::<usize, usize>::new();

    let mut optimized_instructions = vec![];

    for (i, instruction) in instructions.iter().enumerate() {
        let optimized_instruction = OptimizedInstruction::from(instruction);
        optimized_instructions.push(optimized_instruction);
    }

    optimized_instructions
}
