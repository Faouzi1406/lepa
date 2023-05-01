pub mod compile_function_call;
pub mod return_compiler;
pub mod var_compiler;
pub mod std_compiler;

use crate::ast::function::Func;
use std::process::exit;

use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module,
    values::FunctionValue, AddressSpace,
};

pub static LOGGER: Logger = Logger(crate::errors::logger::LogLevels::Info);

use crate::{
    ast::ast::{self, Ast, Type, TypesArg, Variable},
    errors::logger::{Log, Logger},
};

use self::compile_function_call::compile_function_call;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
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
            crate::ast::ast::Type::Program => {
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
    fn gen_block_func(
        &self,
        function: &Func,
        ast: Vec<Ast>,
        func: &FunctionValue,
        block: &BasicBlock,
    );
}

impl<'ctx> Gen for CodeGen<'ctx> {
    fn compile_gen(&self, ast: Ast) {
        for node in ast.body {
            match node.type_ {
                crate::ast::ast::Type::Variable(var) => {
                    let _ = &self.gen_var(&var);
                }
                crate::ast::ast::Type::Function(func) => {
                    let _ = &self.gen_func(&func);
                }
                _ => (),
            };
        }
    }
    fn gen_var(&self, var: &Variable) {
        let Variable { name, type_, .. } = var;
        match type_ {
            crate::ast::ast::TypeVar::Number(number) => {
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
                // Todo: Find a way to make this work in a simple way, problem is that we need to
                // know the length of the expected string, we can either let the user declare that
                // or detect it ourselfs, the problem however is that there is no way of us to
                // detect how long a string will be if the function is never called so we have to
                // force usage of the function
                TypesArg::String => {
                    LOGGER.error(&format!(
                        "Found a invalid function argument, {} doesn't have a type.",
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
                let _ = &self.gen_block_func(
                    function,
                    function.body.clone().unwrap().body,
                    func,
                    basic_block,
                );
            }
            token => LOGGER.error(&format!(
                "Compiler expected function block but found: {:#?}",
                token
            )),
        }
    }
    fn gen_block_func(
        &self,
        function: &Func,
        ast: Vec<Ast>,
        func: &FunctionValue,
        block: &BasicBlock,
    ) {
        for token in &ast {
            match &token.type_ {
                Type::Return(ret) => {
                    let ret =
                        return_compiler::return_type_build(&self, ret.clone(), function, func);
                    if ret.is_err() {
                        LOGGER.error(&ret.err().unwrap());
                        exit(0x0100)
                    }
                }
                Type::Variable(var) => {
                    var_compiler::compile_var_func(&self, var.clone(), func);
                }
                Type::Function(func) => {
                    let _ = &self.gen_func(func);
                    let _ = &self.builder.position_at_end(*block);
                }
                Type::FunctionCall(call) => {
                    let _call = compile_function_call(&self, call);
                }
                type_ => {
                    println!("huh? {:#?}", type_);
                    LOGGER.error(&format!(
                        "This token type is not yet supported for function bodies: {:#?}",
                        type_
                    ))
                }
            }
        }
    }
}
