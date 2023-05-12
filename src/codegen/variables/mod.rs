use crate::{ast::function::Func, errors::logger::Log};
use colored::Colorize;
use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue};

use crate::ast::variable::{TypeVar, Variable};

use super::{
    get_args_function::Args, std_compiler::Std, validation::compare_args, CodeGen, LOGGER,
};

pub trait GenVar<'ctx> {
    fn gen_variable(&self, variable: &Variable, function: &Func, func: &FunctionValue<'ctx>);
}

trait Gen<'ctx> {
    fn gen_num(&self, num: &i32, variable: &Variable);
    fn gen_string(&self, string: &str, variable: &Variable);
    fn gen_call(&self, call: &Func, func: &FunctionValue<'ctx>, variable: &Variable);
    fn gen_assign_identifier(
        &self,
        id: &str,
        function: &Func,
        variable: &Variable,
        func: &FunctionValue<'ctx>,
    );
}

impl<'ctx> Gen<'ctx> for CodeGen<'ctx> {
    fn gen_num(&self, num: &i32, variable: &Variable) {
        let i32_type = self.context.i32_type();
        let var = self.builder.build_alloca(i32_type, &variable.name);
        self.builder
            .build_store(var, i32_type.const_int(*num as u64, false));
    }
    fn gen_string(&self, str: &str, variable: &Variable) {
        let arr = self.context.i8_type().array_type(str.len() as u32);
        let var = self.builder.build_alloca(arr, &variable.name);
        let arr_value = self.context.const_string(str.as_bytes(), false);
        let _ = self.builder.build_store(var, arr_value);
    }
    fn gen_call(&self, call: &Func, func: &FunctionValue<'ctx>, variable: &Variable) {
        let call_fn = self.module.get_function(&call.name);
        let fn_args = CodeGen::get_args_value(self, call, func);
        if call_fn.is_none() {
            let call_std = self.std_functions(call, fn_args, Some(&variable.name));
            if call_std.is_ok() {
                return;
            }
            LOGGER.display_error(&format!(
                "Found a call to a function that does not exist. {}",
                call.name
            ));
            return;
        }
        let call_fn = call_fn.unwrap();
        let args_fn = call_fn.get_params();
        let compare_args = compare_args(args_fn, fn_args.clone());
        if !compare_args {
            LOGGER.display_error(&format!(
                "Invalid arguments for function call to: {}",
                call.name
            ));
        }
        self.builder
            .build_call(call_fn, &fn_args.to_owned(), &variable.name);
    }
    fn gen_assign_identifier(
        &self,
        id: &str,
        function: &Func,
        variable: &Variable,
        func: &FunctionValue<'ctx>,
    ) {
        let item = func.get_first_basic_block();
        if let Some(block) = item {
            let var = block.get_instruction_with_name(id);
            match var {
                Some(ins) => {
                    let ins = ins.as_any_value_enum();
                    match ins {
                        AnyValueEnum::IntValue(int) => {
                            let value = self.context.i32_type();
                            let var = self.builder.build_alloca(value, &variable.name);
                            let _ = self.builder.build_store(var, int);
                        }
                        // Still kinda need to think about how I want to handle this
                        // &value  || something like that
                        // I am not sure yet
                        AnyValueEnum::PointerValue(pointer) => {
                            let _ = self.builder.build_load(pointer, &variable.name);
                        }
                        _ => (),
                    }
                }
                None => {
                    let arg = function.get_arg_index_(id);
                    if let Some(arg) = arg {
                        let func_arg = func.get_nth_param(arg);
                        if let Some(arg) = func_arg {
                            let arg = arg.as_any_value_enum();
                            match arg {
                                inkwell::values::AnyValueEnum::IntValue(value) => {
                                    let int = self.context.i32_type();
                                    let var = self.builder.build_alloca(int, &variable.name);
                                    let _ = &self.builder.build_store(var, value);
                                }
                                inkwell::values::AnyValueEnum::PointerValue(value) => {
                                    let _ = &self.builder.build_load(value, &variable.name);
                                }
                                _ => todo!("Not yet supported argument type"),
                            }
                        } else {
                            LOGGER.error(&"Found argument but not at index.");
                        }
                    } else {
                        LOGGER.error(&format!(
                            "tried assigning {}, to {}, but {} doesn't exist within this scope.",
                            id.blue().bold(),
                            variable.name.bold().yellow(),
                            id.blue().bold()
                        ));
                    }
                }
            }
        }
    }
}

impl<'ctx> GenVar<'ctx> for CodeGen<'ctx> {
    fn gen_variable(&self, variable: &Variable, function: &Func, func: &FunctionValue<'ctx>) {
        match &variable.type_ {
            // For arrays we need to make sure that all types within the array are the same type
            TypeVar::Arr { .. } => {}
            TypeVar::Number(value) => {
                self.gen_num(value, variable);
            }
            TypeVar::String(value) => self.gen_string(value, variable),
            TypeVar::Identifier(id) => self.gen_assign_identifier(id, function, variable, func),
            TypeVar::FunctionCall(call) => self.gen_call(call, func, variable),
            TypeVar::None => {}
        }
    }
}
