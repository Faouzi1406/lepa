pub mod compile_function_call;
pub mod compile_logic_cases;
pub mod get_args_function;
pub mod return_compiler;
pub mod std_compiler;
pub mod validation;
pub mod var_compiler;

use crate::ast::function::Func;
use std::{
    env::current_dir,
    process::{exit, Command},
};

use colored::Colorize;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module,
    values::FunctionValue, AddressSpace,
};

pub static LOGGER: Logger = Logger(crate::errors::logger::LogLevels::Info);

use crate::ast::variable::Variable;
use crate::{
    ast::ast::{self, Ast, Type, TypesArg},
    errors::logger::{Log, Logger},
};

use self::{compile_function_call::compile_function_call, compile_logic_cases::compile_logic_case};

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
}

pub trait Compile {
    ///  Takes the ast and returns the llvm ir string
    fn compile(&self) -> String;
    fn create_binary(file_name: &str);
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
    fn create_binary(file_name: &str) {
        let mut llvm = Command::new("llvm-as");
        llvm.arg(current_dir().unwrap().to_str().unwrap().to_string() + &file_name);

        let run_llvm = llvm.spawn();
        match run_llvm {
            Ok(_) => {
                let mut clang = Command::new("clang");
                clang
                    .arg(current_dir().unwrap().to_str().unwrap().to_string() + &file_name + ".bc");
                clang
                    .arg("-o")
                    .arg(current_dir().unwrap().to_str().unwrap().to_string() + &file_name);

                let run_clang = clang.output();
                match run_clang {
                    Ok(_) => {
                        LOGGER.info(&format!(
                            "Compiled to: {}",
                            current_dir().unwrap().to_str().unwrap().to_string() + &file_name
                        ));
                    }
                    Err(value) => {
                        LOGGER.error(&format!(
                            "Couldn't compile to: {}; error {:?}",
                            current_dir().unwrap().to_str().unwrap().to_string() + &file_name,
                            value
                        ));
                    }
                }
            }
            Err(value) => {
                LOGGER.error(&format!(
                    "Couldn't compile to: {}; error {:?}",
                    current_dir().unwrap().to_str().unwrap().to_string() + &file_name,
                    value
                ));
            }
        }
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
                crate::ast::ast::Type::ConstVar(var) => {
                    let _ = &self.gen_var(&var);
                }
                crate::ast::ast::Type::Variable(var) => {
                    LOGGER.display_error(&format!(
                            "Found let keyword outside of function body, consider changing this to: {} {} {} {:#?}; ignoring variable", "const".yellow().bold(), var.name.blue().bold(), "=", var.type_))
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
            crate::ast::variable::TypeVar::Number(number) => {
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
                Type::Logic(case) => {
                    let logic_case = compile_logic_case(&self, function, case, block, func);
                    if logic_case.is_err() {
                        LOGGER.error(&logic_case.err().unwrap());
                        exit(0x0100)
                    }
                }
                Type::FunctionCall(call) => {
                    let _call = compile_function_call(&self, call, func);
                }
                type_ => LOGGER.error(&format!(
                    "This token type is not yet supported for function bodies: {:#?}",
                    type_
                )),
            }
        }
    }
}
