use crate::parser::{ Parser, precedence::Precedence };

pub use self::rules_store::PARSE_RULES;

use super::RuleArg;
mod rules_store;
#[derive(Debug)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Parser, RuleArg)>,
    pub infix: Option<fn(&mut Parser, RuleArg)>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn get_prefix(&self) -> Option<fn(&mut Parser, RuleArg)> {
        self.prefix
    }

    pub fn get_infix(&self) -> Option<fn(&mut Parser, RuleArg)> {
        self.infix
    }

    pub fn get_precedence(&self) -> &Precedence {
        &self.precedence
    }
}
