use lepa::{
    errors::logger::{Log, LogLevels, Logger},
    lexer::lexer::{Lexer, Token},
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/main.lp").to_string());

    let log = Logger::new(LogLevels::Info);
    log.info(&"something wrong worngin");
    log.info(&"Updating the parser okaychamp");
}
