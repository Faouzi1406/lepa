use crate::ast::{
    ast::{Ast, Type},
    function::Func,
};

use super::DetectUnused;

pub struct DetectedFunc<'a> {
    pub block: &'a Ast,
    pub func: &'a Func,
}

impl<'a> DetectedFunc<'a> {
    pub fn new(func: &'a Func, block: &'a Ast) -> DetectedFunc<'a> {
        return DetectedFunc { func, block };
    }
}

pub trait DetectUnusedFunc<'a> {
    fn detect_func(&self) -> Vec<DetectedFunc<'a>>;
    //fn detect_unused_funcs(&self) -> Vec<UnusedValues>;
}

impl<'a> DetectUnusedFunc<'a> for DetectUnused<'a> {
    fn detect_func(&self) -> Vec<DetectedFunc<'a>> {
        let mut vec_detected = Vec::new();
        let in_ = &self.ast;
        for token in &self.ast.body {
            match &token.type_ {
                Type::Function(func) => {
                    vec_detected.push(DetectedFunc::new(&func, in_));
                }
                Type::Block => {
                    let detector = DetectUnused::new(in_);
                    let mut detected = detector.detect_func();
                    vec_detected.append(&mut detected);
                }
                _ => (),
            }
        }
        vec_detected
    }
    //fn detect_unused_funcs(&self) -> Vec<UnusedValues> {}
}
