use std::process::exit;

use super::{CodeGen, LOGGER};
use crate::{
    ast::{ast::TypesArg, function::Func},
    errors::logger::Log,
};
use inkwell::{
    types::BasicTypeEnum,
    values::{BasicMetadataValueEnum, FunctionValue},
};

pub fn get_args_value<'ctx>(
    code: &CodeGen<'ctx>,
    function: &Func,
    func: &FunctionValue<'ctx>,
) -> Vec<BasicMetadataValueEnum<'ctx>> {
    let func_ = function.clone();
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
                    None => {
                        let blocks = func.get_basic_blocks();
                        for block in blocks {
                        }
                        // Check if it's and param if yes pass as arguments
                        let arg_i = func_.get_arg_index_(&arg.value);
                        if arg_i.is_some() {
                            let arg_i = arg_i.unwrap();
                            let params = func.get_nth_param(arg_i - 1);
                            if params.is_some() {
                                let param = params.unwrap();
                                let type_ = param.get_type();
                                match type_ {
                                    BasicTypeEnum::IntType(_) => {
                                        let param = param.into_int_value();
                                        args.push(param.into());
                                    }
                                    _ => {}
                                }
                            } else {
                                // Should technicly never happen
                                LOGGER.error(&"Something went wrong getting the argument of the function at known index.");
                                exit(1);
                            }
                        }
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

    return args;
}
