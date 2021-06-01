use self::{cursor::Cursor, literal_kind::LiteralKind, token::Token, token_kind::TokenKind};

mod cursor;
mod literal_kind;
mod token_kind;
mod token;

pub fn first_token(input: &str) -> Token {
    Cursor::new(input).advance_token()
}

pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if input.is_empty() {
            return None;
        }

        let token = first_token(input);
        input = &input[token.len..];
        Some(token)
    })
}

pub fn is_whitespace(c: char) -> bool {
    matches!(c,
        ' ' | '\t' | '\r' | '\n'
    )
}

pub fn is_id_start(c: char) -> bool {
    c == '_'
        || ('a'..='z').contains(&c)
        || ('A'..='Z').contains(&c)
}

pub fn is_id_continue(c: char) -> bool {
    is_id_start(c)
        || ('0'..='9').contains(&c)
}

pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(start) = chars.next() {
        is_id_start(start) && chars.all(is_id_continue)
    } else {
        false
    }
}

pub fn is_operator(c: char) -> bool {
    matches!(c,
        '+' | '-' | '*' | '/' |
        '!' | '@' | '#' | '$' |
        '%' | '^' | '&' | '~' |
        '?' | ':' | '>' | '<' |
        '|' | '#' | '='
    )
}


impl Cursor<'_> {
    fn advance_token(&mut self) -> Token {
        let first_char = self.advance().unwrap();
        let token_kind = match first_char {
            // whitespace
            c if is_whitespace(c) => self.whitespace(),

            // identifier
            c if is_id_start(c) => self.ident(),

            // numeric literal
            c @ '0'..='9' => {
                let literal_kind = self.number();
                TokenKind::Literal { kind: literal_kind }
            }

            // operator
            c if is_operator(c) => self.operator(),

            // One-symbol tokens
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,

            _ => TokenKind::Unknown,
        };
        Token::new(token_kind, self.len_consumed())
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn ident(&mut self) -> TokenKind {
        self.eat_while(is_id_continue);
        TokenKind::Ident
    }

    fn number(&mut self) -> LiteralKind {
        self.eat_decimal_digits();
        LiteralKind::Int
    }

    fn operator(&mut self) -> TokenKind {
        self.eat_while(is_operator);
        TokenKind::Operator
    }


    fn eat_decimal_digits(&mut self) {
        loop {
            if matches!(self.first(), '_' | '0'..='9') {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn eat_identifier(&mut self) {
        if !is_id_start(self.first()) {
            return;
        }
        self.advance();

        self.eat_while(is_id_continue);
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.advance();
        }
    }
}