pub const TOKEN_TYPES_AND_PARSE_RULES: [&str; 39] = [
    // Single-character tokens
    "LeftParen          = { grouping,                           None,                                       PrecNone        }",
    "RightParen         = { None,                               None,                                       PrecNone        }",
    "LeftCurlyBrace     = { None,                               None,                                       PrecNone        }",
    "RightCurlyBrace    = { None,                               None,                                       PrecNone        }",
    "LeftSquareBracket  = { None,                               None,                                       PrecNone        }",
    "RightSquareBracket = { None,                               None,                                       PrecNone        }",
    "DoubleQuote        = { string,                             None,                                       PrecNone        }",
    "Semicolon          = { None,                               None,                                       PrecNone        }",
    "Bang               = { unary,                              None,                                       PrecNone        }",
    "Assign             = { None,                               None,                                       PrecNone        }",
    "Comma              = { None,                               None,                                       PrecNone        }",

    // Arithmetic operators
    "Minus              = { unary,                              binary,                                     PrecTerm        }",
    "Plus               = { None,                               binary,                                     PrecTerm        }",
    "Slash              = { None,                               binary,                                     PrecFactor      }",
    "Star               = { None,                               binary,                                     PrecFactor      }",
    // "Power                  = { None,          binary, PrecFactor     }",

    // Equality
    "EqualEqual         = { None,                               binary,                                     PrecEquality    }",
    "BangEqual          = { None,                               binary,                                     PrecEquality    }",

    // Comparison
    "Greater            = { None,                               binary,                                     PrecComparison  }",
    "GreaterEqual       = { None,                               binary,                                     PrecComparison  }",
    "Less               = { None,                               binary,                                     PrecComparison  }",
    "LessEqual          = { None,                               binary,                                     PrecComparison  }",

    // Literals
    "Number             = { number,                             None,                                       PrecNone        }",
    "Identifier         = { identifier,                         None,                                       PrecNone        }",
    "String             = { None,                               None,                                       PrecNone        }",

    // Types
    "True               = { literal,                            None,                                       PrecNone        }",
    "False              = { literal,                            None,                                       PrecNone        }",

    // Double-character tokens
    "Define             = { None,                               None,                                       PrecNone        }",

    // Keywords
    "Mutable            = { None,                               None,                                       PrecNone        }",
    "Function           = { None,                               None,                                       PrecNone        }",
    "Typing             = { None,                               None,                                       PrecNone        }",
    "Print              = { None,                               None,                                       PrecNone        }",
    "Return             = { None,                               None,                                       PrecNone        }",
    "If                 = { None,                               None,                                       PrecNone        }",
    "Else               = { None,                               None,                                       PrecNone        }",
    "Break              = { None,                               None,                                       PrecNone        }",
    "Continue           = { None,                               None,                                       PrecNone        }",
    "Loop               = { None,                               None,                                       PrecNone        }",

    "Error              = { error,                              None,                                       PrecNone        }",
    "EOF                = { None,                               None,                                       PrecNone        }",
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

/*
pub const TOKEN_TYPES_AND_PARSE_RULES: [&str; 35] = [
    // Single-character tokens
    "LeftParen          = { grouping(MethodGrouping),           None,                                       PrecNone        }",
    "RightParen         = { None,                               None,                                       PrecNone        }",
    "LeftCurlyBrace     = { None,                               None,                                       PrecNone        }",
    "RightCurlyBrace    = { None,                               None,                                       PrecNone        }",
    "LeftSquareBracket  = { None,                               None,                                       PrecNone        }",
    "RightSquareBracket = { None,                               None,                                       PrecNone        }",
    "Semicolon          = { None,                               None,                                       PrecNone        }",
    "Bang               = { unary(MethodUnary(UnaryOp::Neg)),   None,                                       PrecNone        }",
    "Assign             = { None,                               None,                                       PrecNone        }",
    "Comma              = { None,                               None,                                       PrecNone        }",

    // Arithmetic operators
    "Minus              = { unary(MethodUnary(UnaryOp::Neg)),   binary(MethodBinary(BinaryOp::Sub)),        PrecTerm        }",
    "Plus               = { None,                               binary(MethodBinary(BinaryOp::Add)),        PrecTerm        }",
    "Slash              = { None,                               binary(MethodBinary(BinaryOp::Div)),        PrecFactor      }",
    "Star               = { None,                               binary(MethodBinary(BinaryOp::Mul)),        PrecFactor      }",
    // "Power                  = { None,          binary, PrecFactor     }",

    // Equality
    "EqualEqual         = { None,                               binary(MethodBinary(BinaryOp::Sub)),        PrecEquality    }",
    "BangEqual          = { None,                               binary(MethodBinary(BinaryOp::Sub)),        PrecEquality    }",

    // Comparison
    "Greater            = { None,                               binary(MethodBinary(BinaryOp::Sub)),        PrecComparison  }",
    "GreaterEqual       = { None,                               binary(MethodBinary(BinaryOp::Sub)),        PrecComparison  }",
    "Less               = { None,                               binary(MethodBinary(BinaryOp::Sub)),        PrecComparison  }",
    "LessEqual          = { None,                               binary(MethodBinary(BinaryOp::Sub)),        PrecComparison  }",

    // Literals
    "Number             = { number(MethodNumber),               None,                                       PrecNone        }",
    "Identifier         = { identifier(MethodIdentifier),       None,                                       PrecNone        }",

    // Types
    "True               = { literal(MethodLiteral),             None,                                       PrecNone        }",
    "False              = { literal(MethodLiteral),             None,                                       PrecNone        }",
    "Int32              = { None,                               None,                                       PrecNone        }",
    "Bool               = { None,                               None,                                       PrecNone        }",

    // Double-character tokens
    "Define             = { None,                               None,                                       PrecNone        }",

    // Keywords
    "Mutable            = { None,                               None,                                       PrecNone        }",
    "Function           = { None,                               None,                                       PrecNone        }",
    "Typing             = { None,                               None,                                       PrecNone        }",
    "Print              = { None,                               None,                                       PrecNone        }",
    "Return             = { None,                               None,                                       PrecNone        }",
    "If                 = { None,                               None,                                       PrecNone        }",

    "Error              = { error(MethodError),                 None,                                       PrecNone        }",
    "EOF                = { None,                               None,                                       PrecNone        }",
];
*/
