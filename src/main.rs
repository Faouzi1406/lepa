use lepa::{
    errors::{logger::{Log, LogLevels, Logger}, error::{ErrorBuilder, BuildError}},
    lexer::lexer::{Lexer, Token}, logme, parser::{Parser, Parse},
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/testing/test_functions.lp").to_string());
    let logger = Logger::new(LogLevels::Info);
    let parse = Parser::new(lexer).parse();
    println!("{:#?}", parse);
}
