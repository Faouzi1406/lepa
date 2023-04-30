use crate::ast::ast::{Arg, Ast, ReturnTypes};

#[derive(Debug, PartialEq, Clone)]
pub struct Func {
    pub name: String,
    pub args: Vec<Arg>,
    pub body: Option<Box<Ast>>,
    pub return_type: ReturnTypes,
}

impl Func {
    pub fn get_arg_index_(&self, value: &str) -> Option<u32> {
        for (i, arg) in self.args.iter().enumerate() {
            if arg.value == value {
                return Some(i as u32);
            }
        }
        return None;
    }
}
