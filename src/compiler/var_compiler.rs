use std::process::exit;

use inkwell::values::FunctionValue;

use crate::{ast::variable::TypeVar, compiler::Variable, errors::logger::Log};

use super::{get_args_function::get_args_value, validation::compare_args, CodeGen, LOGGER};

pub fn compile_var_func<'ctx>(code: &CodeGen<'ctx>, variable: Variable, func: &FunctionValue<'ctx>) {
    match variable.type_ {
        TypeVar::Number(num) => {
            let i32_type = code.context.i32_type();
            let var = code.builder.build_alloca(i32_type, &variable.name);
            code.builder
                .build_store(var, i32_type.const_int(num as u64, false));
        }
        // Todo: Going to add helper functions for string into the compiler and std library
        TypeVar::String(str) => {
            let arr = code.context.i8_type().array_type(str.len() as u32);
            let var = code.builder.build_alloca(arr, &variable.name);
            let arr_value = code.context.const_string(str.as_bytes(), false);
            let _ = code.builder.build_store(var, arr_value);
        }
        TypeVar::FunctionCall(call) => {
            let call_fn = code.module.get_function(&call.name);
            if call_fn.is_none() {
                LOGGER.display_error(&format!(
                    "Found a call to a function that does not exist. {}",
                    call.name
                ));
                exit(1);
            }
            let call_fn = call_fn.unwrap();
            let fn_args = get_args_value(code, &call, func).to_owned();
            let args_fn = call_fn.get_params();
            let compare_args = compare_args(args_fn, fn_args.clone());
            if !compare_args {
                LOGGER.display_error(&format!(
                    "Invalid arguments for function call to: {}",
                    call.name
                ));
                exit(1);
            }
            code.builder.build_call(call_fn, &fn_args, &variable.name);
        }
        _ => (),
    }
}
