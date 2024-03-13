pub fn is_digit(c: Option<char>) -> bool {
    if let Some(c) = c { c >= '0' && c <= '9' } else { false }
}

pub fn is_alphabetic(c: Option<char>) -> bool {
    if let Some(c) = c { c.is_alphabetic() || c == '_' } else { false }
}
