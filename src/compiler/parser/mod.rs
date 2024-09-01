pub mod token;

mod core_methods;
mod helper_methods;
mod lexer;
mod parse_rule;
mod precedence;
mod stmt_methods;
mod expr_methods;
mod parser_macros;

use std::rc::Rc;

use ahash::AHashMap;
use lexer::Lexer;
use parse_rule::ParseRule;
use parser_macros::current;
use token::Token;

use crate::macros::create_tokens_and_parse_rules;

use super::{
    ds::{ value::{ ops::{ BinaryOp, ComparisonOp, UnaryOp }, Value } },
    error_handler::{ CompileError, ErrorHandler },
    ir::ast::{ expr::ExprBuilder, stmt::{ BlockStmt, Stmt }, Ast, AstArena },
    ProgramSymbolTablePhase1,
};

#[derive(Debug, Hash)]
pub struct Lexeme {
    lexeme: Rc<str>,
}

impl Lexeme {
    pub fn new(lexeme: Rc<str>) -> Self {
        Self {
            lexeme,
        }
    }

    pub fn get_lexeme_str(&self) -> &str {
        &self.lexeme
    }

    pub fn take_lexeme_rc(self) -> Rc<str> {
        self.lexeme
    }

    pub fn borrow_ident(&self) -> &Rc<str> {
        &self.lexeme
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.lexeme)
    }

    pub fn parse_number(&self) -> Option<Value> {
        let parse_hireachy = [Self::try_parse_int, Self::try_parse_i64];

        for parse_fn in parse_hireachy {
            match parse_fn(self) {
                Some(val) => {
                    return Some(val);
                }
                None => {
                    continue;
                }
            }
        }

        None
    }

    fn try_parse_int(&self) -> Option<Value> {
        match self.lexeme.parse::<i64>() {
            Ok(v) => Some(Value::Int(v)),
            Err(_) => None,
        }
    }

    fn try_parse_i64(&self) -> Option<Value> {
        // match self.lexeme.parse::<i64>() {
        //     Ok(v) => Some(Value::Int32(v)),
        //     Err(_) => None,
        // }
        None
    }
}

pub type StmtMethodRetType<'b> = Result<Stmt<'b>, CompileError>;
pub type StmtMethodArgs<'b, 'c> = (&'c mut ProgramSymbolTablePhase1, &'b AstArena<'b>);

pub type ExprMethodRetType = Result<(), CompileError>;
pub type ExprMethodArgs<'b, 'c> = (
    &'c mut ExprBuilder<'b>,
    &'c mut ProgramSymbolTablePhase1,
    &'b AstArena<'b>,
);

pub struct Parser<'a> {
    source: &'a Vec<char>,
    panic_mode: bool,
    error_handler: &'a mut ErrorHandler,
    current: usize,
    tokens: Vec<Token>,
    parse_rules: &'static [ParseRule; 48],
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Vec<char>, error_handler: &'a mut ErrorHandler) -> Self {
        let tokens = Lexer::new(source).get_tokens();

        Self {
            tokens,
            source,
            panic_mode: false,
            current: 0,
            error_handler,
            parse_rules: &PARSE_RULES,
        }
    }

    pub fn parse_ast<'b, 'c>(
        &'a mut self,
        program_symbol_table: &'c mut ProgramSymbolTablePhase1,
        ast_arena: &'b AstArena<'b>
    ) -> Ast<'b>
        where 'b: 'a + 'c
    {
        let symbol_table_id = program_symbol_table.new_symbol_table(None, None);

        let mut main_scope = BlockStmt::new(symbol_table_id);

        while !self.is_at_end() {
            match self.statement((program_symbol_table, ast_arena)) {
                Ok(stmt) =>
                    match main_scope.push_stmt(stmt, program_symbol_table) {
                        Ok(_) => {}
                        Err(err) => self.error_handler.report_compile_error(err),
                    }
                Err(err) => {
                    self.report_compile_error(err);
                }
            }

            if self.panic_mode {
                self.synchronize();
            }
        }

        Ast::new(main_scope)
    }
}

