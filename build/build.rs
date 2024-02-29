use std::io;
mod generate_precedence;
mod generate_rules_store;
mod build_info;

fn main() -> io::Result<()> {
    generate_precedence::generate_precedence()?;
    generate_rules_store::generate_rules_store()?;
    Ok(())
}
