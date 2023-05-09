use colored::Colorize;

use crate::{
    ast::{
        ast::{Arg, Ast},
        function::Func,
        variable::Variable,
    },
    codegen::LOGGER,
    errors::logger::Log,
};

use self::{unused_args::DetectUnusedArgs, unused_fn::DetectUnusedFunc, unused_vars::DetectorVars};

pub mod unused_args;
pub mod unused_fn;
pub mod unused_vars;

#[derive(Debug)]
pub enum UnusedValues<'a> {
    Variable(&'a Variable),
    Function(&'a Func),
    Argument(&'a Arg, &'a str),
}

struct DetectUnused<'a> {
    ast: &'a Ast,
}

impl<'a> DetectUnused<'a> {
    fn new(ast: &'a Ast) -> DetectUnused<'a> {
        DetectUnused { ast }
    }
}

pub trait Unused<'a> {
    /// Finds all unused value's.
    ///
    /// Cases:
    ///  - Variables
    ///  - Functions
    ///  - Arguments
    fn find_unused(ast: &'a Ast) -> Vec<UnusedValues<'a>> {
        let detector: DetectUnused<'a> = DetectUnused::new(ast);
        let mut unused = Vec::new();
        unused.append(&mut detector.detect_unused_funcs());
        unused.append(&mut detector.detect_unused_vars());
        unused.append(&mut detector.detect_unused_args());
        let args = detector.detect_arg();
        unused
    }
    /// Takes in all unused values and logs them to the terminal in a formatted way
    fn log_unused(unused_values: Vec<UnusedValues<'a>>) {
        for unused in unused_values {
            match unused {
                UnusedValues::Variable(var) => LOGGER.display_warning(&format!(
                    "Found a unused {}, at line {}, name {}",
                    "variable".yellow().bold(),
                    var.line,
                    var.name
                )),
                UnusedValues::Function(func) => LOGGER.display_warning(&format!(
                    "Found a unused {}, at line {}, name {}",
                    "function".yellow().bold(),
                    func.line,
                    func.name
                )),
                UnusedValues::Argument(arg, func) => LOGGER.display_warning(&format!(
                    "Found a unused {}; name = {}, in function {}",
                    "argument".yellow().bold(),
                    arg.value,
                    func
                )),
            }
        }
    }
}

impl<'a> Unused<'a> for Ast {}
