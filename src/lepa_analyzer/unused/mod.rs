use colored::Colorize;

use crate::{
    ast::{
        ast::{Arg, Ast},
        function::Func,
        variable::Variable,
    },
    codegen::LOGGER,
    errors::logger::Log,
    lepa_analyzer::unused::unused_vars::DetectorVars,
};

use self::unused_fn::DetectUnusedFunc;

pub mod unused_fn;
pub mod unused_vars;

#[derive(Debug)]
pub enum UnusedValues<'a> {
    Variable(&'a Variable),
    Function(&'a Func),
    Argument(&'a Arg),
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
    fn find_unused(ast: Ast) {
        let mut unused: Vec<UnusedValues> = Vec::new();
        let detector = DetectUnused::new(&ast);
        

        // unused vars
        unused.append(&mut detector.detect_unused_vars());
        unused.append(&mut detector.detect_unused_funcs());
        //unused.append(&mut detector.detect_unused_funcs());

        for var in &unused {
            match var {
                UnusedValues::Argument(_) => (),
                UnusedValues::Variable(var) => LOGGER.display_warning(&format!(
                    "Found a unused {} with name: {}; on line {};",
                    "variable".bold().yellow(),
                    var.name,
                    var.line
                )),
                UnusedValues::Function(func) => LOGGER.display_warning(&format!(
                    "Found a {} unused with name: {}; on line {};",
                    "function".bold().yellow(),
                    func.name,
                    func.line
                )),
            }
        }
    }
}

impl<'a> Unused<'a> for Ast {}
