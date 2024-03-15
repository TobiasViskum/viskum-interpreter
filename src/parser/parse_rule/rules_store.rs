extern crate lazy_static;
use lazy_static::lazy_static;
use crate::parser::precedence::Precedence;
use super::ParseRule;
lazy_static! {
    pub static ref PARSE_RULES: Vec<ParseRule> = {
        let mut parse_rules_vec = Vec::new();
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.grouping())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.unary())),
            infix: (Some(|c| c.binary())),
            precedence: Precedence::PrecTerm,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (Some(|c| c.binary())),
            precedence: Precedence::PrecTerm,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (Some(|c| c.binary())),
            precedence: Precedence::PrecFactor,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (Some(|c| c.binary())),
            precedence: Precedence::PrecFactor,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.number())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.identifier())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.literal())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.literal())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.skip())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.unary())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.skip())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.skip())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c| c.error())),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec
    };
}
