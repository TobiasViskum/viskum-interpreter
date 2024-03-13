use crate::parser::token::Token;
use colored::Colorize;

const AT_STR: &str = "at: ";
const CONTEXT_RANGE: isize = 5;

enum ErrorType {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct CompileError {
    message: String,
    token: Token,
}

#[derive(Debug)]
pub struct ErrorHandler {
    compile_warnings: Vec<CompileError>,
    compile_errors: Vec<CompileError>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self { compile_warnings: Vec::new(), compile_errors: Vec::new() }
    }

    pub fn report_compile_error(&mut self, message: String, token: Token) {
        self.compile_errors.push(CompileError { message, token });
    }

    pub fn has_error(&self) -> bool {
        !self.compile_errors.is_empty()
    }

    pub fn print_errors(&self, src: &str) {
        eprintln!("{}", "Errors:\n".red().underline().bold());
        for error in &self.compile_errors {
            eprintln!("[line {}] {}", error.token.get_line(), error.message);
            eprintln!("{}", self.get_five_char_context(src, &error.token));
            eprintln!("{}", self.get_arrows_up_to_error_token(&error.token, ErrorType::Error));
        }
    }

    fn get_five_char_context(&self, src: &str, token: &Token) -> String {
        let start = token.get_start() as isize;
        let length = token.get_len() as isize;
        let src_len = src.len() as isize;

        let context_start = (start - CONTEXT_RANGE).max(0) as usize;
        let context_end = (start + length + CONTEXT_RANGE).min(src_len) as usize;

        let src_chars = src.chars().collect::<Vec<_>>();

        let mut first_five = String::new();
        let mut token_string = String::new();
        let mut last_five = String::new();

        for char in &src_chars[context_start..token.get_start()] {
            first_five.push(*char);
        }
        for char in &src_chars[token.get_start()..token.get_start() + token.get_len()] {
            token_string.push(*char);
        }
        for char in &src_chars[token.get_start() + token.get_len()..context_end] {
            last_five.push(*char);
        }

        format!("{}{}{}{}", AT_STR, first_five, token_string, last_five)
    }

    fn get_arrows_up_to_error_token(&self, token: &Token, error_type: ErrorType) -> String {
        let start = token.get_start() as isize;

        let context_start = (start - CONTEXT_RANGE).max(0);

        let prefix_spaces = ((start - context_start).min(CONTEXT_RANGE) as usize) + AT_STR.len();

        let mut arrows = String::new();
        for _ in 0..prefix_spaces {
            arrows.push(' ');
        }

        for _ in 0..token.get_len() {
            arrows.push('^');
        }

        format!("{}", arrows.red().bold())
    }
}
