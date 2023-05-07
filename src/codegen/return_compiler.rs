use inkwell::values::{AnyValue, AnyValueEnum, FunctionValue};

use super::CodeGen;
use crate::ast::ast::{Return, ReturnTypes};
use crate::ast::function::Func;

pub trait GenReturn {
    fn gen_return(
        &self,
        return_type: Return,
        function: &Func,
        func: &FunctionValue,
    ) -> Result<(), String>;
}

trait GenReturnTypes {
    fn gen_number(&self, return_type: Return);
    fn gen_id(&self, return_type: Return, function: &Func, func: &FunctionValue);
    fn gen_string(&self, return_type: Return);
}

impl<'ctx> GenReturnTypes for CodeGen<'ctx> {
    fn gen_number(&self, return_type: Return) {
        let num: i32 = return_type.value.parse().unwrap();
        let var = self.context.i32_type();
        let num = var.const_int(num as u64, false);
        self.builder.build_return(Some(&num));
    }
    // Todo: Local variables
    fn gen_id(&self, return_type: Return, function: &Func, func: &FunctionValue) {
        let val = self.module.get_global(&return_type.value);

        if val.is_some() {
            let val = val.unwrap();
            let val = val.as_pointer_value();
            let load = &self.builder.build_load(val, &function.name);
            self.builder.build_return(Some(load));
        }

        let local = func.get_first_basic_block();
        if let Some(local) = local {
            let local_item = local.get_instruction_with_name(&return_type.value);
            if let Some(local) = local_item {
                let val = local.as_any_value_enum();
                println!("{:#?}", val);
                match val {
                    AnyValueEnum::IntValue(int) => {
                        self.builder.build_return(Some(&int));
                        return;
                    }
                    AnyValueEnum::PointerValue(val) => {
                        let load = &self.builder.build_load(val, &return_type.value);
                        self.builder.build_return(Some(load));
                        return;
                    }
                    _ => return,
                }
            }
        }

        let param_ = function.get_arg_index_(&return_type.value);
        if param_.is_some() {
            let val = func.get_nth_param(param_.unwrap());
            if val.is_some() {
                self.builder.build_return(Some(&val.unwrap()));
            }
        }
    }
    fn gen_string(&self, _return_type: Return) {
        todo!("Generating strings as return types is currently not supported")
    }
}

impl<'ctx> GenReturn for CodeGen<'ctx> {
    fn gen_return(
        &self,
        return_type: Return,
        function: &Func,
        func: &FunctionValue,
    ) -> Result<(), String> {
        match return_type.type_ {
            ReturnTypes::Number => {
                let _ = &self.gen_number(return_type);
            }
            ReturnTypes::Identifier => {
                let _ = &self.gen_id(return_type, function, func);
            }
            ReturnTypes::String => {
                let _ = &self.gen_string(return_type);
            }
            ReturnTypes::None => {
                self.builder.build_return(None);
            }
        }

        return Ok(());
    }
}
