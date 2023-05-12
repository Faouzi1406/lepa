use crate::errors::error::BuildError;
use crate::errors::error::ErrorBuilder;

use super::ast::TypesArg;
use super::function::Func;

#[derive(Debug, PartialEq, Clone)]
pub enum TypeVar {
    Arr { values: Vec<TypeVar> },
    Number(i32),
    String(String),
    Identifier(String),
    FunctionCall(Func),
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
    pub fn uses(&self, uses: &str) -> bool {
        match &self {
            TypeVar::Identifier(value) if *value == uses => true,
            TypeVar::Arr { values } => {
                for value in values {
                    let false = value.uses(uses) else {
                        return true;
                    };
                }
                false
            }
            TypeVar::FunctionCall(func) => {
                let args = &func.args;
                for arg in args {
                    if arg.type_ == TypesArg::None && arg.value == uses {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub type_: TypeVar,
    pub line: usize,
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
        if !self.name.is_empty() {
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
