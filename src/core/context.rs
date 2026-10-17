use inkwell::{
    AddressSpace,
    basic_block::BasicBlock,
    context::Context as LLVMContext,
    values::{FloatValue, FunctionValue, IntValue, PointerValue},
};

use crate::{backend::Builder, core::module::Module, types::Type};

pub struct Context {
    context: LLVMContext,
}

impl Context {
    pub fn new() -> Self {
        Self {
            context: LLVMContext::create(),
        }
    }

    pub fn const_uint(&self, value_type: Type, value: u64) -> IntValue<'_> {
        let llvm_type = match value_type {
            Type::Int1 => self.context.bool_type(),
            Type::Int8 => self.context.i8_type(),
            Type::Int16 => self.context.i16_type(),
            Type::Int32 => self.context.i32_type(),
            Type::Int64 => self.context.i64_type(),
            Type::Int128 => self.context.i128_type(),
            _ => panic!("Cannot generate an integer constant using a non-integer Type variant."),
        };

        llvm_type.const_int(value, false)
    }

    pub fn const_sint(&self, value_type: Type, value: i64) -> IntValue<'_> {
        let llvm_type = match value_type {
            Type::Int1 => self.context.bool_type(),
            Type::Int8 => self.context.i8_type(),
            Type::Int16 => self.context.i16_type(),
            Type::Int32 => self.context.i32_type(),
            Type::Int64 => self.context.i64_type(),
            Type::Int128 => self.context.i128_type(),
            _ => panic!("Cannot generate an integer constant using a non-integer Type variant."),
        };

        llvm_type.const_int(value as u64, false)
    }

    pub fn const_float(&self, value_type: Type, value: f64) -> FloatValue<'_> {
        let llvm_type = match value_type {
            Type::Float16 => self.context.f16_type(),
            Type::Float32 => self.context.f32_type(),
            Type::Float64 => self.context.f64_type(),
            Type::Float128 => self.context.f128_type(),
            _ => panic!(
                "Cannot generate a floating-point constant using a non-decimal Type variant."
            ),
        };

        llvm_type.const_float(value)
    }

    pub fn const_bool(&self, value: bool) -> IntValue<'_> {
        self.context.bool_type().const_int(value as u64, false)
    }

    pub fn const_nullptr(&self) -> PointerValue<'_> {
        self.context.ptr_type(AddressSpace::default()).const_null()
    }

    pub fn append_basic_block<'ctx>(
        &'ctx self,
        function: FunctionValue<'ctx>,
        name: &str,
    ) -> BasicBlock<'ctx> {
        self.context.append_basic_block(function, name)
    }

    pub fn create_module(&self, name: &str) -> Module<'_> {
        let module = self.context.create_module(name);
        Module::new(&self.context, module)
    }

    pub fn create_module_from_ir() {}

    pub fn create_builder(&self) -> Builder<'_> {
        Builder::new(&self.context)
    }

    /// Returns a reference to an `LLVMContext`/`inkwell::context::Context`
    ///
    /// ### Warning
    /// Prefer `Prism`'s API whenever an equivalent method is available.
    /// Use this method when LLVM functionality is needed that Prism
    /// doesn't currently expose.
    ///
    /// Direct access to the LLVM context couples your code to Inkwell
    /// and LLVM's API, which may make future `Prism` API changes harder
    /// to accommodate.
    pub fn get_context(&self) -> &LLVMContext {
        &self.context
    }
}
