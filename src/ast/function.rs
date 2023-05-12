use crate::ast::ast::{Arg, Ast, ReturnTypes};

use super::ast::TypesArg;

#[derive(Debug, PartialEq, Clone)]
pub struct Func {
    pub name: String,
    pub args: Vec<Arg>,
    pub body: Option<Box<Ast>>,
    pub return_type: ReturnTypes,
    pub line: usize,
}

impl Func {
    pub fn remove_arg_at_index(&mut self, i: usize) -> Option<Arg> {
        if self.args.len() < i {
            return None;
        }
        let arg = self.args.remove(i);
        Some(arg)
    }

    pub fn get_arg_index_(&self, value: &str) -> Option<u32> {
        for (i, arg) in self.args.iter().enumerate() {
            if arg.value == value {
                return Some(i as u32);
            }
        }
        None
    }

    /// Checks if args uses a certaint value only works for identifiers
    pub fn args_uses(&self, value: &str) -> bool {
        for arg in &self.args {
            if arg.type_ == TypesArg::None && arg.value == value {
                return true;
            }
        }
        false
    }
}
