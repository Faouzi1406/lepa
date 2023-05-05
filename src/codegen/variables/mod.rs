use crate::{ast::function::Func, errors::logger::Log};
use inkwell::values::FunctionValue;

use crate::ast::variable::{TypeVar, Variable};

use super::{get_args_function::Args, validation::compare_args, CodeGen, LOGGER};

pub trait GenVar<'ctx> {
    fn gen_variable(&self, function: &Func, variable: &Variable, func: &FunctionValue<'ctx>);
}

trait Gen<'ctx> {
    fn gen_num(&self, num: &i32, variable: &Variable);
    fn gen_string(&self, string: &str, variable: &Variable);
    fn gen_call(
        &self,
        call: &Func,
        function: &Func,
        func: &FunctionValue<'ctx>,
        variable: &Variable,
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
    fn gen_call(
        &self,
        call: &Func,
        function: &Func,
        func: &FunctionValue<'ctx>,
        variable: &Variable,
    ) {
        let call_fn = self.module.get_function(&call.name);
        if call_fn.is_none() {
            LOGGER.display_error(&format!(
                "Found a call to a function that does not exist. {}",
                call.name
            ));
        }
        let call_fn = call_fn.unwrap();
        let fn_args = CodeGen::get_args_value(&self, function, func);
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
}

impl<'ctx> GenVar<'ctx> for CodeGen<'ctx> {
    fn gen_variable(&self, function: &Func, variable: &Variable, func: &FunctionValue<'ctx>) {
        match &variable.type_ {
            TypeVar::Arr { .. } => {
                //Todo: Not yet suported
            }
            TypeVar::Number(value) => {
                self.gen_num(value, &variable);
            }
            TypeVar::String(value) => self.gen_string(&value, variable),
            TypeVar::Identifier(_) => {}
            TypeVar::FunctionCall(call) => self.gen_call(call, function, func, variable),
            TypeVar::None => {}
        }
    }
}
