use ahash::AHashMap;

use crate::vm::instructions::{ InstrJmpType, Instruction };

use super::{ OptimizedInstruction, Param };

pub fn optimize_instructions(instructions: &Vec<Instruction>) -> Vec<OptimizedInstruction> {
    let mut jmp_instrs_ids = vec![];

    let mut optimized_instructions = vec![];

    for instruction in instructions.iter() {
        if let Some(jmp_instr_type) = instruction.get_jmp_instr_type() {
            jmp_instrs_ids.push((optimized_instructions.len(), jmp_instr_type));
        }

        let optimized_instruction = OptimizedInstruction::from(instruction);
        optimized_instructions.push(optimized_instruction);
    }

    for (jmp_instr_id, instr_jmp_type) in jmp_instrs_ids.iter() {
        macro_rules! overwrite_param {
            ($param:ident) => {
                {
                    let target_instr = unsafe {
                        optimized_instructions[*jmp_instr_id].$param.temp_jmp_pos
                    };

                    optimized_instructions.get_mut(*jmp_instr_id).unwrap().$param = Param::jmp(
                        &optimized_instructions[target_instr] as *const OptimizedInstruction
                    );
                }
            };
        }

        match instr_jmp_type {
            InstrJmpType::Goto => overwrite_param!(param1),
            InstrJmpType::JmpCmp => {
                overwrite_param!(param1);
                overwrite_param!(param2);
            }
        }
    }

    optimized_instructions
}
