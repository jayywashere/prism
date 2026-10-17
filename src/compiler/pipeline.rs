use std::path::Path;

use inkwell::{context::Context, passes::PassBuilderOptions};

use crate::{
    compiler::{CodeGenerator, Linker, OptimizationLevel, ir},
    error::PrismError,
};

pub fn compile(
    linker: &Linker,
    ir_path: &Path,
    output_path: &Path,
    optimization: OptimizationLevel,
) -> Result<(), PrismError> {
    let context = Context::create();
    let module = ir::load_ir(&context, ir_path)?;

    let codegen = CodeGenerator::native(optimization)?;

    if let Some(passes) = optimization.pass_pipeline() {
        module
            .run_passes(
                passes,
                codegen.target_machine(),
                PassBuilderOptions::create(),
            )
            .map_err(|err| PrismError::Compilation(err.to_string()))?;
    }

    let object_path = output_path.with_extension("o");

    codegen.emit_object(&module, &object_path)?;
    linker.link(&object_path, output_path)?;

    Ok(())
}
