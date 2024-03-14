pub const TOKEN_TYPES_AND_PARSE_RULES: [&str; 19] = [
    // Single-character tokens
    "LeftParen              = { grouping,      None,   PrecNone       }",
    "RightParen             = { None,          None,   PrecNone       }",

    // Arithmetic operators
    "Minus                  = { unary,         binary, PrecTerm       }",
    "Plus                   = { None,          binary, PrecTerm       }",
    "Slash                  = { None,          binary, PrecFactor     }",
    "Star                   = { None,          binary, PrecFactor     }",
    // "Power                  = { None,          binary, PrecFactor     }",

    // Literals
    "Number                 = { number,        None,   PrecNone       }",
    "Identifier             = { variable,      None,   PrecNone       }",

    // Types
    "True                   = { literal,       None,   PrecNone       }",
    "False                  = { literal,       None,   PrecNone       }",
    "Int32                  = { skip,       None,   PrecNone       }",

    // Single-character tokens
    "Semicolon              = { None,          None,   PrecNone       }",
    "Bang                   = { unary,         None,   PrecNone       }",
    "Assign                 = { None,          None,   PrecNone       }",

    // Double-character tokens
    "Define                 = { None,          None,   PrecNone       }",

    // Keywords
    "Mutable                = { skip,          None,   PrecNone       }",
    "Function               = { skip,          None,   PrecNone       }",

    "Error                  = { error,         None,   PrecNone       }",
    "EOF                    = { None,          None,   PrecNone       }",
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
