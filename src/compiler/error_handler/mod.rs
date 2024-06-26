use colored::Colorize;

use super::parser::token::TokenMetadata;

pub trait ErrorTrait {}

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
pub struct ReportedError {
    message: String,
    chars_range: SrcCharsRange,
}

impl ReportedError {
    pub fn new(message: String, chars_range: SrcCharsRange) -> Self {
        Self { message, chars_range }
    }
}

#[derive(Debug)]
pub struct CompileError {
    error_parts: Vec<ReportedError>,
}

impl CompileError {
    pub fn new(reported_error: ReportedError) -> Self {
        Self {
            error_parts: vec![reported_error],
        }
    }

    pub fn new_multiple(reported_errors: Vec<ReportedError>) -> Self {
        Self {
            error_parts: reported_errors,
        }
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

        for (i, error) in self.compile_errors.iter().enumerate() {
            println!("Error {:?}", error);
        }
    }
}
