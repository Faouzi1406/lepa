use inkwell::values::FunctionValue;

use crate::codegen::get_args_function::Args;
use crate::codegen::std_compiler::Std;
use crate::codegen::CodeGen;
use crate::codegen::LOGGER;
use crate::{ast::function::Func, errors::logger::Log};

pub trait GenFunctionCall<'ctx> {
    fn gen_function_call(&self, function: &Func, func: &FunctionValue<'ctx>) -> Result<(), String>;
}

impl<'ctx> GenFunctionCall<'ctx> for CodeGen<'ctx> {
    fn gen_function_call(&self, function: &Func, func: &FunctionValue<'ctx>) -> Result<(), String> {
        let args = &self.get_args_value(function, func);

        let std = &self.std_functions(function, args.clone(), None);
        match std {
            Ok(_) => {
                return Ok(());
            }
            Err(_) => (),
        }

        let Some(func) = &self.module.get_function(&function.name) else {
                let error = format!("Call to function that doesn't exist {} on line", &function.name);
                LOGGER.error(&error);
                return Err(error)
        };

        let _ = &self.builder.build_call(*func, args, "call");

        Ok(())
    }
}
