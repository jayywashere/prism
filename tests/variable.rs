use prism_llvm::{core::Context, types::Type};

#[test]
fn variable() {
    let context = Context::new();
    let module = context.create_module("variables");
    let builder = context.create_builder();

    let function = module.create_function("variable", Type::Int32, &[]);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    let x = builder.build_allocation(&Type::Int32, "x").unwrap();

    let ten = context.const_sint(Type::Int32, 10);
    builder.build_store(x, ten).unwrap();

    let twenty = context.const_sint(Type::Int32, 20);
    builder.build_store(x, twenty).unwrap();

    let value = builder
        .build_load(&Type::Int32, x, "x_value")
        .unwrap()
        .into_int_value();

    builder.build_return(Some(&value)).unwrap();

    let ir = module.get_ir();
    println!("{ir}");

    assert!(ir.contains("%x = alloca i32"));
    assert!(ir.contains("store i32 10"));
    assert!(ir.contains("store i32 20"));
    assert!(ir.contains("load i32"));
    assert!(ir.contains("ret i32 %x_value"));
}
