pub mod token;

mod helper_methods;
mod core_methods;
mod precedence;
mod parse_rule;
mod parse_rule_methods;
pub mod ast_generator;
mod lexer;

use crate::{
    ast::{ stmt::{ FunctionStmt, ScopeStmt, Stmt }, Ast },
    error_handler::ErrorHandler,
    operations::{ BinaryOp, UnaryOp },
    parser::{ lexer::Lexer, token::Token },
};

use self::{ precedence::Precedence };

#[derive(Debug, PartialEq)]
pub enum RuleArg {
    None,
    MutVar,
    ParseMethod(ParseMethod),
}

#[derive(Debug, PartialEq)]
pub enum ParseMethod {
    MethodBinary(BinaryOp),
    MethodUnary(UnaryOp),
    MethodGrouping,
    MethodLiteral,
    MethodIdentifier,
    MethodNumber,
    MethodError,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source: &'a Vec<char>,
    /*
    next: Option<Token>,
    current: Option<Token>,
    previous_tokens: Vec<Token>,
    */
    had_error: bool,
    panic_mode: bool,
    error_handler: &'a mut ErrorHandler,
    current: usize,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Vec<char>, error_handler: &'a mut ErrorHandler) -> Self {
        let tokens = Lexer::new(source).get_tokens();

        Self {
            lexer: Lexer::new(source),
            tokens,
            source,
            /*
            next: None,
            current: None,
            previous_tokens: Vec::with_capacity(64), // implement function to clear when: consume_expr_end
            */
            had_error: false,
            panic_mode: false,
            current: 0,
            error_handler,
        }
    }

    pub fn free(&mut self) {
        /*
        self.previous_tokens.clear();
        self.current = None;
        self.next = None;
        */
        self.had_error = false;
        self.panic_mode = false;

        self.lexer.free();
    }

    #[profiler::function_tracker]
    pub fn parse_to_ast(&mut self) -> Ast {
        let mut main_scope = ScopeStmt::new();

        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => {
                    match stmt {
                        Stmt::FunctionStmt(_) => main_scope.forwards_declarations.push(stmt),
                        _ => main_scope.cf_stmts.push(stmt),
                    }
                }
                Err(e) => {
                    // Report by sending object instead of doing this expensive clone
                    self.report_compile_error(e.get_msg(), e.get_error_metadata());
                }
            }

            if self.panic_mode {
                self.synchronize();
            }
        }

        // println!("main: {:#?}", main_scope);

        self.free();

        Ast::new(main_scope)
    }
}
