use inkwell::{context::Context, module::Module, execution_engine::ExecutionEngine, builder::Builder, AddressSpace};

use crate::ast::{Ast, Func, Variable};

pub struct Compiler {
    pub ast: Ast,
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl Compiler {
    pub fn new(ast: Ast) -> Compiler {
        Compiler { ast }
    }
}

pub trait Compile {
    fn compile(&self);
}

impl Compile for Compiler {
    fn compile(&self)  {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("main");
        let execution_engine = module.create_jit_execution_engine(inkwell::OptimizationLevel::None).unwrap();
        let code_gen =  CodeGen {
            module,
            context:&context,
            builder,
            execution_engine
        };

        match &self.ast.type_ {
            // crate::ast::Type::Function(func) => {
            // }
            crate::ast::Type::Variable(var) => {
                code_gen.gen_var(var);
            }
            _ => (),
        };

        let string = code_gen.module.to_string();
        println!("{:#?}", string);
    }
}

trait Gen {
    fn gen_var(&self, var: &Variable);
}

impl<'ctx> Gen for CodeGen<'ctx> {
    fn gen_var(&self, var: &Variable) {
        let Variable { name, type_, .. } = var;
        match type_  {
            crate::ast::TypeVar::Number(number) => {
                let num = self.context.i32_type();
                let number = num.const_int(*number as u64, false);
                let var = &self.module.add_global(num, Some(AddressSpace::from(1_u16)), name);
                var.set_initializer(&number);
            }
            _ => ()
        }
    }
}
