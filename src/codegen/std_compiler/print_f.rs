use inkwell::{module::Linkage, values::BasicMetadataValueEnum, AddressSpace};

use crate::{
    ast::{ast::TypesArg, function::Func},
    codegen::{CodeGen, LOGGER},
    errors::logger::Log,
};

pub trait StdPrint {
    fn std_print_f(&self, func: &Func, args_values: Vec<BasicMetadataValueEnum>);
}

impl<'ctx> StdPrint for CodeGen<'ctx> {
    fn std_print_f(&self, func: &Func, args_values: Vec<BasicMetadataValueEnum>) {
        let Some(first_arg) = func.args.get(0) else {
            LOGGER.error(&"Printf expected a value but got none.");
            return;
        };

        match &first_arg.type_ {
            TypesArg::String => {
                let printf = &self.module.get_function("printf");

                if printf.is_some() {
                    let _ = &self.builder.build_call(
                        printf.unwrap(),
                        &args_values.clone(),
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

                let _ = &self
                    .builder
                    .build_call(*printf, &args_values, "printf_call");
            }
            value => {
                LOGGER.error(&format!(
                    "Printf a string as first value but got. {:#?}",
                    value
                ));
            }
        }
    }
}
