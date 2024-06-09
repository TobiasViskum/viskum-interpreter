use crate::parser::token::TokenMetadata;
use colored::Colorize;

// const AT_STR: &str = "at: ";
const CONTEXT_RANGE: isize = 5;

enum ErrorType {
    Error,
    Warning,
}

#[derive(Debug)]
pub enum InternalErrorCode {
    Compile(usize),
    Runtime(usize),
}

#[derive(Debug)]
pub struct InternalError {
    error_code: InternalErrorCode,
    message: String,
}

impl InternalError {
    pub fn new(error_code: InternalErrorCode, message: &str) -> Self {
        Self {
            error_code,
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SrcCharsRange {
    char_info: (usize, usize),
    line_info: (usize, usize),
}

impl Into<TokenMetadata> for SrcCharsRange {
    fn into(self) -> TokenMetadata {
        TokenMetadata::new(self.char_info.0, self.char_info.1 - self.char_info.0, self.line_info.0)
    }
}

impl SrcCharsRange {
    pub fn new(char_info: (usize, usize), line_info: (usize, usize)) -> Self {
        Self {
            char_info,
            line_info,
        }
    }

    pub fn dec_char_by(&mut self, amount: usize) {
        self.char_info.0 -= amount;
    }

    pub fn inc_char_by(&mut self, amount: usize) {
        self.char_info.1 += amount;
    }

    pub fn merge(&mut self, other: &Self) {
        let other_char_info = other.char_info;
        if other_char_info.0 < self.char_info.0 {
            self.char_info.0 = other_char_info.0;
        }
        if other_char_info.1 > self.char_info.1 {
            self.char_info.1 = other_char_info.1;
        }

        let other_line_info = other.line_info;
        if other_line_info.0 < self.line_info.0 {
            self.line_info.0 = other_line_info.0;
        }
        if other_line_info.1 > self.line_info.1 {
            self.line_info.1 = other_line_info.1;
        }
    }
}

#[derive(Debug)]
pub struct CompileError {
    message: String,
    chars_range: SrcCharsRange,
}

impl CompileError {
    pub fn new(message: String, chars_range: SrcCharsRange) -> Self {
        Self {
            message,
            chars_range,
        }
    }

    pub fn get_msg(&self) -> String {
        self.message.clone()
    }

    pub fn get_metadata(&self) -> SrcCharsRange {
        self.chars_range
    }
}

#[derive(Debug)]
pub struct ErrorHandler {
    _compile_warnings: Vec<CompileError>,
    compile_errors: Vec<CompileError>,
    src: String,
}

impl ErrorHandler {
    pub fn new(src: String) -> Self {
        Self { src, _compile_warnings: Vec::new(), compile_errors: Vec::new() }
    }

    pub fn report_compile_error(&mut self, compile_error: CompileError) {
        self.compile_errors.push(compile_error);
    }

    pub fn report_many_compile_errors(&mut self, compile_errors: Vec<CompileError>) {
        for compile_error in compile_errors {
            self.report_compile_error(compile_error);
        }
    }

    pub fn has_error(&self) -> bool {
        !self.compile_errors.is_empty()
    }

    pub fn print_errors(&self) {
        if self.compile_errors.len() > 0 {
            eprintln!("{}", "Errors:\n".red().underline().bold());
        }
        for error in &self.compile_errors {
            println!("Error: {}", error.message);
            // let error_metadata = &error.error_metadata;
            // if error.error_metadata.len() == 1 {
            //     let metadata = &error.error_metadata[0];
            //     eprintln!("[line {}] {}", metadata.get_line(), error.message);
            //     eprintln!("-------> {}", self.get_five_char_context(self.src.as_str(), metadata));
            // eprintln!("{}", self.get_arrows_up_to_error_token(metadata, ErrorType::Error));
            // } else {

            // let first = error_metadata.last().unwrap();
            // let last = error_metadata.first().unwrap();

            // let combined_metadata = TokenMetadata::new(
            //     first.get_start(),
            //     last.get_start() - first.get_start() + last.get_len(),
            //     last.get_line(),
            //     last.get_ttype()
            // );

            // eprintln!("[line {}] {}", combined_metadata.get_line(), error.message);
            // eprintln!(
            //     "-------> {}",
            //     self.get_five_char_context(self.src.as_str(), &combined_metadata)
            // );

            // eprintln!(
            //     "{}",
            //     self.get_arrows_up_to_error_token(&combined_metadata, ErrorType::Error)
            // );
            // }
        }
    }

    fn get_five_char_context(&self, src: &str, token: &TokenMetadata) -> String {
        let start = token.get_start() as isize;
        let length = token.get_len() as isize;

        let context_start = (start - CONTEXT_RANGE).max(0) as usize;
        let context_end = (start + length + CONTEXT_RANGE).min(
            src.chars().count() as isize
        ) as usize;

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

        format!("{}", token_string.on_red())
    }

    fn _get_arrows_up_to_error_token(
        &self,
        token: &TokenMetadata,
        _error_type: ErrorType
    ) -> String {
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
