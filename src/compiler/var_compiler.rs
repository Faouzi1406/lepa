use inkwell::values::FunctionValue;

use crate::ast::ast::{TypeVar, Variable};

use super::CodeGen;

pub fn compile_var_func(code: &CodeGen, variable: Variable, func: &FunctionValue) {
    match variable.type_ {
        TypeVar::Number(num) => {
            let i32_type= code.context.i32_type();
            let var = code.builder.build_alloca(i32_type, &variable.name);
            code.builder.build_store(var, i32_type.const_int(num as u64, false));
        }
        _ => (),
    }
}
