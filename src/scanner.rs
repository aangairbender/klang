use std::str::Chars;

use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: Chars<'a>,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source_str: &'a str) -> Self {
        Self {
            source: source_str.chars(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&'a mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
        std::mem::replace(&mut self.tokens, vec![])
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_simple_token(TokenType::LeftParen),
            ')' => self.add_simple_token(TokenType::RightParen),

            ' ' => (),
            '\r' => (),
            '\t' => (),

            '\n' => self.line += 1,

            '"' => self.read_string(),

            _ => panic!("Unexpected character '{}' at line {}", c, self.line)
        }
    }

    fn cur(&self) -> char {
        self.source.
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let res = self.source[self.current];
        self.current += 1;
        res
    }

    fn add_simple_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, "", self.line));
    }

    fn add_lexeme_token(&mut self, token_type: TokenType, lexeme: &'a str) {
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }

    fn cur_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            return Some(self.source[self.current]);
        }
    }

    fn read_string(&mut self) {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            } else if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string at line {}", self.line);
        }

        // the closing '"'
        self.advance();

        let lexeme = unsafe {
            self.source.get_unchecked((self.start+1)..self.current)
        };
        self.add_lexeme_token(TokenType::String, lexeme);
    }
}