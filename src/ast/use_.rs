use std::{fs, path::PathBuf};

use crate::{
    errors::error::{BuildError, ErrorBuilder},
    parser_lexer::{
        lexer::lexer::{Lexer, Token},
        parser::{Parse, Parser},
    }, compiler::Compile,
};

use super::ast::{Ast, Type};

/// The use struct containing the file name of the use
#[derive(Clone, Debug, PartialEq)]
pub struct Use(String);

#[derive(Clone, Debug, PartialEq)]
pub struct Used(Vec<Use>);

pub struct UsedCompiled {
    pub file_name: String,
    pub contents: String,
}

impl UsedCompiled {
    fn new(file_name:String, contents:String) -> UsedCompiled {
        UsedCompiled { file_name, contents }
    }
}

impl Use {
    pub fn new(file_name: String) -> Use {
        Use(file_name)
    }

    pub fn valide_ext(&self) -> bool {
        let path = PathBuf::from(&self.0);
        let Some(ext) = path.extension() else {
            return false;
        };
        ext == "lp"
    }

    pub fn exists(&self) -> bool {
        let path = PathBuf::from(&self.0);
        path.exists()
    }
    pub fn validate(&self) -> bool {
        self.exists() && self.valide_ext()
    }
}

pub trait GetUses {
    fn get_use(ast: Ast) -> Result<Used, ErrorBuilder> {
        let uses = Self::get_uses(ast);
        return Ok(Used(uses?));
    }

    fn get_uses(ast: Ast) -> Result<Vec<Use>, ErrorBuilder> {
        let mut uses = Vec::new();
        for ast in ast.body {
            match ast.type_ {
                Type::Use(use_) => {
                    let true = use_.validate() else {
                    return Err(ErrorBuilder::new().message(format!("Found a invalid use: {};", use_.0)).helper("consider changing the extension of the file to .lp or bringing it into scope.").build_error())
                };
                    uses.push(use_);
                }
                Type::Block => {
                    uses.append(&mut Self::get_uses(ast)?);
                }
                Type::Function(func) => {
                    let Some(body) = func.body else {
                        continue;
                    };
                    uses.append(&mut Self::get_uses(*body)?);
                }
                _ => (),
            }
        }
        return Ok(uses);
    }
}

pub trait CompileUses {
    fn compile(&self) -> Result<Vec<UsedCompiled>, ErrorBuilder>;
}

impl GetUses for Use {}

impl CompileUses for Used {
    fn compile(&self) -> Result<Vec<UsedCompiled>, ErrorBuilder> {
        let mut compile_used = Vec::new();
        for file in &self.0 {
            let string = fs::read_to_string(&file.0).unwrap();
            let lexed = Token::lex(string);
            let parse = Parser::new(lexed).parse().unwrap();
            let compile = parse.compile();
            compile_used.push(UsedCompiled::new(file.0.clone(), compile));
        }
        return Ok(compile_used);
    }
}
