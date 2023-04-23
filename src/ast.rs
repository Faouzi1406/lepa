pub struct Variable {
    pub name: String,
    pub value: String,
}

pub enum Type {
    Program,
    Variable(Variable),
    Function,
    Block,
}

pub struct Ast {
    pub type_: Type,
    pub body: Vec<Ast>,
}

impl Ast {
    pub fn new(type_: Type) -> Self {
        Self {
            type_,
            body: Vec::new(),
        }
    }
}
