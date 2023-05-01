use super::{std_compiler::Std, CodeGen, LOGGER};
use crate::{
    ast::{ast::TypesArg, function::Func},
    errors::logger::Log,
};

pub fn compile_function_call(code: &CodeGen, function: &Func) -> Result<(), String> {
    let vars = &function.args;
    let mut args = vec![];

    for arg in vars {
        match arg.type_ {
            TypesArg::None => {
                LOGGER.error(&format!(
                    "Found a invalid function argument, {} doesn't have a type.",
                    arg.value
                ));
            }
            TypesArg::Number => {
                let value = code.context.i32_type();
                let int = value.const_int(arg.value.parse().unwrap(), false);
                args.push(int.into());
            }
            TypesArg::String => {
                let bytes = &arg.value;
                let value = code.context.const_string(bytes.as_bytes(), false);
                args.push(value.into());
            }
        }
    }

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
