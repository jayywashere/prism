use inkwell::{
    context::Context as LLVMContext,
    module::{Linkage, Module as LLVMModule},
    types::{BasicMetadataTypeEnum, BasicTypeEnum},
    values::FunctionValue,
};

use crate::types::Type;

pub struct Module<'a> {
    context: &'a LLVMContext,
    module: LLVMModule<'a>,
}

impl<'a> Module<'a> {
    pub fn new(context: &'a LLVMContext, module: LLVMModule<'a>) -> Self {
        Self { context, module }
    }

    pub fn create_function(
        &self,
        name: &str,
        return_type: Type,
        param_types: &[Type],
    ) -> FunctionValue<'_> {
        let mapped_params: Vec<BasicMetadataTypeEnum> = param_types
            .iter()
            .map(|t| {
                t.to_basic_type(self.context)
                    .expect("Cannot use 'Void' as a function parameter type.")
                    .into()
            })
            .collect();

        let fn_type = match return_type {
            Type::Void => self.context.void_type().fn_type(&mapped_params, false),

            _ => {
                let basic_ret = return_type.to_basic_type(self.context).unwrap();

                match basic_ret {
                    BasicTypeEnum::IntType(t) => t.fn_type(&mapped_params, false),
                    BasicTypeEnum::FloatType(t) => t.fn_type(&mapped_params, false),

                    _ => unreachable!(),
                }
            }
        };

        self.module
            .add_function(name, fn_type, Some(Linkage::External))
    }

    pub fn get_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub fn get_name(&self) -> &str {
        self.module.get_name().to_str().unwrap()
    }
}
