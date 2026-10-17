use std::path::Path;

use inkwell::{context::Context, memory_buffer::MemoryBuffer, module::Module};

use crate::error::PrismError;

pub fn load_ir<'ctx>(context: &'ctx Context, path: &Path) -> Result<Module<'ctx>, PrismError> {
    let buffer = MemoryBuffer::create_from_file(path)
        .map_err(|err| PrismError::Compilation(err.to_string()))?;

    context
        .create_module_from_ir(buffer)
        .map_err(|err| PrismError::Compilation(err.to_string()))
}
