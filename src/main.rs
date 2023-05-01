use std::{io::Write, fs};

use lepa::{
    ast::use_::{GetUses, Use, CompileUses},
    compiler::Compile,
    logme,
    parser_lexer::lexer::lexer::{Lexer, Token},
    parser_lexer::parser::{Parse, Parser},
};

fn main() {
    let files = fs::read_to_string("./main.lp");
    let lexer = Token::lex(files.unwrap());
    let parse = Parser::new(lexer).parse().unwrap();

    let uses = Use::get_use(parse.clone()).unwrap();
    let compile_uses = uses.compile().unwrap();

    for compile in compile_uses {
        let mut file = fs::File::create("./target/".to_string()  + &compile.file_name).unwrap();
        file.write_all(compile.contents.as_bytes()).expect("write");
    }

    let compile = parse.compile();
    let main_file = std::fs::File::create("./target/main");

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
