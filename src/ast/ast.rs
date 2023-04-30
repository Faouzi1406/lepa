use crate::{
    errors::{
        error::{BuildError, ErrorBuilder},
        error_messages::invalid_if_statement_operator,
    },
    parser_lexer::lexer::lexer::Operators,
};

use super::function::Func;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeVar {
    Arr { values: Vec<TypeVar> },
    Number(i32),
    String(String),
    Identifier(String),
    None,
}

impl TypeVar {
    pub fn is_none(&self) -> bool {
        self == &TypeVar::None
    }
    pub fn parse_number(num: String) -> Self {
        let num = num.parse().unwrap();
        Self::Number(num)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub type_: TypeVar,
    pub line: usize,
}

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

pub trait VarBuilder {
    /// Create a new variable with no known type.
    fn new() -> Self;
    /// Assign name to new created variable
    #[must_use]
    fn name(&mut self, name: impl AsRef<str>) -> Result<(), ErrorBuilder>;
    /// Assign type to newly created type
    #[must_use]
    fn type_(&mut self, type_: TypeVar) -> Result<(), ErrorBuilder>;
    /// Assign line to variable
    fn line(&mut self, line: usize) -> &mut Self;
}

impl VarBuilder for Variable {
    fn new() -> Self {
        Self {
            name: String::new(),
            type_: TypeVar::None,
            line: 0,
        }
    }
    fn type_(&mut self, type_: TypeVar) -> Result<(), ErrorBuilder> {
        if !self.type_.is_none() {
            return Err(ErrorBuilder::new()
                .message(format!(
                    "Tried assigning a value to and already variable value: {:?}",
                    type_
                ))
                .file_name("should_ad_this:)")
                .line(self.line)
                .build_error());
        }
        self.type_ = type_;
        Ok(())
    }
    fn name(&mut self, name: impl AsRef<str>) -> Result<(), ErrorBuilder> {
        if self.name != "" {
            return Err(ErrorBuilder::new()
                .message(format!(
                    "Tried assigning a name to and already named variable: {}",
                    name.as_ref()
                ))
                .file_name("should_ad_this:)")
                .line(self.line)
                .build_error());
        }
        self.name = name.as_ref().to_string();
        Ok(())
    }
    fn line(&mut self, line: usize) -> &mut Self {
        self.line = line;
        self
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
    pub fn assign(&self,one:TypeVar, two:TypeVar) -> Case {
        match self {
            Case::MoreEq(_, _) => Case::MoreEq(one, two),
            Case::EqEq(_, _) => Case::EqEq(one, two),
            Case::More(_, _) => Case::More(one, two),
            Case::LessEq(_, _) => Case::LessEq(one, two),
            Case::Less(_, _) => Case::Less(one, two),
            Case::None => Case::None
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
    pub if_: Case,
    pub do_: Box<Ast>,
    pub else_: Option<Box<Ast>>,
}

impl Logic {
    pub fn new(case:Case,else_: Option<Box<Ast>>, do_: Ast) -> Logic {
        return Logic {
            if_:case,
            do_: Box::from(do_),
            else_,
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Program,
    Variable(Variable),
    Function(Func),
    /// A call to a function
    //
    /// It contains the func that is being called under Func.name and the arguments passed into the
    /// function
    FunctionCall(Func),
    Block,
    Return(Return),
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
