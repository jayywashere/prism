use inkwell::{
    basic_block::BasicBlock,
    builder::Builder as LLVMBuilder,
    context::Context as LLVMContext,
    values::{
        BasicMetadataValueEnum, BasicValue, BasicValueEnum, CallSiteValue, FloatValue,
        FunctionValue, InstructionValue, IntValue, PointerValue,
    },
};

use crate::{
    error::PrismError,
    types::{IntegerSign, Type},
};

pub struct Builder<'ctx> {
    builder: LLVMBuilder<'ctx>,
    context: &'ctx LLVMContext,
}

impl<'ctx> Builder<'ctx> {
    pub fn new(context: &'ctx LLVMContext) -> Self {
        Self {
            builder: context.create_builder(),
            context,
        }
    }

    // INTEGER MATH ->

    pub fn build_int_add(
        &self,
        a: IntValue<'ctx>,
        b: IntValue<'ctx>,
        name: &str,
    ) -> Result<IntValue<'_>, PrismError> {
        self.builder
            .build_int_add(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_int_sub(
        &self,
        a: IntValue<'ctx>,
        b: IntValue<'ctx>,
        name: &str,
    ) -> Result<IntValue<'_>, PrismError> {
        self.builder
            .build_int_sub(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_int_mul(
        &self,
        a: IntValue<'ctx>,
        b: IntValue<'ctx>,
        name: &str,
    ) -> Result<IntValue<'_>, PrismError> {
        self.builder
            .build_int_mul(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_int_div(
        &self,
        a: IntValue<'ctx>,
        b: IntValue<'ctx>,
        sign: IntegerSign,
        name: &str,
    ) -> Result<IntValue<'_>, PrismError> {
        match sign {
            IntegerSign::Signed => self.builder.build_int_signed_div(a, b, name),
            IntegerSign::Unsigned => self.builder.build_int_unsigned_div(a, b, name),
        }
        .map_err(PrismError::Builder)
    }

    pub fn build_int_rem(
        &self,
        a: IntValue<'ctx>,
        b: IntValue<'ctx>,
        sign: IntegerSign,
        name: &str,
    ) -> Result<IntValue<'_>, PrismError> {
        match sign {
            IntegerSign::Signed => self.builder.build_int_signed_rem(a, b, name),
            IntegerSign::Unsigned => self.builder.build_int_unsigned_rem(a, b, name),
        }
        .map_err(PrismError::Builder)
    }

    // INTEGER MATH <-

    // DECIMAL MATH ->

    pub fn build_float_add(
        &self,
        a: FloatValue<'ctx>,
        b: FloatValue<'ctx>,
        name: &str,
    ) -> Result<FloatValue<'_>, PrismError> {
        self.builder
            .build_float_add(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_float_sub(
        &self,
        a: FloatValue<'ctx>,
        b: FloatValue<'ctx>,
        name: &str,
    ) -> Result<FloatValue<'_>, PrismError> {
        self.builder
            .build_float_sub(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_float_mul(
        &self,
        a: FloatValue<'ctx>,
        b: FloatValue<'ctx>,
        name: &str,
    ) -> Result<FloatValue<'_>, PrismError> {
        self.builder
            .build_float_mul(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_float_div(
        &self,
        a: FloatValue<'ctx>,
        b: FloatValue<'ctx>,
        name: &str,
    ) -> Result<FloatValue<'_>, PrismError> {
        self.builder
            .build_float_div(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn build_float_rem(
        &self,
        a: FloatValue<'ctx>,
        b: FloatValue<'ctx>,
        name: &str,
    ) -> Result<FloatValue<'_>, PrismError> {
        self.builder
            .build_float_rem(a, b, name)
            .map_err(|err| PrismError::Builder(err))
    }

    // DECIMAL MATH <-

    pub fn build_call(
        &self,
        function: FunctionValue<'ctx>,
        args: &[BasicMetadataValueEnum<'ctx>],
        name: &str,
    ) -> Result<CallSiteValue<'_>, PrismError> {
        self.builder
            .build_call(function, args, name)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn get_call_value(&self, call: CallSiteValue<'ctx>) -> Option<BasicValueEnum<'_>> {
        call.try_as_basic_value().basic()
    }

    pub fn build_allocation(
        &self,
        value_type: &Type,
        name: &str,
    ) -> Result<PointerValue<'_>, PrismError> {
        let llvm_type = value_type.to_basic_type(self.context).ok_or_else(|| {
            PrismError::Compilation("Cannot allocate stack memory for Type::Void.".to_owned())
        })?;

        self.builder
            .build_alloca(llvm_type, name)
            .map_err(PrismError::Builder)
    }

    pub fn build_store<V: BasicValue<'ctx>>(
        &self,
        pointer: PointerValue<'ctx>,
        value: V,
    ) -> Result<InstructionValue<'_>, PrismError> {
        self.builder
            .build_store(pointer, value)
            .map_err(PrismError::Builder)
    }

    pub fn build_load(
        &self,
        value_type: &Type,
        pointer: PointerValue<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'_>, PrismError> {
        let llvm_type = value_type.to_basic_type(self.context).ok_or_else(|| {
            PrismError::Compilation("Cannot load a value of Type::Void.".to_owned())
        })?;

        self.builder
            .build_load(llvm_type, pointer, name)
            .map_err(PrismError::Builder)
    }

    pub fn build_return(
        &self,
        value: Option<&dyn BasicValue<'ctx>>,
    ) -> Result<InstructionValue<'_>, PrismError> {
        self.builder
            .build_return(value)
            .map_err(|err| PrismError::Builder(err))
    }

    pub fn position_at_end(&self, block: BasicBlock) {
        self.builder.position_at_end(block);
    }
}
