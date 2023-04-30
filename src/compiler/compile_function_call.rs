use super::{CodeGen, LOGGER};
use crate::{ast::{function::Func, ast::TypesArg}, errors::logger::Log};

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
                LOGGER.error(&format!(
                        "Found a invalid function argument, {} is of type string, this is not supported yet :(.",
                        arg.value
                    ));
            }
        }
    }

    let Some(func) = code.module.get_function(&function.name) else {
        let error = format!("Call to function that doesn't exist {} on line", &function.name);
        LOGGER.error(&error);
        return Err(error)
    };

    let _call = code.builder.build_call(func, &args, "call");

    return Ok(());
}
