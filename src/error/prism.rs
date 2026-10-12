use std::{error::Error, fmt::Display, io};

use inkwell::builder::BuilderError;

use crate::error::ToolchainError;

#[derive(Debug)]
pub enum PrismError {
    Toolchain(ToolchainError),
    Io(io::Error),
    Builder(BuilderError),
    Compilation(String),
    Parse(String),
    OutOfMemory,
}

impl Display for PrismError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toolchain(err) => {
                write!(f, "[ToolchainError] {err}")
            }

            Self::Io(err) => {
                write!(f, "[IoError] {err}")
            }

            Self::Builder(msg) => {
                write!(f, "[BuilderError] {msg}")
            }

            Self::Compilation(msg) => {
                write!(f, "[CompilationError] {msg}")
            }

            Self::Parse(msg) => {
                write!(f, "[OptimizationError] {msg}")
            }

            Self::OutOfMemory => {
                write!(f, "[OutOfMemoryError] The process has run out of memory.")
            }
        }
    }
}

impl Error for PrismError {}
