#[derive(Debug)]
pub enum TypeVar {
    Number(i32),
    String(String),
    None
}

impl TypeVar {
    pub fn parse_number(num:String) -> Self  {
        let num = num.parse().unwrap();
        Self::Number(num)
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub type_:TypeVar
}

#[derive(Debug)]
pub enum Type {
    Program,
    Variable(Variable),
    Function,
    Block,
}

#[derive(Debug)]
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
