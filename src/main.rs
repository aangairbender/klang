mod lexer;

fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
    
        let tokens = lexer::tokenize(input.as_str()).collect::<Vec<_>>();
        for t in tokens {
            eprintln!("{}", t);
        }
    }
}
