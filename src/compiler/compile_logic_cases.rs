use inkwell::{basic_block::BasicBlock, values::FunctionValue};

use super::{CodeGen, Gen};
use crate::ast::{
    ast::{Case, Logic, Type},
    function::Func,
    variable::TypeVar,
};

// Todo: Add support for comparing variables && function calls
// I need to add both support for it in the parser and the compiler
pub fn compile_logic_case(
    code: &CodeGen,
    function: &Func,
    logic: &Logic,
    block: &BasicBlock,
    func: &FunctionValue,
) -> Result<(), String> {
    match &logic.if_ {
        Case::None => {
            return Err("Found if without any comparisons!".into());
        }
        Case::EqEq(val1, val2) => {}
        Case::More(val1, val2) => {
            compile_compare_nums(code, function, logic, block, func, "more", (val1, val2))?;
        }
        Case::MoreEq(val1, val2) => {
            compile_compare_nums(code, function, logic, block, func, "more_eq", (val1, val2))?;
        }
        Case::Less(val1, val2) => {
            compile_compare_nums(code, function, logic, block, func, "less", (val1, val2))?;
        }
        Case::LessEq(val1, val2) => {
            compile_compare_nums(code, function, logic, block, func, "less_eq", (val1, val2))?;
        }
    }
    return Ok(());
}

pub fn compare_nums(num_1:i32, num_2:i32,case:&str) -> bool {
    match case {
        "more" => return num_1 > num_2,
        "less" => return num_1 < num_2,
        "more_eq" => return num_1 >= num_2,
        "less_eq" => return num_1 <= num_2,
        _ => false
    }
}

pub fn compile_compare_nums(
    code: &CodeGen,
    function: &Func,
    logic: &Logic,
    block: &BasicBlock,
    func: &FunctionValue,
    case:&str,
    (val1, val2): (&TypeVar, &TypeVar),
) -> Result<(), String> {
    match (val1, val2) {
        (TypeVar::Number(num1), TypeVar::Number(num2)) => {
            // For static values we don't really add the comparison in the code we can just
            // check it here and add it if the case is true.
            if compare_nums(*num1, *num2, case) {
                println!("logic: {:#?}", logic);
                println!("TYEPEEPEPEP {:#?}", num1);
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
        case => return Err(format!("Cannot check if {:#?} is more then {:#?}", case.0, case.1)),
    }
    return Ok(());
}
