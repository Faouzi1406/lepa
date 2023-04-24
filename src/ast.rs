#[derive(Debug , PartialEq)]
pub enum TypeVar {
    Number(i32),
    String(String),
    None,
}

impl TypeVar {
    pub fn parse_number(num: String) -> Self {
        let num = num.parse().unwrap();
        Self::Number(num)
    }
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub type_: TypeVar,
}

#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub args: Vec<Variable>,
    pub body: Option<Box<Ast>>,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Program,
    Variable(Variable),
    Function(Func),
    Block,
}

#[derive(Debug, PartialEq)]
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

    pub fn var_name(&self) -> Option<String> {
        match &self.type_ {
            Type::Variable(var) => Some(var.name.clone()),
            _ => None,
        }
    }
}
