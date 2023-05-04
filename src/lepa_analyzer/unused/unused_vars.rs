use std::ops::Deref;

use crate::ast::ast::Case;
use crate::ast::ast::Type;
use crate::ast::ast::TypesArg;
use crate::ast::variable::TypeVar;
use crate::lepa_analyzer::unused::DetectUnused;
use crate::lepa_analyzer::unused::UnusedValues;
use crate::lepa_analyzer::Variable;
use crate::ast::ast::Ast;

#[derive(Debug)]
pub struct DetectedVar {
    pub block: Ast,
    pub var: Variable,
}

impl DetectedVar {
    fn new(block: Ast, var: Variable) -> DetectedVar {
        return DetectedVar { block, var };
    }
}

pub trait DetectorVars {
    fn detect_vars(&self) -> Vec<DetectedVar>;
    fn detect_unused_vars(&self) -> Vec<UnusedValues>;
    fn detect_used_in_block(&self, value:&str) -> bool;
}

impl DetectorVars for DetectUnused {
    fn detect_vars(&self) -> Vec<DetectedVar> {
        let mut vars = Vec::new();
        let block = &self.ast;

        for token in &self.ast.body {
            match &token.type_ {
                Type::Variable(var) => {
                    vars.push(DetectedVar::new(block.clone(), var.clone()));
                }
                Type::Function(func) => match &func.body {
                    Some(body) => {
                        let func = DetectUnused::new(*body.clone());
                        let mut func_vars = func.detect_vars();
                        vars.append(&mut func_vars);
                    }
                    None => (),
                },
                Type::Block => {
                    let func = DetectUnused::new(token.clone());
                    let mut func_vars = func.detect_vars();
                    vars.append(&mut func_vars);
                }
                Type::Logic(logic) => {
                    let logic_if = DetectUnused::new(*logic.do_.clone());
                    let mut logic_vars_if = logic_if.detect_vars();
                    vars.append(&mut logic_vars_if);

                    if logic.else_.is_some() {
                        let else_ = DetectUnused::new(*logic.else_.clone().unwrap());
                        let mut else_vars = else_.detect_vars();
                        vars.append(&mut else_vars);
                    }

                    
                }
                _ => (),
            }
        }
        vars
    }

    fn detect_unused_vars(&self) -> Vec<UnusedValues> {
        let detect_vars = &self.detect_vars();
        let mut unused_vars = Vec::new();
        for detected_var in detect_vars {
            let mut used = false;
            for token in &detected_var.block.body {
                match &token.type_ {
                    Type::Variable(var) => match &var.type_ {
                        TypeVar::Identifier(id) => {
                            if *id == detected_var.var.name {
                                used = true;
                                break;
                            }
                        }
                        _ => (),
                    },
                    Type::Block => {
                        let func = DetectUnused::new(token.clone());
                        if func.detect_used_in_block(&detected_var.var.name) {used = true; break;};
                    }
                    Type::Function(_)=> {
                        let func = DetectUnused::new(token.clone());
                        if func.detect_used_in_block(&detected_var.var.name) {used = true; break;};
                    }
                    Type::Logic(logic) => match &logic.if_ {
                        Case::EqEq(val1, val2) => {
                            let false = val1.uses(&detected_var.var.name) else {
                                used = true; 
                                break;
                            };
                            let false = val2.uses(&detected_var.var.name) else {
                                used = true; 
                                break;
                            };

                            let block = DetectUnused::new(*logic.do_.clone());
                            if block.detect_used_in_block(&detected_var.var.name) {used = true; break;};

                            if logic.else_.is_some() {
                                let block = DetectUnused::new(*logic.else_.clone().unwrap());
                                if block.detect_used_in_block(&detected_var.var.name) {used = true; break;};
                            }
                        }
                        _ => (),
                    },
                    Type::FunctionCall(call) => {
                            if call.args_uses(&detected_var.var.name) {used = true; break};
                    }
                    _ => continue,
                }
            }
            if !used {
                unused_vars.push(UnusedValues::Variable(detected_var.var.to_owned()));
            }
        }
        unused_vars
    }

    fn detect_used_in_block(&self, value:&str) -> bool {
        for token in &self.ast.body {
           match &token.type_ {
                Type::Variable(var) => {
                    match &var.type_ {
                        TypeVar::Identifier(id) if id == value => {
                            return true;
                        }
                        _ => continue
                    }
                }
                Type::Function(func) => {
                    for arg in &func.args {
                        if arg.type_ == TypesArg::None {
                            if arg.value == value {
                                return true;
                            }
                        }
                    }
                    if func.body.is_some() {
                        let body = func.body.as_ref().unwrap();
                        let body = DetectUnused::new(*body.clone());
                        return body.detect_used_in_block(value)
                    }
                }
                Type::Block => {
                    let body = DetectUnused::new(token.clone());
                    return body.detect_used_in_block(value);
                }
                _ => continue
            }
        }
        return false;
    }
}