create_tokens_and_parse_rules!(
    /* TokenType                = { Prefix,         Infix,          Precedence      } */

    // Braces
    [TokenLeftParen]            = { grouping,       call,           PrecCall        },
    [TokenRightParen]           = { None,           None,           PrecNone        },
    [TokenLeftCurlyBrace]       = { None,           None,           PrecNone        },
    [TokenRightCurlyBrace]      = { None,           None,           PrecNone        },
    [TokenLeftSquareBrace]      = { array,          array_index,    PrecArray       },
    [TokenRightSquareBrace]     = { None,           None,           PrecNone        },

    // Other symbols    
    [TokenDoubleQuote]          = { string,         None,           PrecNone        },
    [TokenSemicolon]            = { None,           None,           PrecNone        },
    [TokenComma]                = { None,           None,           PrecNone        },
    [TokenDot]                  = { None,           None,           PrecNone        },

    // Unary operators  
    [TokenBang]                 = { unary,          None,           PrecNone        },
    [TokenReference]            = { unary,          None,           PrecNone        },
    [TokenMutableReference]     = { unary,          None,           PrecNone        },
    [TokenIncrement]            = { None,           None,           PrecNone        },
    [TokenDecrement]            = { None,           None,           PrecNone        },

    // Binary operators (and some unary)    
    [TokenMinus]                = { unary,          binary,         PrecTerm        },
    [TokenPlus]                 = { None,           binary,         PrecTerm        },
    [TokenSlash]                = { None,           binary,         PrecFactor      },
    [TokenStar]                 = { unary,          binary,         PrecFactor      },

    // Equality and comparisons 
    [TokenEqualEqual]           = { None,           binary,         PrecEquality    }, 
    [TokenBangEqual]            = { None,           binary,         PrecEquality    },
    [TokenGreater]              = { None,           binary,         PrecComparison  },
    [TokenGreaterEqual]         = { None,           binary,         PrecComparison  },
    [TokenLess]                 = { None,           binary,         PrecComparison  },
    [TokenLessEqual]            = { None,           binary,         PrecComparison  },

    // Literals and identifiers 
    [TokenNumber]               = { number,         None,           PrecNone        },
    [TokenIdentifier]           = { identifier,     None,           PrecNone        },
    [TokenString]               = { None,           None,           PrecNone        },
    [TokenTrue]                 = { boolean,        None,           PrecNone        },
    [TokenFalse]                = { boolean,        None,           PrecNone        },

    // Assign and define    
    [TokenAssign]               = { None,           None,           PrecNone        },
    [TokenDefine]               = { None,           None,           PrecNone        },

    // Keywords 
    [TokenMutable]              = { None,           None,           PrecNone        },
    [TokenFunction]             = { None,           None,           PrecNone        },
    [TokenTyping]               = { None,           None,           PrecNone        },
    [TokenReturn]               = { None,           None,           PrecNone        },
    [TokenIf]                   = { None,           None,           PrecNone        },
    [TokenElse]                 = { None,           None,           PrecNone        },
    [TokenBreak]                = { None,           None,           PrecNone        },
    [TokenContinue]             = { None,           None,           PrecNone        },
    [TokenLoop]                 = { None,           None,           PrecNone        },
    [TokenWhile]                = { None,           None,           PrecNone        },
    [TokenEnd]                  = { None,           None,           PrecNone        },
    [TokenDo]                   = { None,           None,           PrecNone        },
    [TokenElif]                 = { None,           None,           PrecNone        },
    [TokenDef]                  = { None,           None,           PrecNone        },

    [TokenError]                = { None,           None,           PrecNone        },
    [TokenEOF]                  = { None,           None,           PrecNone        },
);

impl TokenType {
    pub fn is(&self, other: &TokenType) -> bool {
        self == other
    }

    pub fn parse_binary(&self) -> Result<BinaryOp, ()> {
        match self {
            Self::TokenPlus => Ok(BinaryOp::Add),
            Self::TokenMinus => Ok(BinaryOp::Sub),
            Self::TokenSlash => Ok(BinaryOp::Div),
            Self::TokenStar => Ok(BinaryOp::Mul),
            Self::TokenEqualEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::Eq)),
            Self::TokenBangEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::Ne)),
            Self::TokenGreater => Ok(BinaryOp::ComparisonOp(ComparisonOp::Gt)),
            Self::TokenGreaterEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::Ge)),
            Self::TokenLess => Ok(BinaryOp::ComparisonOp(ComparisonOp::Lt)),
            Self::TokenLessEqual => Ok(BinaryOp::ComparisonOp(ComparisonOp::Le)),
            _ => Err(()),
        }
    }

    pub fn parse_unary(&self) -> Result<UnaryOp, ()> {
        match self {
            Self::TokenBang => Ok(UnaryOp::Not),
            Self::TokenMinus => Ok(UnaryOp::Neg),
            // Self::TokenReference => Ok(UnaryOp::Ref),
            // Self::TokenMutableReference => Ok(UnaryOp::MutRef),
            // Self::TokenStar => Ok(UnaryOp::Deref),
            _ => Err(()),
        }
    }

    pub fn can_terminate_block(&self) -> bool {
        matches!(self, TokenType::TokenEnd | TokenType::TokenElse | TokenType::TokenElif)
    }

    pub fn requires_end_block(&self) -> bool {
        matches!(self, TokenType::TokenIf | TokenType::TokenLoop)
    }
}
