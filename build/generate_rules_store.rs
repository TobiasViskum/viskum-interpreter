use std::{ fs::File, io::{ self, Write } };

use crate::build_info::TOKEN_TYPES_AND_PARSE_RULES;

pub fn format_enum_name(s: &str) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let mut uppercase_next = true;
    let mut enum_name = String::new();
    for char in &chars {
        if uppercase_next {
            enum_name.push(char.to_uppercase().to_string().chars().next().unwrap());
            // let mut char = char;
            // (*char).to_uppercase();
            // enum_name.push(*char);
            uppercase_next = false;
        } else {
            if char == &'_' {
                uppercase_next = true;
            } else {
                enum_name.push(*char);
            }
        }
    }

    enum_name
}

pub fn generate_rules_store() -> io::Result<()> {
    let out_file = "src/parser/parse_rule/rules_store.rs";
    let out_token_file = "src/parser/token/token_type/mod.rs";

    let mut file = File::create(out_file)?;

    writeln!(file, "extern crate lazy_static;")?;
    writeln!(file, "use lazy_static::lazy_static;")?;
    writeln!(file, "use crate::parser::precedence::Precedence;")?;
    writeln!(file, "use super::ParseRule;")?;

    writeln!(file, "lazy_static! {{")?;
    writeln!(file, "    pub static ref PARSE_RULES: Vec<ParseRule> = {{")?;
    writeln!(
        file,
        "        let mut parse_rules_vec = Vec::with_capacity({});",
        TOKEN_TYPES_AND_PARSE_RULES.len()
    )?;
    for parse_rule in TOKEN_TYPES_AND_PARSE_RULES {
        let split = parse_rule.split("=").collect::<Vec<&str>>();

        let second = split[1].replace("{", "").replace("}", "").trim().to_string();
        let args = second.split(",").collect::<Vec<&str>>();

        let arg1 = args[0].trim();

        let (prefix, _prefix_enum_name) = if arg1 == "None" {
            ("None".to_string(), None)
        } else {
            (
                format!("Some(|c| c.{}())", arg1),
                Some(format!("Some(ParsingRuleMethod::{})", format_enum_name(arg1))),
            )
        };

        let arg2 = args[1].trim();
        let (infix, _infix_enum_name) = if arg2 == "None" {
            ("None".to_string(), None)
        } else {
            (
                format!("Some(|c| c.{}())", arg2),
                Some(format!("Some(ParsingRuleMethod::{})", format_enum_name(arg2))),
            )
        };

        let precedence = format!("Precedence::{}", args[2].trim());

        writeln!(file, "        parse_rules_vec.push(ParseRule {{")?;
        writeln!(file, "            prefix: ({}),", prefix)?;
        writeln!(file, "            infix: ({}),", infix)?;

        writeln!(file, "            precedence: {},", precedence)?;
        writeln!(file, "        }});")?;
    }
    writeln!(file, "        parse_rules_vec")?;
    writeln!(file, "    }};")?;
    writeln!(file, "}}")?;

    // Token file
    let mut token_file = File::create(out_token_file)?;

    writeln!(token_file, "mod helper_methods;")?;

    writeln!(token_file, "#[derive(Debug, PartialEq, Eq, Clone, Copy)]")?;
    writeln!(token_file, "pub enum TokenType {{")?;
    for parse_rule in TOKEN_TYPES_AND_PARSE_RULES {
        let split = parse_rule.split("=").collect::<Vec<&str>>();
        let token_name = format!("Token{}", split[0].trim());
        writeln!(token_file, "    {},", token_name)?;
    }
    writeln!(token_file, "}}")?;

    writeln!(token_file, "impl From<TokenType> for usize {{")?;
    writeln!(token_file, "    fn from(value: TokenType) -> Self {{")?;
    writeln!(token_file, "        match value {{")?;
    for i in 0..TOKEN_TYPES_AND_PARSE_RULES.len() {
        let token_and_parse_rule = TOKEN_TYPES_AND_PARSE_RULES[i];
        let split = token_and_parse_rule.split("=").collect::<Vec<&str>>();
        let token_name = format!("Token{}", split[0].trim());
        writeln!(token_file, "            TokenType::{} => {},", token_name, i)?;
    }
    writeln!(token_file, "        }}")?;
    writeln!(token_file, "    }}")?;
    writeln!(token_file, "}}")?;

    Ok(())
}
