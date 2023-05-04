use crate::{
    ast::{
        ast::{Arg, Ast},
        function::Func,
        variable::Variable,
    },
    lepa_analyzer::unused::unused_vars::DetectorVars,
};

pub mod unused_vars;

#[derive(Debug)]
pub enum UnusedValues {
    Variable(Variable),
    Function(Func),
    Argument(Arg),
}

struct DetectUnused {
    ast: Ast,
}

impl DetectUnused {
    fn new(ast: Ast) -> DetectUnused {
        DetectUnused { ast: ast.clone() }
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
        let unused = Vec::new();
        let detector = DetectUnused::new(ast);

        // unused vars
        let _ = detector.detect_unused_vars();
        unused
    }
}

impl Unused for Ast {}
