pub mod ast;
pub mod codegen;
pub mod cursor;
pub mod errors;
pub mod helpers_inkwel;
pub mod lepa_analyzer;
pub mod parser_lexer;
mod testing;
// Test the lexer with the given input
//
#[test]
fn test_find_instruction_with_name() {
    use inkwell::context::Context;
    use inkwell::AddressSpace;

    let context = Context::create();
    let module = context.create_module("ret");
    let builder = context.create_builder();

    let void_type = context.void_type();
    let i32_type = context.i32_type();
    let i32_ptr_type = i32_type.ptr_type(AddressSpace::default());

    let fn_type = void_type.fn_type(&[i32_ptr_type.into()], false);
    let fn_value = module.add_function("ret", fn_type, None);
    let entry = context.append_basic_block(fn_value, "entry");
    builder.position_at_end(entry);

    let var = builder.build_alloca(i32_type, "some_number");
    builder.build_store(var, i32_type.const_int(1 as u64, false));
    builder.build_return(None);

    let var = builder.build_alloca(i32_type, "some_number");
    builder.build_store(var, i32_type.const_int(1 as u64, false));
    builder.build_return(None);

    let block = fn_value.get_first_basic_block().unwrap();
    let some_number = block.get_instruction_with_name("some_number");

    assert!(some_number.is_some());
    assert_eq!(
        some_number.unwrap().get_name().unwrap().to_str(),
        Ok("some_number")
    )
}
