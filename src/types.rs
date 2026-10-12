use inkwell::{AddressSpace, context::Context as LLVMContext, types::BasicTypeEnum};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int1,     // Boolean
    Int8,     // Byte / Char
    Int16,    // 16-bit/Short Integer
    Int32,    // 32-bit/Standard Integer
    Int64,    // 64-bit/Long Integer
    Int128,   // 128-bit/Huge Integer
    Float16,  // Half-precision Decimal (16-bit float)
    Float32,  // Single-precision Decimal (32-bit float)
    Float64,  // Double-precision Decimal (64-bit float)
    Float128, // Quadruple-precision Decimal (128-bit float)
    Pointer,  // Generic Pointer Type
    Array(Box<Type>, u32),
    Void, // Nothing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerSign {
    Signed,
    Unsigned,
}

impl Type {
    pub fn to_basic_type<'a>(&self, context: &'a LLVMContext) -> Option<BasicTypeEnum<'a>> {
        match self {
            Type::Int1 => Some(BasicTypeEnum::IntType(context.bool_type())),
            Type::Int8 => Some(BasicTypeEnum::IntType(context.i8_type())),
            Type::Int16 => Some(BasicTypeEnum::IntType(context.i16_type())),
            Type::Int32 => Some(BasicTypeEnum::IntType(context.i32_type())),
            Type::Int64 => Some(BasicTypeEnum::IntType(context.i64_type())),
            Type::Int128 => Some(BasicTypeEnum::IntType(context.i128_type())),
            Type::Float16 => Some(BasicTypeEnum::FloatType(context.f16_type())),
            Type::Float32 => Some(BasicTypeEnum::FloatType(context.f32_type())),
            Type::Float64 => Some(BasicTypeEnum::FloatType(context.f64_type())),
            Type::Float128 => Some(BasicTypeEnum::FloatType(context.f128_type())),
            Type::Pointer => Some(BasicTypeEnum::PointerType(
                context.ptr_type(AddressSpace::default()),
            )),
            Type::Array(element_type, length) => {
                let llvm_element = element_type
                    .to_basic_type(context)
                    .expect("annot create an array containing Type::Void elements.");

                match llvm_element {
                    BasicTypeEnum::IntType(t) => {
                        Some(BasicTypeEnum::ArrayType(t.array_type(*length)))
                    }
                    BasicTypeEnum::FloatType(t) => {
                        Some(BasicTypeEnum::ArrayType(t.array_type(*length)))
                    }
                    BasicTypeEnum::PointerType(t) => {
                        Some(BasicTypeEnum::ArrayType(t.array_type(*length)))
                    }
                    BasicTypeEnum::ArrayType(t) => {
                        Some(BasicTypeEnum::ArrayType(t.array_type(*length)))
                    }
                    _ => unreachable!(),
                }
            }
            Type::Void => None,
        }
    }
}
