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
                let get_var = code.module.get_global(&arg.value);
                match get_var {
                    Some(value) => {
                        let value  = value.as_pointer_value();
                        args.push(value.into());
                    }
                    None => {
                        LOGGER.error(&format!("Couldn't find variable {} in local scope or global scope.", &arg.value))
                    }
                }
            }
            TypesArg::Number => {
                let value = code.context.i32_type();
                let int = value.const_int(arg.value.parse().unwrap(), false);
                args.push(int.into());
            }
            TypesArg::String => {
                let rand_name: f64 = rand::random();
                let value = code
                    .builder
                    .build_global_string_ptr(&arg.value, &format!("str_argument{rand_name}",));
                args.push(value.as_pointer_value().into());
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
