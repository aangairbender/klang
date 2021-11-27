use lexer::{token::Token, token_kind::TokenKind};

use crate::visit::Visitor;

mod lexer;
mod ast;
mod parser;
mod visit;
mod interpreter;

fn main() {
    let mut interpreter = interpreter::Interpreter::new();
    loop {
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
    
        let mut tokens2 = lexer::tokenize(input.as_str()).collect::<Vec<_>>();
        tokens2.push(Token::new(TokenKind::Eof, 0));
        let mut tokens = Vec::<Token>::new();
        for t in tokens2 {
            if t.kind == TokenKind::Whitespace {
                continue;
            }
            tokens.push(t);
        }
        // for t in &tokens {
        //     eprintln!("{:?}", t);
        // }
        let mut parser = parser::Parser::new(tokens);
        let root = parser.parse();
        let eval_res = interpreter.visit_stmt(&root);
        eprintln!("eval: {}", eval_res);
    }
}
