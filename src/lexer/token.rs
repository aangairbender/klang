use super::token_kind::TokenKind;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
    pub data: Option<String>,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len, data: None }
    }

    pub fn new_with_data(kind: TokenKind, len: usize, data: String) -> Self {
        Self { kind, len, data: Some(data) }
    }
}
