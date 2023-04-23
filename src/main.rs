use lepa::{lexer::lexer::{Lexer, Token}, parser::Parser};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/main.lp").to_string());
    let parser = Parser::new(lexer);
    println!("{:#?}", parser);
}
