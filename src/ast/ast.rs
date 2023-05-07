use crate::{
    errors::{error::ErrorBuilder, error_messages::invalid_if_statement_operator},
    parser_lexer::lexer::lexer::Operators,
};

use super::{
    function::Func,
    use_::Use,
    variable::{TypeVar, Variable},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypesArg {
    String,
    Number,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub value: String,
    pub type_: TypesArg,
}

impl Into<TypeVar> for TypesArg {
    fn into(self) -> TypeVar {
        match self {
            Self::String => TypeVar::String("".into()),
            Self::Number => TypeVar::Number(0),
            Self::None => TypeVar::None,
        }
    }
}

impl Arg {
    pub fn new() -> Arg {
        Arg {
            value: String::new(),
            type_: TypesArg::None,
        }
    }
    pub fn assign_value(&mut self, value: String) -> Result<(), &'static str> {
        if self.value != "" {
            return Err("This argument already has a value");
        }
        self.value = value;
        Ok(())
    }
    pub fn assign_type(&mut self, value: TypesArg) -> Result<(), &'static str> {
        if self.type_ != TypesArg::None {
            return Err("This argument already has a type");
        }
        self.type_ = value;
        Ok(())
    }
    pub fn clear_value(&mut self) {
        self.value = "".into();
    }
    pub fn clear_type(&mut self) {
        self.type_ = TypesArg::None;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnTypes {
    Number,
    String,
    Identifier,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: String,
    pub type_: ReturnTypes,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    EqEq(TypeVar, TypeVar),
    More(TypeVar, TypeVar),
    MoreEq(TypeVar, TypeVar),
    Less(TypeVar, TypeVar),
    LessEq(TypeVar, TypeVar),
    None,
}

impl Case {
    pub fn assign(&self, one: TypeVar, two: TypeVar) -> Case {
        match self {
            Case::MoreEq(_, _) => Case::MoreEq(one, two),
            Case::EqEq(_, _) => Case::EqEq(one, two),
            Case::More(_, _) => Case::More(one, two),
            Case::LessEq(_, _) => Case::LessEq(one, two),
            Case::Less(_, _) => Case::Less(one, two),
            Case::None => Case::None,
        }
    }
    pub fn from_op(value: Operators) -> Result<Case, ErrorBuilder> {
        let value = match value {
            Operators::EqEq => Case::EqEq(TypeVar::None, TypeVar::None),
            Operators::Less => Case::Less(TypeVar::None, TypeVar::None),
            Operators::LessEq => Case::LessEq(TypeVar::None, TypeVar::None),
            Operators::More => Case::More(TypeVar::None, TypeVar::None),
            Operators::MoreEq => Case::MoreEq(TypeVar::None, TypeVar::None),
            Operators::Invalid(invalid) => {
                return Err(invalid_if_statement_operator(Operators::Invalid(invalid)))
            }
            Operators::Eq => return Err(invalid_if_statement_operator(Operators::Eq)),
        };
        Ok(value)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Logic {
    pub if_: Vec<Case>,
    pub do_: Box<Ast>,
    pub else_: Option<Box<Ast>>,
}

impl Logic {
    pub fn new(case: Vec<Case>, else_: Option<Box<Ast>>, do_: Ast) -> Logic {
        return Logic {
            if_: case,
            do_: Box::from(do_),
            else_,
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Program,
    ConstVar(Variable),
    Variable(Variable),
    Function(Func),
    /// A call to a function
    //
    /// It contains the func that is being called under Func.name and the arguments passed into the
    /// function
    FunctionCall(Func),
    Block,
    Return(Return),
    Use(Use),
    Logic(Logic),
}

#[derive(Debug, PartialEq, Clone)]
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

pub trait AstVar {
    fn var_name(&self) -> Option<String>;
    fn var_value(&self) -> Option<TypeVar>;
}

impl AstVar for Ast {
    fn var_name(&self) -> Option<String> {
        match &self.type_ {
            Type::Variable(var) => Some(var.name.clone()),
            _ => None,
        }
    }

    fn var_value(&self) -> Option<TypeVar> {
        match &self.type_ {
            Type::Variable(var) => Some(var.type_.clone()),
            _ => None,
        }
    }
}
