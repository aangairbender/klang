use crate::lexer::literal_kind::LiteralKind;
use crate::lexer::token::Token;
use crate::ast::ast_node::AstNode;
use crate::ast::nodes::*;
use crate::lexer::token_kind::TokenKind;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Box<dyn AstNode<Output = i32>> {
        self.expression()
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

    // expression = term
    fn expression(&mut self) -> Box<dyn AstNode<Output = i32>> {
        self.term()
    }

    // term = factor ( ("-" | "+") factor )*
    fn term(&mut self) -> Box<dyn AstNode<Output = i32>> {
        let mut expr = self.factor();

        while self.match_t(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator_kind = self.previous().kind;
            let right = self.factor();
            expr = Box::new(BinOpNode::new(
                match operator_kind {
                    TokenKind::Plus => BinOp::Plus,
                    TokenKind::Minus => BinOp::Minus,
                    _ => panic!("smth went wrong term")
                },
                expr,
                right
            ));
        }

        expr
    }

    // factor = unary ( ("/" | "*") unary )*
    fn factor(&mut self) -> Box<dyn AstNode<Output = i32>> {
        let mut expr = self.unary();

        while self.match_t(&[TokenKind::Star, TokenKind::Slash]) {
            let operator_kind = self.previous().kind;
            let right = self.unary();
            expr = Box::new(BinOpNode::new(
                match operator_kind {
                    TokenKind::Star => BinOp::Multiply,
                    TokenKind::Slash => BinOp::Divide,
                    _ => panic!("smth went wrong factor")
                },
                expr,
                right
            ));
        }

        expr
    }

    // unary = "-" unary | number
    fn unary(&mut self) -> Box<dyn AstNode<Output = i32>> {
        if self.match_t(&[TokenKind::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Box::new(UnaryOpNode::new(
                UnaryOp::Minus,
                right
            ));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<dyn AstNode<Output = i32>> {
        if self.match_t(&[TokenKind::Literal { kind: LiteralKind::Number }]) {
            return Box::new(NumberNode(self.previous().data.as_ref().unwrap().parse().unwrap()))
        }

        if self.match_t(&[TokenKind::Identifier]) {
            return Box::new(IdentifierNode(*self.previous().data.as_ref().unwrap()))
        }

        if self.match_t(&[TokenKind::OpenParen]) {
            let expr = self.expression();
            self.consume(&TokenKind::CloseParen, "Expect ')' after expression");
            return expr;
        }

        panic!("unexpected primary")
    }
}
