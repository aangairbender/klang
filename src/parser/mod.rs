use crate::lexer::literal_kind::LiteralKind;
use crate::lexer::token::Token;
use crate::ast::*;
use crate::lexer::token_kind::TokenKind;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Stmt {
        self.statement()
    }

    fn match_t(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }

        false
    }
    
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == *kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous() 
    }

    fn is_at_end(&self) -> bool { self.peek().kind == TokenKind::Eof }

    fn peek(&self) -> &Token { &self.tokens[self.current] }

    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }

    fn consume(&mut self, kind: &TokenKind, msg: &'static str) {
        if self.check(kind) {
            self.advance();
            return;
        }
        panic!("{}", msg);
    }

    fn statement(&mut self) -> Stmt {
        if self.match_t(&[TokenKind::Let]) {
            self.consume(&TokenKind::Identifier, "identifier expected");
            let id = self.name();
            self.consume(&TokenKind::Eq, "eq expected");
            let e = self.expression();
            Stmt::Let(id, e)
        } else {
            Stmt::Expr(self.expression())
        }
    }

    fn name(&mut self) -> String {
        self.previous().data.as_ref().unwrap().clone()
    }

    // expression = factor ( ("-" | "+") expression )*
    fn expression(&mut self) -> Expr {
        let expr = self.factor();

        if self.match_t(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator_kind = self.previous().kind;
            let right = self.expression();
            match operator_kind {
                TokenKind::Plus => Expr::Add(Box::new(expr), Box::new(right)),
                TokenKind::Minus => Expr::Sub(Box::new(expr), Box::new(right)),
                _ => panic!("smth went wrong term")
            }
        } else {
            Expr::Factor(expr)
        }
    }

    // factor = unary ( ("/" | "*") factor )*
    fn factor(&mut self) -> Factor {
        let expr = self.unary();

        if self.match_t(&[TokenKind::Star, TokenKind::Slash]) {
            let operator_kind = self.previous().kind;
            let right = self.factor();
            match operator_kind {
                TokenKind::Star => Factor::Multiply(Box::new(expr), Box::new(right)),
                TokenKind::Slash => Factor::Divide(Box::new(expr), Box::new(right)),
                _ => panic!("smth went wrong factor")
            }
        } else {
            Factor::Unary(expr)
        }
    }

    // unary = "-" unary | number
    fn unary(&mut self) -> Unary {
        if self.match_t(&[TokenKind::Minus]) {
            let operator_kind = self.previous().kind;
            let right = self.unary();
            match operator_kind {
               TokenKind::Minus => Unary::Minus(Box::new(right)),
               _ => panic!("wtf unary")
            }
        } else {
            Unary::Primary(self.primary())
        }
    }

    fn primary(&mut self) -> Primary {
        if self.match_t(&[TokenKind::Literal { kind: LiteralKind::Number }]) {
            Primary::Literal(self.previous().data.as_ref().unwrap().parse().unwrap())
        } else if self.match_t(&[TokenKind::Identifier]) {
            Primary::Identifier(self.name())
        } else if self.match_t(&[TokenKind::OpenParen]) {
            let expr = self.expression();
            self.consume(&TokenKind::CloseParen, "Expect ')' after expression");
            Primary::Grouping(Box::new(expr))
        } else {
            panic!("unexpected primary")
        }
    }
}
