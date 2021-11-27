use self::{cursor::Cursor, literal_kind::LiteralKind, token::Token, token_kind::TokenKind};

mod cursor;
pub mod literal_kind;
pub mod token_kind;
pub mod token;

pub fn first_token(input: &str) -> Token {
    Cursor::new(input).bump_token()
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
    matches!(c, '_' | 'a'..='z' | 'A'..='Z')
}

pub fn is_id_continue(c: char) -> bool {
    is_id_start(c) || matches!(c, '0'..='9')
}

pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(start) = chars.next() {
        is_id_start(start) && chars.all(is_id_continue)
    } else {
        false
    }
}

impl Cursor<'_> {
    fn bump_token(&mut self) -> Token {
        let first_char = self.bump().unwrap();
        let mut data = None;
        let token_kind = match first_char {
            // keywords
            c if c == 'l' && self.first() == 'e' && self.second() == 't' => {
                self.bump();
                self.bump();
                TokenKind::Let
            },

            // whitespace
            c if is_whitespace(c) => self.whitespace(),

            // identifier
            c if is_id_start(c) => {
                let mut str = String::new();
                str.push(c);
                let t = self.identifier(&mut str);
                data = Some(str);
                t
            },

            // numeric literal
            c @ '0'..='9' => {
                let mut str = String::new();
                str.push(c);
                let literal_kind = self.number(&mut str);
                data = Some(str);
                TokenKind::Literal { kind: literal_kind }
            }

            // One-symbol tokens
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '=' => TokenKind::Eq,

            _ => TokenKind::Unknown,
        };

        match data {
            Some(data) => Token::new_with_data(token_kind, self.len_consumed(), data),
            None => Token::new(token_kind, self.len_consumed())
        }
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn identifier(&mut self, s: &mut String) -> TokenKind {
        self.eat_while_saving(is_id_continue, s);
        TokenKind::Identifier
    }

    fn number(&mut self, s: &mut String) -> LiteralKind {
        self.eat_decimal_digits(s);
        LiteralKind::Number
    }

    fn eat_decimal_digits(&mut self, s: &mut String) -> bool {
        let mut has_digits = false;
        loop {
            let c = self.first();
            match c {
                '_' => { self.bump(); },
                '0'..='9' => {
                    has_digits = true;
                    s.push(c);
                    self.bump();
                }
                _ => break
            }
        }
        has_digits
    }

    fn eat_identifier(&mut self) {
        if !is_id_start(self.first()) {
            return;
        }
        self.bump();

        self.eat_while(is_id_continue);
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    fn eat_while_saving(&mut self, mut predicate: impl FnMut(char) -> bool, s: &mut String) {
        while predicate(self.first()) && !self.is_eof() {
            s.push(self.first());
            self.bump();
        }
    }
}