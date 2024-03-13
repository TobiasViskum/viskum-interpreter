use std::{ fs::File, io::{ self, Write } };

use crate::build_info::TOKEN_TYPES_AND_PARSE_RULES;

pub fn generate_rules_store() -> io::Result<()> {
    let out_file = "src/parser/parse_rule/rules_store.rs";
    let out_token_file = "src/parser/token/token_type.rs";

    let mut file = File::create(out_file)?;
    let mut token_file = File::create(out_token_file)?;
    writeln!(token_file, "#[derive(Debug, PartialEq, Eq, Clone, Copy)]")?;

    writeln!(file, "extern crate lazy_static;")?;
    writeln!(file, "use lazy_static::lazy_static;")?;
    writeln!(file, "use crate::parser::precedence::Precedence;")?;
    writeln!(file, "use super::ParseRule;")?;
    writeln!(file, "lazy_static! {{")?;
    writeln!(file, "    pub static ref PARSE_RULES: Vec<ParseRule> = {{")?;
    writeln!(file, "        let mut parse_rules_vec = Vec::new();")?;
    for parse_rule in TOKEN_TYPES_AND_PARSE_RULES {
        let split = parse_rule.split("=").collect::<Vec<&str>>();

        let token_name = format!("Token{}", split[0].trim());

        let second = split[1].replace("{", "").replace("}", "").trim().to_string();
        let args = second.split(",").collect::<Vec<&str>>();

        let arg1 = args[0].trim();
        let prefix = if arg1 == "None" {
            "None".to_string()
        } else {
            format!("Some(|c| c.{}())", arg1)
        };

        let arg2 = args[1].trim();
        let infix = if arg2 == "None" {
            "None".to_string()
        } else {
            format!("Some(|c| c.{}())", arg2)
        };

        let precedence = format!("Precedence::{}", args[2].trim());

        writeln!(file, "        parse_rules_vec.push(ParseRule {{")?;
        writeln!(file, "            prefix: {},", prefix)?;
        writeln!(file, "            infix: {},", infix)?;
        writeln!(file, "            precedence: {},", precedence)?;
        writeln!(file, "        }});")?;
    }
    writeln!(file, "        parse_rules_vec")?;
    writeln!(file, "    }};")?;
    writeln!(file, "}}")?;

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

/*

extern crate lazy_static;

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{ precedence::Precedence, token::token_type::TokenType::{ self, * } };

use super::ParseRule;

lazy_static! {
    pub static ref PARSE_RULES: HashMap<TokenType,  ParseRule<'static>> = {
        let mut map = HashMap::new();
        map.insert(TokenLeftParen, ParseRule {
            prefix: Some(|c| c.grouping()),
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenRightParen, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenLeftBrace, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenRightBrace, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenComma, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenDot, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenMinus, ParseRule {
            prefix: Some(|c| c.unary()),
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecTerm,
        });
        map.insert(TokenPlus, ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecTerm,
        });
        map.insert(TokenSemicolon, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenSlash, ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecFactor,
        });
        map.insert(TokenStar, ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecFactor,
        });
        map.insert(TokenBang, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenBangEqual, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenEqual, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenNumber, ParseRule {
            prefix: Some(|c| c.number()),
            infix: None,
            precedence: Precedence::PrecNone,
        });
        


        map
    };
}


*/
