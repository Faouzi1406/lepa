use crate::ast::Ast;

pub struct Compiler {
    pub ast: Ast,
}

pub trait Compile {
    fn new(ast: Ast) -> Compiler {
        return Compiler { ast };
    }
    fn compile(&mut self);
}

impl Compile for Compiler {
    fn compile(&mut self) {
        
    }
}
