use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ToolchainError {
    MissingTool(String),
}

impl Display for ToolchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTool(tool) => {
                write!(f, "Missing tool: '{tool}'.")
            }
        }
    }
}

impl Error for ToolchainError {}
