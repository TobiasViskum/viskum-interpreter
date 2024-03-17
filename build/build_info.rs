pub const TOKEN_TYPES_AND_PARSE_RULES: [&str; 23] = [
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
    "Bool                   = { skip,          None,   PrecNone       }",

    // Single-character tokens
    "Semicolon              = { None,          None,   PrecNone       }",
    "Bang                   = { unary,         None,   PrecNone       }",
    "Assign                 = { None,          None,   PrecNone       }",
    "LeftCurlyBrace         = { None,          None,   PrecNone       }",
    "RightCurlyBrace        = { None,          None,   PrecNone       }",

    // Double-character tokens
    "Define                 = { None,          None,   PrecNone       }",

    // Keywords
    "Mutable                = { skip,          None,   PrecNone       }",
    "Function               = { skip,          None,   PrecNone       }",
    "Print                  = { None,          None,   PrecNone       }",

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
    "BOTH   Register(InstructionRegister)",
    "BOTH   Constant(Value)",

    "IR     VariableRegister(InstructionRegister)",
];

pub const BYTECODE_INSTRUCTIONS: [&str; 12] = [
    "Halt",

    "StartScope",
    "EndScope",

    // "Print  {   reg: usize      }",

    "Load               {   reg: InstructionRegister,       src: T      }",

    "BINARY     Add    {   dest: InstructionRegister,      src1: T,        src2: T     }",
    "BINARY     Sub    {   dest: InstructionRegister,      src1: T,        src2: T     }",
    "BINARY     Mul    {   dest: InstructionRegister,      src1: T,        src2: T     }",
    "BINARY     Div    {   dest: InstructionRegister,      src1: T,        src2: T     }",

    "UNARY      Neg    {   dest: InstructionRegister,      src: T      }",
    "UNARY      Truthy {   dest: InstructionRegister,      src: T      }",

    "DEFINEMENT Define {   dest: InstructionRegister,      src: T      }",
    "ASSIGNMENT Assign {   dest: InstructionRegister,      src: T      }",
];
