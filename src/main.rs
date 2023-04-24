use lepa::{
    compiler::compiler::{Compile, Compiler},
    lexer::lexer::{Lexer, Token},
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/main.lp").to_string());
    println!("{:#?}", lexer);
}
