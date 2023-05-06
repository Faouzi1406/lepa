use inkwell::{basic_block::BasicBlock, types::BasicTypeEnum, values::FunctionValue};

use super::{CodeGen, Gen};
use crate::ast::{
    ast::{Case, Logic, Type},
    function::Func,
    variable::TypeVar,
};

pub trait GenLogicCase<'ctx> {
    // Todo: Add support for comparing variables && function calls
    // I need to add both support for it in the parser and the compiler
    fn gen_logic_case(
        &self,
        function: &Func,
        logic: &Logic,
        block: &BasicBlock,
        func: &FunctionValue<'ctx>,
    ) -> Result<(), String>;
}

impl<'ctx> GenLogicCase<'ctx> for CodeGen<'ctx> {
    // Todo: Updated it for multiple cases so this currently doesn't work
    fn gen_logic_case(
        &self,
        function: &Func,
        logic: &Logic,
        block: &BasicBlock,
        func: &FunctionValue<'ctx>,
    ) -> Result<(), String> {
        // match &logic.if_.get(0) {
        //     Case::None => {
        //         return Err("Found if without any comparisons!".into());
        //     }
        //     Case::EqEq(_, _) => {}
        //     Case::More(val1, val2) => {
        //         compile_compare_nums(&self, function, logic, block, func, "more", (val1, val2))?;
        //     }
        //     Case::MoreEq(val1, val2) => {
        //         compile_compare_nums(&self, function, logic, block, func, "more_eq", (val1, val2))?;
        //     }
        //     Case::Less(val1, val2) => {
        //         compile_compare_nums(&self, function, logic, block, func, "less", (val1, val2))?;
        //     }
        //     Case::LessEq(val1, val2) => {
        //         compile_compare_nums(&self, function, logic, block, func, "less_eq", (val1, val2))?;
        //     }
        // }
        return Ok(());
    }
}

pub fn compare_nums(num_1: i32, num_2: i32, case: &str) -> bool {
    match case {
        "more" => return num_1 > num_2,
        "less" => return num_1 < num_2,
        "more_eq" => return num_1 >= num_2,
        "less_eq" => return num_1 <= num_2,
        _ => false,
    }
}

pub fn compile_compare_nums<'ctx>(
    code: &CodeGen<'ctx>,
    function: &Func,
    logic: &Logic,
    block: &BasicBlock,
    func: &FunctionValue<'ctx>,
    case: &str,
    (val1, val2): (&TypeVar, &TypeVar),
) -> Result<(), String> {
    match (val1, val2) {
        // For static values we just replace the code with what it would do if comparison is true or false  
        (TypeVar::Number(num1), TypeVar::Number(num2)) => {
            if compare_nums(*num1, *num2, case) {
                match &logic.do_.type_ {
                    Type::Block => {
                        code.gen_block_func(function, logic.do_.body.clone(), func, block);
                    }
                    t => {
                        return Err(format!(
                            "Found invalid token type after comparison, {:#?}",
                            t
                        ))
                    }
                }
            } else {
                if logic.else_.is_some() {
                    let else_ = logic.else_.clone();
                    if else_.is_some() {
                        let else_ = else_.unwrap();
                        match else_.type_ {
                            Type::Block => {
                                code.gen_block_func(function, else_.body, func, block);
                            }
                            t => {
                                return Err(format!(
                                    "Found invalid token type after comparison, {:#?}",
                                    t
                                ))
                            }
                        }
                    }
                }
            }
        }
        // Todo:  Handle identifiers for if statements
        (TypeVar::Identifier(id_1), TypeVar::Identifier(id_2)) => {}
        case => {
            return Err(format!(
                "Cannot check if {:#?} is more then {:#?}",
                case.0, case.1
            ))
        }
    }
    return Ok(());
}
