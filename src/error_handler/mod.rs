use crate::parser::token::{ Token, TokenMetadata };
use colored::Colorize;

// const AT_STR: &str = "at: ";
const CONTEXT_RANGE: isize = 5;

enum ErrorType {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct CompileError {
    message: String,
    error_metadata: Vec<TokenMetadata>,
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

    pub fn report_compile_error(&mut self, message: String, error_metadata: Vec<TokenMetadata>) {
        self.compile_errors.push(CompileError { message, error_metadata });
    }

    pub fn has_error(&self) -> bool {
        !self.compile_errors.is_empty()
    }

    pub fn print_errors(&self, src: &str) {
        eprintln!("{}", "Errors:\n".red().underline().bold());
        for error in &self.compile_errors {
            let error_metadata = &error.error_metadata;
            if error.error_metadata.len() == 1 {
                let metadata = &error.error_metadata[0];
                eprintln!("[line {}] {}", metadata.get_line(), error.message);
                eprintln!("{}", self.get_five_char_context(src, metadata));
                eprintln!("{}", self.get_arrows_up_to_error_token(metadata, ErrorType::Error));
            } else {
                let first = error_metadata.last().unwrap();
                let last = error_metadata.first().unwrap();

                let combined_metadata = TokenMetadata::new(
                    first.get_start(),
                    last.get_start() - first.get_start() + last.get_len(),
                    last.get_line()
                );

                eprintln!("[line {}] {}", combined_metadata.get_line(), error.message);
                eprintln!("{}", self.get_five_char_context(src, &combined_metadata));
                eprintln!(
                    "{}",
                    self.get_arrows_up_to_error_token(&combined_metadata, ErrorType::Error)
                );
            }
        }
    }

    fn get_five_char_context(&self, src: &str, token: &TokenMetadata) -> String {
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

        format!("{}{}{}", first_five, token_string, last_five)
    }

    fn get_arrows_up_to_error_token(&self, token: &TokenMetadata, error_type: ErrorType) -> String {
        let start = token.get_start() as isize;

        let context_start = (start - CONTEXT_RANGE).max(0);

        let prefix_spaces = (start - context_start).min(CONTEXT_RANGE) as usize;

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
