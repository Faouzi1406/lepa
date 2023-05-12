use crate::ast::{ast::Type, function::Func, variable::TypeVar};

use super::{DetectUnused, UnusedValues};

#[derive(Debug, PartialEq)]
pub struct DetectedFunc<'a> {
    pub func: &'a Func,
}

impl<'a> DetectedFunc<'a> {
    pub fn new(func: &'a Func) -> DetectedFunc<'a> {
        DetectedFunc { func }
    }
}

pub trait DetectUnusedFunc<'a> {
    fn detect_func(&self) -> Vec<DetectedFunc<'a>>;
    // returns a vec with names of all used funcs
    fn detect_used_funcs(&self) -> Vec<&'a String>;
    fn detect_unused_funcs(&self) -> Vec<UnusedValues<'a>>;
}

impl<'a> DetectUnusedFunc<'a> for DetectUnused<'a> {
    fn detect_func(&self) -> Vec<DetectedFunc<'a>> {
        let mut found_funcs = Vec::new();
        for token in &self.ast.body {
            match &token.type_ {
                Type::Function(func) => {
                    found_funcs.push(DetectedFunc::new(func));
                }
                Type::Block => {
                    let detector = DetectUnused::new(token);
                    let mut detected = detector.detect_func();
                    found_funcs.append(&mut detected);
                }
                _ => (),
            }
        }
        found_funcs
    }
    fn detect_used_funcs(&self) -> Vec<&'a String> {
        let mut found_funcs: Vec<&'a String> = Vec::new();
        for token in &self.ast.body {
            match &token.type_ {
                Type::Variable(var) => {
                    if let TypeVar::FunctionCall(call) = &var.type_ {
                        let get_found = found_funcs.iter().find(|x| **x == &call.name);
                        if get_found.is_none() {
                            found_funcs.push(&call.name);
                        }
                    }
                }
                Type::FunctionCall(call) => {
                    let get_found = found_funcs.iter().find(|x| **x == &call.name);
                    if get_found.is_none() {
                        found_funcs.push(&call.name);
                    }
                }
                Type::Function(func) => {
                    if func.body.is_some() {
                        let detector = DetectUnused::new(func.body.as_ref().unwrap());
                        let mut detected = detector.detect_used_funcs();
                        found_funcs.append(&mut detected);
                    }
                }
                Type::Block => {
                    let detector = DetectUnused::new(token);
                    let mut detected = detector.detect_used_funcs();
                    found_funcs.append(&mut detected);
                }
                _ => (),
            }
        }
        found_funcs
    }
    fn detect_unused_funcs(&self) -> Vec<UnusedValues<'a>> {
        let mut unused = Vec::new();
        let funcs = self.detect_func();
        let used = self.detect_used_funcs();
        for func in funcs {
            match func.func.name.as_str() {
                "main" => {
                    continue;
                }
                name => {
                    if !used.contains(&&(name.to_string())) {
                        unused.push(UnusedValues::Function(func.func));
                    }
                }
            }
        }
        unused
    }
}
