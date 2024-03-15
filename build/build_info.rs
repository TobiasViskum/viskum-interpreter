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
    "Identifier             = { identifier,    None,   PrecNone       }",

    // Types
    "True                   = { literal,       None,   PrecNone       }",
    "False                  = { literal,       None,   PrecNone       }",
    "Int32                  = { skip,          None,   PrecNone       }",

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

pub const INSTRUCTION_SRC: [&str; 3] = [
    "BOTH   Register(usize)",
    "BOTH   Constant(Value)",

    "IR     VariableRegister(usize)",
];

pub const BYTECODE_INSTRUCTIONS: [&str; 10] = [
    "Halt",

    "Load   {   reg: usize,       src: T      }",

    "BINARY     Add    {   dest: usize,      src1: T,        src2: T     }",
    "BINARY     Sub    {   dest: usize,      src1: T,        src2: T     }",
    "BINARY     Mul    {   dest: usize,      src1: T,        src2: T     }",
    "BINARY     Div    {   dest: usize,      src1: T,        src2: T     }",

    "UNARY      Neg    {   dest: usize,      src: T      }",
    "UNARY      Truthy {   dest: usize,      src: T      }",

    "DEFINEMENT Define {   dest: usize,      src: T      }",
    "ASSIGNMENT Assign {   dest: usize,      src: T      }",
];
