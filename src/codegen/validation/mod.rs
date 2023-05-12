use inkwell::{
    types::BasicTypeEnum,
    values::{BasicMetadataValueEnum, BasicValueEnum},
};

pub fn compare_args(args_fn: Vec<BasicValueEnum>, args_input: Vec<BasicMetadataValueEnum>) -> bool {
    if args_fn.len() != args_input.len() {
        return false;
    }

    for (i, arg) in args_fn.iter().enumerate() {
        let arg_input = args_input[i];
        let arg_type = arg.get_type();

        match arg_type {
            BasicTypeEnum::IntType(_) => {
                if !arg_input.is_int_value() {
                    return false;
                }
            }
            BasicTypeEnum::ArrayType(_) => {
                if !arg_input.is_array_value() {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}
