use self::token_type::TokenType;

pub mod token_type;

#[derive(Debug, Clone, Copy)]
pub struct TokenMetadata {
    start: usize,
    length: usize,
    line: usize,
    token_type: TokenType,
}

impl TokenMetadata {
    pub fn new(start: usize, length: usize, line: usize, token_type: TokenType) -> Self {
        Self {
            start,
            length,
            line,
            token_type,
        }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_len(&self) -> usize {
        self.length
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
    pub fn get_ttype(&self) -> TokenType {
        self.token_type
    }

    pub fn increment_length(&mut self) {
        self.length += 1;
    }
    pub fn decrement_start(&mut self) {
        if self.start > 0 {
            self.start -= 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
    optional_message: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, length: usize, line: usize) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
            optional_message: None,
        }
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        TokenMetadata {
            start: self.start,
            length: self.length,
            line: self.line,
            token_type: self.token_type,
        }
    }

    pub fn new_error(
        token_type: TokenType,
        start: usize,
        length: usize,
        line: usize,
        message: String
    ) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
            optional_message: Some(message),
        }
    }

    pub fn get_ttype(&self) -> &TokenType {
        &self.token_type
    }

    #[profiler::function_tracker]
    pub fn get_lexeme(&self, source: &Vec<char>) -> String {
        let mut lexeme = String::new();
        for i in self.start..self.start + self.length {
            lexeme.push(*source.get(i).unwrap());
        }
        lexeme
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_message(&self) -> Option<&String> {
        self.optional_message.as_ref()
    }
}
