// /Rust being a bit not smart
#![allow(unused_assignments)]

use super::{DetectUnused, UnusedValues};
use crate::ast::{
    ast::{Arg, Ast, Type},
    variable::TypeVar,
};

#[derive(Debug)]
pub struct DetectedArg<'a> {
    func_name: &'a str,
    arg: &'a Arg,
    block: &'a Ast,
}

pub trait DetectUnusedArgs<'a> {
    fn detect_arg(&self) -> Vec<DetectedArg<'a>>;
    fn detect_unused_args(&self) -> Vec<UnusedValues<'a>>;
    fn detect_unused_args_block(block: &'a Ast, value: &str) -> bool {
        for token in &block.body {
            match &token.type_ {
                Type::Variable(var) => {
                    if var.type_.uses(value) {
                        return false;
                    }
                }
                Type::FunctionCall(call) => {
                    if call.args_uses(value) {
                        return true;
                    }
                }
                Type::Return(ret) => {
                    if ret.value == value {
                        return false;
                    }
                }
                _ => continue,
            }
        }
        true
    }
}

impl<'a> DetectedArg<'a> {
    pub fn new(arg: &'a Arg, block: &'a Ast, func_name: &'a str) -> DetectedArg<'a> {
        Self {
            func_name,
            arg,
            block,
        }
    }
    pub fn multiple(
        args_: &'a Vec<Arg>,
        block: &'a Ast,
        func_name: &'a str,
    ) -> Vec<DetectedArg<'a>> {
        let mut args = Vec::new();
        for arg in args_ {
            let unused = DetectedArg::new(arg, block, func_name);
            args.push(unused);
        }
        args
    }
}

impl<'a> DetectUnusedArgs<'a> for DetectUnused<'a> {
    fn detect_arg(&self) -> Vec<DetectedArg<'a>> {
        let mut vec = Vec::new();
        for token in &self.ast.body {
            match &token.type_ {
                Type::Function(func) => {
                    if func.body.is_some() {
                        vec.append(&mut DetectedArg::multiple(
                            &func.args,
                            func.body.as_ref().unwrap(),
                            &func.name,
                        ));
                    }
                }
                _ => (),
            }
        }
        vec
    }
    fn detect_unused_args(&self) -> Vec<UnusedValues<'a>> {
        let get_all_args = &self.detect_arg();
        let mut vec_unused = Vec::new();
        for item in get_all_args {
            let mut used = false;
            match &item.block.type_ {
                Type::FunctionCall(call) => {
                    if call.args_uses(&item.arg.value) {
                        used = true;
                    }
                }
                Type::Variable(var) => match &var.type_ {
                    TypeVar::Identifier(id) => {
                        if id == &item.arg.value {
                            used = true;
                        }
                    }
                    TypeVar::FunctionCall(call) => {
                        if call.args_uses(&item.arg.value) {
                            used = true;
                        }
                    }
                    _ => (),
                },
                Type::Function(func) => {
                    if let Some(body) = &func.body {
                        if !DetectUnused::detect_unused_args_block(body, &item.arg.value) {
                            used = true;
                        }
                    }
                }
                Type::Block => {
                    if !DetectUnused::detect_unused_args_block(item.block, &item.arg.value) {
                        used = true;
                    }
                }
                _ => (),
            }
            if !used {
                vec_unused.push(UnusedValues::Argument(item.arg, item.func_name));
            }
        }
        vec_unused
    }
}
