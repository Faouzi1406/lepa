use lepa::{ast::ast::Ast, codegen::Compile, lepa_analyzer::unused::Unused, logme};
use std::{fs, io::Write};

use lepa::{
    ast::use_::{CompileUses, GetUses, Use},
    parser_lexer::lexer::{Lexer, Token},
    parser_lexer::parser::{Parse, Parser},
};

fn compile() {
    let files = fs::read_to_string("./main.lp");
    let mut lexer = Token::lex(files.unwrap());
    let parse = Parser::new(lexer.clone()).parse();

    if parse.is_err() {
        println!("{}", parse.err().unwrap());
        return;
    }

    let parse = parse.unwrap();
    let unused = Ast::find_unused(&parse);
    Ast::log_unused(unused);

    let uses = Use::get_use(&parse).unwrap();
    let compile_uses = uses.compile().unwrap();
    let mut tokens_now = Vec::new();
    for mut tokens in compile_uses {
        tokens_now.append(&mut tokens.contents);
    }
    tokens_now.append(&mut lexer);

    let parse = Parser::new(tokens_now).parse().unwrap();

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

    Ast::create_binary("/target/main");
}

fn main() {
    compile()
}
