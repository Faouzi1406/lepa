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

pub mod unused_vars;
pub mod unused_fn;

#[derive(Debug)]
pub enum UnusedValues {
    Variable(Variable),
    Function(Func),
    Argument(Arg),
}

struct DetectUnused<'a> {
    ast: &'a Ast,
}

impl<'a> DetectUnused<'a> {
    fn new(ast: &'a Ast) -> DetectUnused {
        DetectUnused {  ast }
    }
}

pub trait Unused {
    /// Finds all unused value's.
    ///
    /// Cases:
    ///  - Variables
    ///  - Functions
    ///  - Arguments
    fn find_unused(ast: Ast) -> Vec<UnusedValues> {
        let mut unused = Vec::new();
        let detector = DetectUnused::new(&ast);

        // unused vars
        let mut found_var = detector.detect_unused_vars();
        unused.append(&mut found_var);

        for var in &unused {
            match var {
                UnusedValues::Argument(_) => (),
                UnusedValues::Variable(var) => LOGGER.warning(&format!(
                    "Found a unused variable with name: {}; on line {};",
                    var.name, var.line
                )),
                _ => (),
            }
        }
        unused
    }
}

impl Unused for Ast {}
