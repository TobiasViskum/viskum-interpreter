// pub const OPCODES: [&str; 1] = ["OpReturn               = 0  | simple_instruction"];

pub const PARSE_RULES: [&str; 6] = [
    // Single-character tokens
    "LeftParen              = { grouping,      None,   PrecNone       }",
    "RightParen             = { None,          None,   PrecNone       }",

    // Arithmetic operators
    "Minus                  = { unary,         binary, PrecTerm       }",
    "Plus                   = { None,          binary, PrecTerm       }",
    "Slash                  = { None,          binary, PrecFactor     }",
    "Star                   = { None,          binary, PrecFactor     }",
    // "Power                  = { None,          binary, PrecFactor     }",
];

pub const PRECEDENCE: [&str; 11] = [
    "PrecNone",
    "PrecAssignment",
    "PrecOr",
    "PrecAnd",
    "PrecEquality",
    "PrecComparison",
    "PrecTerm",
    "PrecFactor",
    "PrecUnary",
    "PrecCall",
    "PrecPrimary",
];
