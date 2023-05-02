use super::{get_args_function::get_args_value, std_compiler::Std, CodeGen, LOGGER};
use crate::{ast::function::Func, errors::logger::Log};

pub fn compile_function_call(code: &CodeGen, function: &Func) -> Result<(), String> {
    let args = get_args_value(code, function);

    let std = code.std_functions(function, args.clone());
    match std {
        Ok(_) => {
            return Ok(());
        }
        Err(_) => (),
    }

    let Some(func) = code.module.get_function(&function.name) else {
                let error = format!("Call to function that doesn't exist {} on line", &function.name);
                LOGGER.error(&error);
                return Err(error)
    };

    let _ = code.builder.build_call(func, &args, "call");

    return Ok(());
}
