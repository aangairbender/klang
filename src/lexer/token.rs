use super::token_kind::TokenKind;

pub struct Token {
    pub kind: TokenKind,
    pub len: usize
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {}]", self.kind, self.len)
    }
}