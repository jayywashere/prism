use std::{error::Error, io, path::Path, process::Command};

use crate::{compiler::{OptimizationLevel, Toolchain}, error::PrismError};

pub fn compile(
    tools: &Toolchain,
    ir_path: &Path,
    output_path: &Path,
    optimization: OptimizationLevel,
) -> Result<(), Box<dyn Error>> {
    let bitcode_path = output_path.with_extension("bc");
    let optimized_bitcode_path = output_path.with_extension("optimized.bc");
    let object_path = output_path.with_extension("o");

    let status = Command::new(&tools.llvm_as)
        .arg(ir_path)
        .arg("-o")
        .arg(&bitcode_path)
        .status()?;

    if !status.success() {
        return Err(PrismError::Io(io::Error::other("llvm-as failed.")).into());
    }

    let codegen_input = if let Some(flag) = optimization.opt_flag() {
        let opt = tools
            .opt
            .as_ref()
            .ok_or_else(|| PrismError::Io(io::Error::other("LLVM optimizer 'opt' was not found.").into()))?;

        let status = Command::new(opt)
            .arg(flag)
            .arg(&bitcode_path)
            .arg("-o")
            .arg(&optimized_bitcode_path)
            .status()?;

        if !status.success() {
            return Err(PrismError::Io(io::Error::other("LLVM optimization failed.")).into());
        }

        optimized_bitcode_path.as_path()
    } else {
        bitcode_path.as_path()
    };

    let status = Command::new(&tools.llc)
        .arg(optimization.llc_flag())
        .arg("-filetype=obj")
        .arg(codegen_input)
        .arg("-o")
        .arg(&object_path)
        .status()?;

    if !status.success() {
        return Err(PrismError::Io(io::Error::other("llc failed.")).into());
    }

    let status = Command::new(&tools.linker)
        .arg(&object_path)
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !status.success() {
        return Err(PrismError::Io(io::Error::other("linker failed.")).into());
    }

    Ok(())
}
