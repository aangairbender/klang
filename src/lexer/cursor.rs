use std::str::Chars;

pub struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>
}

pub const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    pub fn nth_char(&self, n: usize) -> char {
        self.chars.clone().nth(n).unwrap_or(EOF_CHAR)
    }

    pub fn first(&self) -> char {
        self.nth_char(0)
    }

    pub fn second(&self) -> char {
        self.nth_char(1)
    }

    pub fn is_eof(&self) -> bool {
        self.chars.clone().next().is_none()
    }

    pub fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }
}