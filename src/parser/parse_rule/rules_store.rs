extern crate lazy_static;
use lazy_static::lazy_static;
use crate::parser::{RuleArg, precedence::Precedence};
use super::ParseRule;
lazy_static! {
    pub static ref PARSE_RULES: Vec<ParseRule> = {
        let mut parse_rules_vec = Vec::with_capacity(28);
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.grouping(arg))),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.block(arg))),
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
            prefix: (Some(|c, arg| c.unary(arg))),
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
            prefix: (Some(|c, arg| c.unary(arg))),
            infix: (Some(|c, arg| c.binary(arg))),
            precedence: Precedence::PrecTerm,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (Some(|c, arg| c.binary(arg))),
            precedence: Precedence::PrecTerm,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (Some(|c, arg| c.binary(arg))),
            precedence: Precedence::PrecFactor,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (None),
            infix: (Some(|c, arg| c.binary(arg))),
            precedence: Precedence::PrecFactor,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.number(arg))),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.identifier(arg))),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.literal(arg))),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.literal(arg))),
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
            prefix: (None),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.mut_var_def(arg))),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.function(arg))),
            infix: (None),
            precedence: Precedence::PrecNone,
        });
        parse_rules_vec.push(ParseRule {
            prefix: (Some(|c, arg| c.typing(arg))),
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
            prefix: (Some(|c, arg| c.error(arg))),
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
