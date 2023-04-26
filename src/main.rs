use lepa::{
    errors::{logger::{Log, LogLevels, Logger}, error::{ErrorBuilder, BuildError}},
    lexer::lexer::{Lexer, Token}, logme,
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/main.lp").to_string());

    let log = Logger::new(LogLevels::Info);
    let mut error:ErrorBuilder = BuildError::new();
    let error = error.message("Something went wrong").line(9).helper("Consider not going wrong?").file_name("rust.rs").build();
    logme!("warning",  "wow");

    logme!("error",  error, "me", "error");
    println!("{}", error);
}
