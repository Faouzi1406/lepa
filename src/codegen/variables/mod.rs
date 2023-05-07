use crate::{ast::function::Func, errors::logger::Log};
use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue};

use crate::ast::variable::{TypeVar, Variable};

use super::{get_args_function::Args, validation::compare_args, CodeGen, LOGGER};

pub trait GenVar<'ctx> {
    fn gen_variable(&self, variable: &Variable, func: &FunctionValue<'ctx>);
}

trait Gen<'ctx> {
    fn gen_num(&self, num: &i32, variable: &Variable);
    fn gen_string(&self, string: &str, variable: &Variable);
    fn gen_call(&self, call: &Func, func: &FunctionValue<'ctx>, variable: &Variable);
    fn gen_assign_identifier(&self, id: &str, variable: &Variable, func: &FunctionValue<'ctx>);
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
        if call_fn.is_none() {
            LOGGER.display_error(&format!(
                "Found a call to a function that does not exist. {}",
                call.name
            ));
        }
        let call_fn = call_fn.unwrap();
        let fn_args = CodeGen::get_args_value(&self, call, func);
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
    fn gen_assign_identifier(&self, id: &str, variable: &Variable, func: &FunctionValue<'ctx>) {
        let item = func.get_first_basic_block();
        match item {
            Some(block) => {
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
                    None => (),
                }
            }
            None => {}
        }
    }
}

impl<'ctx> GenVar<'ctx> for CodeGen<'ctx> {
    fn gen_variable(&self, variable: &Variable, func: &FunctionValue<'ctx>) {
        match &variable.type_ {
            // For arrays we need to make sure that all types within the array are the same type
            TypeVar::Arr { .. } => {}
            TypeVar::Number(value) => {
                self.gen_num(value, &variable);
            }
            TypeVar::String(value) => self.gen_string(&value, variable),
            TypeVar::Identifier(id) => self.gen_assign_identifier(id, variable, func),
            TypeVar::FunctionCall(call) => self.gen_call(call, func, variable),
            TypeVar::None => {}
        }
    }
}
