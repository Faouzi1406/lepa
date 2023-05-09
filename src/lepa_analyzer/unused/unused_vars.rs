use crate::ast::ast::Ast;
use crate::ast::ast::Case;
use crate::ast::ast::ReturnTypes;
use crate::ast::ast::Type;
use crate::ast::ast::TypesArg;
use crate::ast::variable::TypeVar;
use crate::lepa_analyzer::unused::DetectUnused;
use crate::lepa_analyzer::unused::UnusedValues;
use crate::lepa_analyzer::Variable;

#[derive(Debug)]
pub struct DetectedVar<'a> {
    pub block: &'a Ast,
    pub var: &'a Variable,
}

impl<'a> DetectedVar<'a> {
    fn new(block: &'a Ast, var: &'a Variable) -> DetectedVar<'a> {
        return DetectedVar { block, var };
    }
}

pub trait DetectorVars<'a> {
    fn detect_vars(&self) -> Vec<DetectedVar<'a>>;
    fn detect_unused_vars(&self) -> Vec<UnusedValues<'a>>;
    fn detect_used_in_block_var(&self, value: &str) -> bool;
}

impl<'a> DetectorVars<'a> for DetectUnused<'a> {
    fn detect_vars(&self) -> Vec<DetectedVar<'a>> {
        let mut vars = Vec::new();
        let block = self.ast;

        for token in &self.ast.body {
            match &token.type_ {
                Type::Variable(var) => {
                    vars.push(DetectedVar::new(block, var));
                }
                Type::Function(func) => match &func.body {
                    Some(body) => {
                        let func = DetectUnused::new(body);
                        let mut func_vars = func.detect_vars();
                        vars.append(&mut func_vars);
                    }
                    None => (),
                },
                Type::Block => {
                    let func = DetectUnused::new(token);
                    let mut func_vars = func.detect_vars();
                    vars.append(&mut func_vars);
                }
                Type::Logic(logic) => {
                    let logic_if = DetectUnused::new(&logic.do_);
                    let mut logic_vars_if = logic_if.detect_vars();
                    vars.append(&mut logic_vars_if);

                    if logic.else_.is_some() {
                        let else_ = &logic.else_.as_ref().unwrap();
                        let else_ = DetectUnused::new(&else_);
                        let mut else_vars = else_.detect_vars();
                        vars.append(&mut else_vars);
                    }
                }
                _ => (),
            }
        }
        vars
    }

    fn detect_unused_vars(&self) -> Vec<UnusedValues<'a>> {
        let detect_vars = self.detect_vars();
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
                        let func = DetectUnused::new(token);
                        if func.detect_used_in_block_var(&detected_var.var.name) {
                            used = true;
                            break;
                        };
                    }
                    Type::Function(_) => {
                        let func = DetectUnused::new(token);
                        if func.detect_used_in_block_var(&detected_var.var.name) {
                            used = true;
                            break;
                        };
                    }
                    Type::Return(val) => {
                        if val.type_ == ReturnTypes::Identifier {
                            if val.value ==  detected_var.var.name {
                                used = true;
                                break;
                            }
                        }
                    }
                    Type::Logic(logic) => {
                        for l in &logic.if_ {
                            match l {
                                Case::EqEq(val1, val2) => {
                                    let false = val1.uses(&detected_var.var.name) else {
                                        used = true; 
                                        break;
                                    };
                                    let false = val2.uses(&detected_var.var.name) else {
                                        used = true; 
                                        break;
                                    };

                                    let block = DetectUnused::new(&logic.do_);
                                    if block.detect_used_in_block_var(&detected_var.var.name) {
                                        used = true;
                                        break;
                                    };

                                    if logic.else_.is_some() {
                                        let else_ = &logic.else_.as_ref().unwrap();
                                        let block = DetectUnused::new(&else_);
                                        if block.detect_used_in_block_var(&detected_var.var.name) {
                                            used = true;
                                            break;
                                        };
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    Type::FunctionCall(call) => {
                        if call.args_uses(&detected_var.var.name) {
                            used = true;
                            break;
                        };
                    }
                    _ => continue,
                }
            }
            if !used {
                unused_vars.push(UnusedValues::Variable(&detected_var.var));
            }
        }
        unused_vars
    }

    fn detect_used_in_block_var(&self, value: &str) -> bool {
        for token in &self.ast.body {
            match &token.type_ {
                Type::Variable(var) => match &var.type_ {
                    TypeVar::Identifier(id) if id == value => {
                        return true;
                    }
                    _ => continue,
                },
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
                        let body = DetectUnused::new(body);
                        return body.detect_used_in_block_var(value);
                    }
                }
                Type::Return(val) => {
                        if val.type_ == ReturnTypes::Identifier {
                            if val.value ==  value {
                                return true;
                            }
                        }
                }
                Type::Block => {
                    let body = DetectUnused::new(token);
                    return body.detect_used_in_block_var(value);
                }
                _ => continue,
            }
        }
        return false;
    }
}
