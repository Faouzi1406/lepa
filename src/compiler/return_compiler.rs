use inkwell::values::FunctionValue;

use crate::ast::{Func, Return, ReturnTypes};

use super::CodeGen;

pub fn return_type_build(
    code: &CodeGen,
    return_type: Return,
    function: &Func,
    func: &FunctionValue,
) -> Result<(), String> {
    match return_type.type_ {
        ReturnTypes::Number => {
            let num: i32 = return_type.value.parse().unwrap();
            let var = code.context.i32_type();
            let num = var.const_int(num as u64, false);
            code.builder.build_return(Some(&num));
        }
        ReturnTypes::Identifier => {
            let val = code.module.get_global(&return_type.value);
            if val.is_some() {
                let val = val.unwrap();
                code.builder.build_return(Some(&val));
            }
            let param_ = function.get_arg_index_(&return_type.value);
            if param_.is_some() {
                let val = func.get_nth_param(param_.unwrap());
                if val.is_some() {
                    code.builder.build_return(Some(&val.unwrap()));
                }
            }
        }
        ReturnTypes::String => {
            return Err("Not yet supported return type..".to_string());
        }
        ReturnTypes::None => {
            code.builder.build_return(None);
        }
    }

    return Ok(());
}
