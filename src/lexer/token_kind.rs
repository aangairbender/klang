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

    Identifier,

    // literals
    Literal { kind: LiteralKind },

    Whitespace,

    Eof,

    Unknown,
}