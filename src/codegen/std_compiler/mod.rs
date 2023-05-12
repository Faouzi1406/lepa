mod math;
mod print_f;

use self::{math::StdMath, print_f::StdPrint};
use super::CodeGen;
use crate::ast::function::Func;
use inkwell::values::BasicMetadataValueEnum;

pub trait Std {
    /// Std functions
    ///
    /// Matches a function against a existing compiler implementation of the function;
    ///
    /// @print("Hello there!"); // Will get detected by the std_compiler and turned into it's
    /// llvm ir/ assembly implementation of the function
    ///
    /// These functions are usually not touched by the user and have a wrapper implemented in the
    /// std of the language, however there has to be a std before that is possible.
    fn std_functions(
        &self,
        func: &Func,
        args_values: Vec<BasicMetadataValueEnum>,
        var_name: Option<&str>,
    ) -> Result<(), String>;
}

impl<'ctx> Std for CodeGen<'ctx> {
    fn std_functions(
        &self,
        func: &Func,
        args_values: Vec<BasicMetadataValueEnum>,
        var_name: Option<&str>,
    ) -> Result<(), String> {
        match func.name.as_str() {
            "@printf" => self.std_print_f(func, args_values),
            "@add" if var_name.is_some() => self.std_add(args_values, func, var_name.unwrap()),
            name => {
                return Err(format!(
                    "Got a std_function call but the function {}; is not a std function",
                    name
                ))
            }
        }
        Ok(())
    }
}
