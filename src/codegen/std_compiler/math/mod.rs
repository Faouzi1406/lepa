//! These are all the standard library implementations for mathemathical functions / calculations
//!
//! Altough currently I haven't added suport for directly doing something like:
//!
//! let a = b + c;
//!
//! This will of course be supported in the future.  
//!
//! Currently still have to add the parsing for it so it will take some time to add this.
//!
//! Current supported
//!
//! @add(a, b)  ->  let wow = a + b;  

use std::{process::exit, fmt::Debug};

use colored::Colorize;
use inkwell::values::{BasicMetadataValueEnum, AnyValue, AnyValueEnum};

use crate::{
    ast::function::Func,
    codegen::{CodeGen, LOGGER},
    errors::logger::Log,
};

pub trait StdMath {
    fn std_add(&self, args_values: Vec<BasicMetadataValueEnum>, func: &Func, var_name: &str);
}

pub trait StdMathErrors {
    /// The std math error for when the expected amount of argument was not met because it got to
    /// many arguments. 
    ///
    /// # Example 
    ///
    /// @add(1, 2, 3): 
    ///
    /// Since add expects 2 arguments but got 3 the error will look something like:
    /// Got to many arguments for function: @add, expected 2 arguments but got 3, on line 10, consider removing the extra arguments.
    fn to_many_args(&self, expected: usize, got: usize, fn_name: &str, line: usize);
    /// The std math error for when the expected amount of argument was not met because it didn't
    /// get enough errors. 
    ///
    /// # Example 
    ///
    /// @add(1): 
    ///
    /// Since add expects 2 arguments but 1 the error will look something like:
    ///
    /// @add didn't get enough arguments: expected 2 arguments but got 1, on line 10, consider adding the extra arguments.
    fn not_enough_args(&self, expected: usize, got: usize, fn_name: &str, line: usize);
    fn incorrect_type<T: Debug>(&self, expected_type: T, received_type: T, fn_name: &str, line: usize); 
}

impl<'ctx> StdMath for CodeGen<'ctx> {
    fn std_add(&self, args_values: Vec<BasicMetadataValueEnum>, func: &Func, var_name: &str) {
        // Since we expect 2 arguments we make sure that we got 2 arguments;
        if args_values.len() > 2 { self.to_many_args(2, args_values.len(), "@add", func.line) };
        if args_values.len() < 2 { self.not_enough_args(2, args_values.len(), "@add", func.line) };
        
        let (add1, add2) = (args_values.get(0).unwrap(), args_values.get(1).unwrap()) ;
        let (add1_value, add2_value) = (add1.as_any_value_enum(), add2.as_any_value_enum());

        match (add1_value, add2_value) {
            (AnyValueEnum::IntValue(add1_value), AnyValueEnum::IntValue(add2_value)) => {
                let add = self.builder.build_int_add(add1_value, add2_value, var_name);
                let i32_type = self.context.i32_type();
                let assign = self.builder.build_alloca(i32_type, var_name);
                self.builder.build_store(assign, add);
            }
            // Todo: We should throw a incorrect type value here. 
            (val1, val2) => (),
        }
        
    }
}

impl<'ctx> StdMathErrors for CodeGen<'ctx> {
    fn to_many_args(&self, expected: usize, got: usize, fn_name: &str, line:usize) {
        let error = format!(
            "Got to many arguments for function: {}, expected {} arguments but got {}, on line {}, consider removing the extra arguments.",
            fn_name.blue().bold(),
            expected,
            got, 
            line
        );
        LOGGER.display_error(&error);
        exit(1);
    }
    fn not_enough_args(&self, expected: usize, got: usize, fn_name: &str, line: usize) {
        let error = format!(
            "{} didn't get enough arguments: expected {} arguments but got {}, on line {}, consider adding the extra arguments.",
            fn_name.blue().bold(),
            expected,
            got, 
            line
        );
        LOGGER.display_error(&error);
        exit(1);
    }
    fn incorrect_type<T: Debug>(&self, expected_type: T, received_type: T, fn_name: &str, line: usize) {
        let error = format!(
            "{} didn't get the correct types: expected {:#?} arguments but got {:#?}, on line {}, consider changing it to the correct type.",
            fn_name.blue().bold(),
            expected_type,
            received_type, 
            line
        );
        LOGGER.display_error(&error);
        exit(1);
    }
}
