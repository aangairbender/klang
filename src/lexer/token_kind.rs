use super::literal_kind::LiteralKind;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "-"
    Minus,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "="
    Eq,

    Identifier,

    // literals
    Literal { kind: LiteralKind },

    // keywords
    Let, 

    // other
    Whitespace,

    Eof,

    Unknown,
}