use super::{CodeGen, LOGGER};
use crate::{
    ast::{ast::TypesArg, function::Func},
    errors::logger::Log,
};
use inkwell::{
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, AnyValue, AnyValueEnum},
};

pub fn get_int_value(value: BasicValueEnum) -> Option<inkwell::values::IntValue> {
    match value.get_type() {
        inkwell::types::BasicTypeEnum::IntType(_) => return Some(value.into_int_value()),
        _ => None,
    }
}

pub trait Args<'ctx> {
    fn get_args_value(
        &self,
        function: &Func,
        func: &FunctionValue<'ctx>,
    ) -> Vec<BasicMetadataValueEnum<'ctx>>;
}

impl<'ctx> Args<'ctx> for CodeGen<'ctx> {
    fn get_args_value(
        &self,
        function: &Func,
        func: &FunctionValue<'ctx>,
    ) -> Vec<BasicMetadataValueEnum<'ctx>> {
        let func_ = function.clone();
        let vars = &function.args;
        let mut args = Vec::new();

        for arg in vars {
            match arg.type_ {
                TypesArg::None => {
                    let get_var = self.module.get_global(&arg.value);
                    match get_var {
                        Some(value) => {
                            let value = value.get_initializer();
                            match value {
                                Some(value) => {
                                    let int_value = get_int_value(value);
                                    if int_value.is_some() {
                                        args.push(int_value.unwrap().into());
                                    }
                                }
                                None => {}
                            }
                        }
                        None => {
                            // Todo: check for value in instructions
                            let blocks = func.get_basic_blocks();
                            let block = blocks.first().unwrap();

                            let value = block.get_instruction_with_name(&arg.value);
                            if value.is_some() {
                                let value = value.unwrap().as_any_value_enum();
                                match value {
                                    AnyValueEnum::IntValue(val) => {
                                        args.push(val.into());
                                    }
                                    AnyValueEnum::PointerValue(val) => {
                                        let load  = self.builder.build_load(val, &arg.value);
                                        args.push(load.into());
                                    }
                                    val => println!("I am... {val:#?}")
                                }
                                continue;
                            }
                            // Check if it's and param if true pass as arguments
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
                                    LOGGER.error(&"Something went wrong getting the argument of the function at known index.");
                                }
                            }
                        }
                    }
                }
                TypesArg::Number => {
                    let value = self.context.i32_type();
                    let int = value.const_int(arg.value.parse().unwrap(), false);
                    args.push(int.into());
                }
                TypesArg::String => {
                    let rand_name: f64 = rand::random();
                    let value = self
                        .builder
                        .build_global_string_ptr(&arg.value, &format!("str_argument{rand_name}",));
                    args.push(value.as_pointer_value().into());
                }
            }
        }

        return args;
    }
}
