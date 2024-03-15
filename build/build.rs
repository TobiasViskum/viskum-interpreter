use std::io;
mod generate_precedence;
mod generate_rules_store;
mod build_info;
mod generate_bytecode_instructions;

fn main() -> io::Result<()> {
    generate_precedence::generate_precedence()?;
    generate_rules_store::generate_rules_store()?;
    generate_bytecode_instructions::generate_bytecode_instructions()?;
    Ok(())
}
