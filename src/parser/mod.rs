pub mod token;

mod helper_methods;
mod core_methods;
mod precedence;
mod parse_rule;
mod parse_rule_methods;
pub mod ast_generator;
mod lexer;

use crate::{ ast::Ast, error_handler::ErrorHandler, parser::{ lexer::Lexer, token::Token } };

use self::{ ast_generator::AstGenerator, precedence::Precedence };

#[derive(Debug, PartialEq)]
pub enum RuleArg {
    None,
    MutVar,
    Precedence(Precedence),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source: &'a Vec<char>,
    next: Option<Token>,
    current: Option<Token>,
    previous_tokens: Vec<Token>,
    had_error: bool,
    panic_mode: bool,
    ast_generator: AstGenerator,
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Vec<char>, error_handler: &'a mut ErrorHandler) -> Self {
        Self {
            lexer: Lexer::new(source),
            source,
            next: None,
            current: None,
            previous_tokens: Vec::with_capacity(64), // implement function to clear when: consume_expr_end
            had_error: false,
            panic_mode: false,
            ast_generator: AstGenerator::new(),
            error_handler,
        }
    }

    pub fn free(&mut self) {
        self.previous_tokens.clear();
        self.current = None;
        self.had_error = false;
        self.panic_mode = false;
        self.ast_generator.free();
        self.lexer.free();
    }

    #[profiler::function_tracker]
    pub fn parse_to_ast(&mut self) -> Ast {
        self.advance();
        self.advance();

        while !self.is_at_end() {
            self.statement();

            if self.panic_mode {
                self.synchronize();
            }
        }

        let ast = self.ast_generator.get_ast();

        //println!("Ast from generator: {:#?}", ast);
        //
        //println!("Ast from parser: {:#?}", self.ast.as_ref().unwrap());

        self.free();
        ast
    }
}
