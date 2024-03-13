use crate::parser::{ Parser, precedence::Precedence };

pub use self::rules_store::PARSE_RULES;
mod rules_store;
#[derive(Debug)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Parser)>,
    pub infix: Option<fn(&mut Parser)>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn new(
        prefix: Option<fn(&mut Parser)>,
        infix: Option<fn(&mut Parser)>,
        precedence: Precedence
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }

    pub fn get_prefix(&self) -> Option<fn(&mut Parser)> {
        self.prefix
    }

    pub fn get_infix(&self) -> Option<fn(&mut Parser)> {
        self.infix
    }

    pub fn get_precedence(&self) -> &Precedence {
        &self.precedence
    }
}
