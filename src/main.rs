use lepa::{lexer::lexer::{Lexer, Token}, {parser::Parser, parser::Parse}};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/variable.lp").to_string());
    let mut parser = Parser::new(lexer);
    let parse = parser.parse();
    println!("{:#?}", parse);
}
