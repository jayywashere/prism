use std::path::Path;

use inkwell::{
    module::Module,
    targets::{
        CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
    },
};

use crate::{compiler::OptimizationLevel, error::PrismError};

pub struct CodeGenerator {
    machine: TargetMachine,
}

impl CodeGenerator {
    pub fn native(optimization: OptimizationLevel) -> Result<Self, PrismError> {
        Target::initialize_all(&InitializationConfig::default());

        let triple = TargetMachine::get_default_triple();

        Self::for_triple(triple, optimization)
    }

    pub fn for_triple(
        triple: TargetTriple,
        optimization: OptimizationLevel,
    ) -> Result<Self, PrismError> {
        let target =
            Target::from_triple(&triple).map_err(|err| PrismError::Compilation(err.to_string()))?;

        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                optimization.llvm_level(),
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or_else(|| {
                PrismError::Compilation("LLVM could not create a target machine.".into())
            })?;

        Ok(Self { machine })
    }

    pub fn emit_object(&self, module: &Module<'_>, path: &Path) -> Result<(), PrismError> {
        module.set_triple(&self.machine.get_triple());

        let data_layout = self.machine.get_target_data().get_data_layout();
        module.set_data_layout(&data_layout);

        module
            .verify()
            .map_err(|err| PrismError::Compilation(err.to_string()))?;

        self.machine
            .write_to_file(module, FileType::Object, path)
            .map_err(|err| PrismError::Compilation(err.to_string()))
    }

    pub fn target_machine(&self) -> &TargetMachine {
        &self.machine
    }
}
