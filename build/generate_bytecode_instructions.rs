use std::{ fs::File, io::{ Result, Write } };
use super::build_info::{ INSTRUCTION_SRC, BYTECODE_INSTRUCTIONS };

fn write_instruction_enums(file: &mut File, prefix: &str) -> Result<()> {
    let vm_instr_match = vec!["BOTH", prefix];

    writeln!(file, "#[derive(Debug, Clone)]")?;
    writeln!(file, "pub enum {}InstructionSrc {{", prefix)?;
    for i in 0..INSTRUCTION_SRC.len() {
        let instruction_src = INSTRUCTION_SRC[i];
        let (lhs, rhs) = instruction_src.split_once(" ").unwrap();
        let instr_match = lhs.trim();

        if vm_instr_match.contains(&instr_match) {
            writeln!(file, "    {},", rhs.trim())?;
        }
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {}InstructionSrc {{", prefix)?;

    writeln!(file, "    pub fn dissassemble(&self) -> String {{")?;
    writeln!(file, "        match self {{")?;

    for i in 0..INSTRUCTION_SRC.len() {
        let instruction_src = INSTRUCTION_SRC[i];
        let (lhs, rhs) = instruction_src.split_once(" ").unwrap();
        let instr_match = lhs.trim();

        if vm_instr_match.contains(&instr_match) {
            let (enum_child_name, enum_child_arg) = rhs.split_once("(").unwrap();
            let enum_child_arg = enum_child_arg.replace(")", "");

            if enum_child_arg == "InstructionRegister" {
                writeln!(
                    file,
                    "            Self::{}(value) => {{ format!(\"{{}}\", value.dissassemble()) }},",
                    enum_child_name.trim()
                )?;
            } else {
                writeln!(
                    file,
                    "            Self::{}(value) => {{ format!(\"{{}}\", value.to_string()) }},",
                    enum_child_name.trim()
                )?;
            }
        }
    }

    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    writeln!(file, "#[derive(Debug, Clone)]")?;
    writeln!(file, "pub enum {}Instruction {{", prefix)?;
    for i in 0..BYTECODE_INSTRUCTIONS.len() {
        let bytecode_instruction = BYTECODE_INSTRUCTIONS[i].replace("}", "");

        let split = bytecode_instruction.split("{").collect::<Vec<_>>();

        if !split[0].contains(' ') {
            writeln!(file, "    {},", split[0].trim())?;
            continue;
        }

        let (left_lhs, left_rhs) = split[0].split_once(" ").unwrap();

        let op_type = left_lhs.trim();
        let instr_name = left_rhs.trim();

        let instr_name = if instr_name.len() == 0 { op_type } else { instr_name };

        if split.len() == 1 {
            writeln!(file, "    {},", instr_name)?;
        } else {
            let split_right = split[1].split(",").collect::<Vec<_>>();
            writeln!(file, "    {} {{", instr_name)?;
            for arg in split_right {
                let arg = arg.replace(": T", format!(": {}InstructionSrc", prefix).as_str());
                writeln!(file, "        {},", arg.trim())?;
            }
            writeln!(file, "    }},")?;
        }
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {}Instruction {{", prefix)?;
    writeln!(file, "    pub fn dissassemble(&self) -> String {{")?;
    writeln!(file, "        match self {{")?;

    for i in 0..BYTECODE_INSTRUCTIONS.len() {
        let bytecode_instruction = BYTECODE_INSTRUCTIONS[i].replace("}", "");

        let split = bytecode_instruction.split("{").collect::<Vec<_>>();

        if !split[0].contains(' ') || split.len() == 1 {
            let instr_name = split[0].trim();

            writeln!(
                file,
                "           Self::{} => {{ \"{}\".to_string() }}",
                instr_name,
                instr_name.to_uppercase()
            )?;
            continue;
        }

        let (left_lhs, left_rhs) = split[0].split_once(" ").unwrap();

        let instr_name = if left_rhs.trim().len() == 0 { left_lhs.trim() } else { left_rhs.trim() };

        write!(file, "\n            Self::{} {{ ", instr_name)?;
        let split_right = split[1].split(",").collect::<Vec<_>>();
        for i in 0..split_right.len() {
            let arg = split_right[i].trim();
            let split_arg = arg.split(":").collect::<Vec<_>>();
            if i == split_right.len() - 1 {
                write!(file, "{}", split_arg[0].trim())?;
            } else {
                write!(file, "{}, ", split_arg[0].trim())?;
            }
        }
        write!(file, " }} => {{")?;
        write!(file, "\n                format!(\"")?;
        write!(file, "{}", instr_name.to_uppercase())?;
        for _ in 0..split_right.len() {
            write!(file, " {{}}")?;
        }
        write!(file, "\"")?;
        for i in 0..split_right.len() {
            let arg = split_right[i].trim();
            let split_arg = arg.split(":").collect::<Vec<_>>();

            write!(file, ", {}{}", split_arg[0].trim(), ".dissassemble()")?;
        }
        write!(file, ")")?;
        write!(file, "\n            }}")?;
    }

    writeln!(file, "\n        }}")?;
    writeln!(file, "    }}\n")?;
    let mut op_categories: Vec<String> = Vec::new();
    for i in 0..BYTECODE_INSTRUCTIONS.len() {
        let bytecode_instruction = BYTECODE_INSTRUCTIONS[i].replace("}", "");

        let split = bytecode_instruction.split("{").collect::<Vec<_>>();

        if !split[0].contains(' ') || split.len() == 1 {
            continue;
        }

        let (left_lhs, left_rhs) = split[0].split_once(" ").unwrap();

        if left_rhs.trim().len() != 0 {
            let op_category = left_lhs.trim().to_string().to_lowercase();
            if !op_categories.contains(&op_category) {
                op_categories.push(op_category);
            }
        }
    }

    writeln!(file, "}}\n")?;

    Ok(())
}

pub fn generate_bytecode_instructions() -> Result<()> {
    let vm_instr_out = "src/vm/instructions/mod.rs";

    let mut vm_instr_file = File::create(vm_instr_out)?;

    writeln!(vm_instr_file, "mod helper_methods;")?;
    writeln!(vm_instr_file, "pub mod helper_structs;")?;

    writeln!(vm_instr_file, "use crate::value::Value;")?;
    writeln!(vm_instr_file, "use self::helper_structs::InstructionRegister;\n")?;

    write_instruction_enums(&mut vm_instr_file, "VM")?;
    write_instruction_enums(&mut vm_instr_file, "IR")?;

    Ok(())
}
