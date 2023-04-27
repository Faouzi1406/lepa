use lepa::{
    errors::{logger::{Log}, error::{BuildError}},
    parser_lexer::lexer::lexer::{Lexer, Token}, parser_lexer::parser::{Parser, Parse},
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/testing/var.lp").to_string());
    let parse = Parser::new(lexer).parse();
    println!("{:#?}", parse);
}
