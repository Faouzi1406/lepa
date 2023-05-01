use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use inkwell::{module::Linkage, values::BasicMetadataValueEnum, AddressSpace};

use crate::{
    ast::{ast::TypesArg, function::Func},
    errors::logger::Log,
};

use super::{CodeGen, LOGGER};

pub trait Std {
    /// Std functions
    ///
    /// Matches a function against a existing compiler implementation of the function;
    ///
    /// print("compile me :)"); // Will get detected by the std_compiler and turned into it's
    /// llvm ir/ assembly implementation of the function
    ///
    /// These functions are usually not touched by the user and have a wrapper implemented in the
    /// std of the language, however there has to be a std before that is possible.
    fn std_functions(
        &self,
        func: &Func,
        args_values: Vec<BasicMetadataValueEnum>,
    ) -> Result<(), String>;
    fn std_print_f(&self, func: &Func, args_values: Vec<BasicMetadataValueEnum>);
}

impl<'ctx> Std for CodeGen<'ctx> {
    fn std_functions(
        &self,
        func: &Func,
        args_values: Vec<BasicMetadataValueEnum>,
    ) -> Result<(), String> {
        match func.name.as_str() {
            "printf" => self.std_print_f(func, args_values),
            name => {
                return Err(format!(
                    "Got a std_function call but the function {}; is not a std function",
                    name
                ))
            }
        }
        return Ok(());
    }
    fn std_print_f(&self, func: &Func, args_values: Vec<BasicMetadataValueEnum>) {
        let Some(first_arg) = func.args.get(0) else {
            LOGGER.error(&"Printf expected a value but got none.");
            return;
        };
        match &first_arg.type_ {
            TypesArg::String => {
                let printf = &self.module.get_function("printf");
                let rand_name: f64 = rand::random();
                println!("random: {rand_name}");

                let value = self
                    .builder
                    .build_global_string_ptr(&first_arg.value, &format!("print_str{rand_name}",));

                if printf.is_some() {
                    let _ = &self.builder.build_call(
                        printf.unwrap(),
                        &[value.as_pointer_value().into()],
                        "printf_call",
                    );
                    return;
                }

                let print_f = &self.context.i32_type().fn_type(
                    &[self
                        .context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .into()],
                    true,
                );

                let printf = &self
                    .module
                    .add_function("printf", *print_f, Some(Linkage::External));

                let _ = &self.builder.build_call(
                    *printf,
                    &[value.as_pointer_value().into()],
                    "printf_call",
                );
            }
            value => {
                LOGGER.error(&format!(
                    "Printf a string as first value but got. {:#?}",
                    value
                ));
                return;
            }
        }
    }
}
