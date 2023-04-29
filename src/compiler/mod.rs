use std::process::exit;

use inkwell::{
    builder::Builder, context::Context, execution_engine::ExecutionEngine, module::Module,
    AddressSpace, values::FunctionValue,
};

static LOGGER: Logger = Logger(crate::errors::logger::LogLevels::Info);

use crate::{
    ast::{self, Ast, Func, Return, ReturnTypes, TypesArg, Variable, Type},
    errors::logger::{self, Log, Logger},
    logme,
    parser_lexer::lexer::lexer::TokenType,
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

pub trait Compile {
    ///  Takes the ast and returns the llvm ir string
    fn compile(&self) -> String;
}

impl Compile for Ast {
    fn compile(&self) -> String {
        let context = Context::create();
        let module = context.create_module("start");

        let code_gen = CodeGen {
            module,
            context: &context,
            builder: context.create_builder(),
        };

        match &self.type_ {
            crate::ast::Type::Program => {
                code_gen.compile_gen(self.clone());
            }
            _ => LOGGER.error(&"[COMPILER] Ast didn't start with program as first node."),
        };

        code_gen.module.to_string()
    }
}

trait Gen {
    fn compile_gen(&self, ast: Ast);
    fn gen_var(&self, var: &Variable);
    fn gen_func(&self, function: &Func);
    fn gen_block_func(&self, function: &Func,ast:Vec<Ast>, func:&FunctionValue);
}

impl<'ctx> Gen for CodeGen<'ctx> {
    fn compile_gen(&self, ast: Ast) {
        for node in ast.body {
            match node.type_ {
                crate::ast::Type::Variable(var) => {
                    let _ = &self.gen_var(&var);
                }
                crate::ast::Type::Function(func) => {
                    let _ = &self.gen_func(&func);
                }
                _ => (),
            };
        }
    }
    fn gen_var(&self, var: &Variable) {
        let Variable { name, type_, .. } = var;
        match type_ {
            crate::ast::TypeVar::Number(number) => {
                let num = self.context.i32_type();
                let number = num.const_int(*number as u64, false);
                let var = &self
                    .module
                    .add_global(num, Some(AddressSpace::from(1_u16)), name);
                var.set_initializer(&number);
            }
            _ => (),
        }
    }
    fn gen_func(&self, function: &Func) {
        let vars = &function.args;
        let mut args = vec![];

        for arg in vars {
            match arg.type_ {
                TypesArg::None => {
                    LOGGER.error(&format!(
                        "Found a invalid function argument, {} doesn't have a type.",
                        arg.value
                    ));
                }
                TypesArg::Number => {
                    args.push(self.context.i32_type().into());
                }
                TypesArg::String => {
                    LOGGER.error(&format!(
                        "Found a invalid function argument, {} is of type string, this is not supported yet :(.",
                        arg.value
                    ));
                }
            }
        }

        let fn_type = match function.return_type {
            ast::ReturnTypes::None => self.context.void_type().fn_type(&args, false),
            ast::ReturnTypes::Number => self.context.i32_type().fn_type(&args, false),
            _ => todo!("Not supported yet..."),
        };

        let func = &self.module.add_function(&function.name, fn_type, None);
        let basic_block = &self.context.append_basic_block(*func, "entry");
        self.builder.position_at_end(*basic_block);
        match &function.body.clone().unwrap().type_ {
            Type::Block => {
                let _ = &self.gen_block_func(function,  function.body.clone().unwrap().body, func);
            }
            token => LOGGER.error(&format!("Compiler expected function block but found: {:#?}", token))
        }
    }
    fn gen_block_func(&self, function: &Func,ast: Vec<Ast>, func:&FunctionValue) {
        for token in &ast {
            match &token.type_ {
                Type::Return(ret) => match &ret.type_ {
                    ReturnTypes::Identifier => {
                        let val = self.module.get_global(&ret.value);
                        if val.is_some() {
                            let val = val.unwrap();
                            self.builder.build_return(Some(&val));
                            break;
                        }
                        let param_ = function.get_arg_index_(&ret.value);
                        if param_.is_some() {
                            let val = func.get_nth_param(param_.unwrap());
                            if val.is_some() {
                                self.builder.build_return(Some(&val.unwrap()));
                            }
                        }
                    }
                    ReturnTypes::Number => {
                        let num: i32 = ret.value.parse().unwrap();
                        let var = self.context.i32_type();
                        let num = var.const_int(num as u64, false);
                        self.builder.build_return(Some(&num));
                    }
                    ReturnTypes::None => {
                        self.builder.build_return(None);
                    }
                    type_ => LOGGER.error(&format!(
                        "This token type is not yet supported for return bodies: {:#?}",
                        type_
                    )),
                },
                type_ => LOGGER.error(&format!(
                    "This token type is not yet supported for function bodies: {:#?}",
                    type_
                )),
            }
        }
    }
}
