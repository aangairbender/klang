use lexer::{token::Token, token_kind::TokenKind};

mod lexer;
mod ast;
mod parser;
mod visit;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
    
        let mut tokens2 = lexer::tokenize(input.as_str()).collect::<Vec<_>>();
        tokens2.push(Token::new(TokenKind::Eof, 0));
        let mut tokens = Vec::<Token>::new();
        for t in tokens2 {
            if t.kind == TokenKind::Whitespace {
                continue;
            }
            tokens.push(t);
        }
        for t in &tokens {
            eprintln!("{:?}", t);
        }
        let mut parser = parser::Parser::new(tokens);
        let root = parser.parse();
        let eval_res = root.eval();
        eprintln!("eval: {}", eval_res);
    }
}
