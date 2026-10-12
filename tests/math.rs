use inkwell::values::BasicMetadataValueEnum;
use prism::{
    compiler::Toolchain,
    core::Context,
    types::{IntegerSign, Type},
};

#[test]
fn math() {
    let context = Context::new();
    let module = context.create_module("test");
    let builder = context.create_builder();

    let toolchain = Toolchain::discover();
    assert!(
        toolchain.is_ok(),
        "Failed to locate local compiler binaries. Are they added to PATH?"
    );

    // INTEGER

    let int_calc_fn = module.create_function("int_calc", Type::Int32, &[Type::Int32, Type::Int32]);
    let int_entry = context.append_basic_block(int_calc_fn, "entry");
    builder.position_at_end(int_entry);

    let int_a = int_calc_fn.get_nth_param(0).unwrap().into_int_value();
    let int_b = int_calc_fn.get_nth_param(1).unwrap().into_int_value();

    let sum = builder.build_int_add(int_a, int_b, "tmp_sum").unwrap();
    let product = builder.build_int_mul(sum, int_b, "tmp_prod").unwrap();
    let final_int = builder
        .build_int_div(product, int_a, IntegerSign::Signed, "int_res")
        .unwrap();

    builder.build_return(Some(&final_int)).unwrap();

    // DECIMAL

    let float_calc_fn =
        module.create_function("float_calc", Type::Float32, &[Type::Float32, Type::Float32]);
    let float_entry = context.append_basic_block(float_calc_fn, "entry");
    builder.position_at_end(float_entry);

    let float_a = float_calc_fn.get_nth_param(0).unwrap().into_float_value();
    let float_b = float_calc_fn.get_nth_param(1).unwrap().into_float_value();

    let float_prod = builder
        .build_float_mul(float_a, float_b, "tmp_fprod")
        .unwrap();
    let final_float = builder
        .build_float_add(float_prod, float_a, "float_res")
        .unwrap();

    builder.build_return(Some(&final_float)).unwrap();

    // MAIN

    let main_fn = module.create_function("main", Type::Int32, &[]);
    let main_entry = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(main_entry);

    let const_a = context.const_sint(Type::Int32, 10);
    let const_b = context.const_sint(Type::Int32, 2);
    let int_args: [BasicMetadataValueEnum; 2] = [const_a.into(), const_b.into()];

    let calc_call = builder
        .build_call(int_calc_fn, &int_args, "call_int")
        .unwrap();
    let calc_res = builder.get_call_value(calc_call).unwrap().into_int_value();

    builder.build_return(Some(&calc_res)).unwrap();

    // ASSERTIONS

    let ir = module.get_ir();
    println!("\n=== COMPILER VERIFICATION IN TREE ===\n{ir}");

    // Assert integer blocks compiled correctly
    assert!(ir.contains("add i32 %0, %1"));
    assert!(ir.contains("sdiv i32 %tmp_prod, %0"));

    // Assert decimal blocks compiled correctly
    assert!(ir.contains("fmul float %0, %1"));
    assert!(ir.contains("fadd float %tmp_fprod, %0"));

    // Assert calling block compiled correctly
    assert!(ir.contains("call i32 @int_calc(i32 10, i32 2)"));
}
