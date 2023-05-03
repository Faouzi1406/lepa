use crate::ast::{ast::Arg, function::Func, variable::Variable};

pub mod unused;

pub enum TypeAnalyzed {
    Variable(Variable),
    Argument(Arg),
    Function(Func),
}

pub enum TypeAnalyzer {
    Unused,
}

pub struct Analyzed {
    pub type_: TypeAnalyzed,
    pub type_analyzer: TypeAnalyzer,
}
