use std::io::Write;

use lepa::{
    parser_lexer::lexer::lexer::{Lexer, Token},
    parser_lexer::parser::{Parse, Parser}, compiler::Compile, logme,
};

fn main() {
    let lexer = Token::lex(include_str!("../sample_code/main.lp").to_string());
    let parse = Parser::new(lexer).parse();
    let compile = parse.unwrap().compile();
    let main_file = std::fs::File::create("./main");

    match main_file {
        Ok(mut file) => {
            let write = file.write_all(compile.as_bytes());
            if write.is_err() {
                logme!("error", write);
            }
        }
        Err(val) => {
            logme!("error", val);
        }
    }
}
