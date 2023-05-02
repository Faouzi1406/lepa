use super::{CodeGen, LOGGER};
use crate::{
    ast::{ast::TypesArg, function::Func},
    errors::logger::Log,
};
use inkwell::values::BasicMetadataValueEnum;

pub fn get_args_value<'ctx>(
    code: &CodeGen<'ctx>,
    function: &Func,
) -> Vec<BasicMetadataValueEnum<'ctx>> {
    let vars = &function.args;
    let mut args = Vec::new();

    for arg in vars {
        match arg.type_ {
            TypesArg::None => {
                let get_var = code.module.get_global(&arg.value);
                match get_var {
                    Some(value) => {
                        let value = value.get_initializer();
                        match value {
                            Some(value) => {
                                let type_ = value.get_type();
                                match type_ {
                                    inkwell::types::BasicTypeEnum::IntType(_) => {
                                        args.push(value.into_int_value().into())
                                    }
                                    _ => (),
                                }
                            }
                            None => {}
                        }
                    }
                    None => LOGGER.error(&format!(
                        "Couldn't find variable {} in local scope or global scope.",
                        &arg.value
                    )),
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

    return args;
}
