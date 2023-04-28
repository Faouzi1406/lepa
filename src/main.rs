use lepa::{
    parser_lexer::lexer::lexer::{Lexer, Token},
    parser_lexer::parser::{Parse, Parser}, compiler::{Compiler, Compile},
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/main.lp").to_string());
    let parse = Parser::new(lexer).parse();
    let compiler = Compiler::new(parse.unwrap());
    let compile = compiler.compile();
}
