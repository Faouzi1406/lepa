use crate::ast::Ast;

pub struct Compiler {
    pub ast:Ast,
}

pub trait Compile {
    fn new(ast:Ast) -> Compiler {
        return Compiler  {
            ast,
        }
    }
}

impl Compile for Compiler {}
