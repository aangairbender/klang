use super::literal_kind::LiteralKind;

#[derive(Debug)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,

    Operator,

    // literals
    Literal { kind: LiteralKind },

    Whitespace,

    Ident,

    // keywords
    // empty for now

    Unknown,
}