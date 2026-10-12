use std::{fs, path::Path};

use crate::error::PrismError;

pub fn write_ir(ir_str: &str, path: impl AsRef<Path>) -> Result<(), PrismError> {
    fs::write(path, ir_str).map_err(|err| PrismError::Io(err))?;

    Ok(())
}
