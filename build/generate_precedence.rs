use std::fs::File;
use std::io::{ self, Write };

use crate::build_info::PRECEDENCE;

pub fn generate_precedence() -> io::Result<()> {
    let out_file = "src/parser/precedence.rs";
    let mut file = File::create(out_file)?;

    writeln!(file, "#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]")?;
    writeln!(file, "pub enum Precedence {{")?;
    for i in 0..PRECEDENCE.len() {
        let precedence = PRECEDENCE[i];
        writeln!(file, "    {} = {},", precedence, i)?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl From<usize> for Precedence {{")?;
    writeln!(file, "    fn from(value: usize) -> Self {{")?;
    writeln!(file, "        match value {{")?;
    for i in 0..PRECEDENCE.len() {
        let precedence = PRECEDENCE[i];
        writeln!(file, "            {} => Precedence::{},", i, precedence)?;
    }
    writeln!(file, "            _ => panic!(\"Invalid precedence value: {{}}\", value),")?;

    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    writeln!(file, "impl From<Precedence> for usize {{")?;
    writeln!(file, "    fn from(value: Precedence) -> Self {{")?;
    writeln!(file, "        match value {{")?;
    for i in 0..PRECEDENCE.len() {
        let precedence = PRECEDENCE[i];
        writeln!(file, "            Precedence::{} => {},", precedence, i)?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    writeln!(file, "impl Precedence {{")?;
    writeln!(file, "    pub fn get_next(self) -> Self {{")?;
    writeln!(file, "        if self == Precedence::{} {{", PRECEDENCE[PRECEDENCE.len() - 1])?;
    writeln!(file, "            Precedence::{}", PRECEDENCE[0])?;
    writeln!(file, "        }} else {{")?;
    writeln!(file, "            Precedence::from((self as usize) + 1)")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "    pub fn get_previous(self) -> Self {{")?;
    writeln!(file, "        if self == Precedence::{} {{", PRECEDENCE[0])?;
    writeln!(file, "            Precedence::{}", PRECEDENCE[PRECEDENCE.len() - 1])?;
    writeln!(file, "        }} else {{")?;
    writeln!(file, "            Precedence::from((self as usize) - 1)")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}
